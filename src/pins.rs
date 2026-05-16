//! Pin-level 68000 bus interface.
//!
//! Provides `clock()` method that advances the CPU by one CLK period,
//! modeling all real 68000 bus signals including E clock, VPA sync,
//! and bus arbitration. Uses partial handlers internally.

use crate::cpu::M68000;
use crate::types::*;

/// CPU output pins — updated after each clock().
#[derive(Clone, Debug, Default)]
pub struct Pins68k {
    pub addr: u32,       // A1-A23
    pub data_out: u16,   // D0-D15 (write data)
    pub fc: u8,          // FC0-FC2
    pub as_n: bool,      // Address strobe (active low)
    pub uds_n: bool,     // Upper data strobe (active low)
    pub lds_n: bool,     // Lower data strobe (active low)
    pub rw: bool,        // true=read, false=write
    pub bg_n: bool,      // Bus grant (active low)
    pub e: bool,         // E clock output
    pub vma_n: bool,     // Valid memory address (active low)
}

/// CPU input pins — provided by the system each clock().
#[derive(Clone, Debug)]
pub struct Inputs68k {
    pub data_in: u16,    // D0-D15 (read data)
    pub dtack_n: bool,   // Data transfer acknowledge (active low)
    pub berr_n: bool,    // Bus error (active low)
    pub vpa_n: bool,     // Valid peripheral address (active low)
    pub ipl0_n: bool,    // IPL bit 0 (active low)
    pub ipl1_n: bool,    // IPL bit 1 (active low)
    pub ipl2_n: bool,    // IPL bit 2 (active low)
    pub br_n: bool,      // Bus request (active low)
    pub bgack_n: bool,   // Bus grant acknowledge (active low)
    pub halt_n: bool,    // Halt (active low)
    pub reset_n: bool,   // Reset (active low)
}

impl Default for Inputs68k {
    fn default() -> Self {
        Self {
            data_in: 0,
            dtack_n: true,
            berr_n: true,
            vpa_n: true,
            ipl0_n: true,
            ipl1_n: true,
            ipl2_n: true,
            br_n: true,
            bgack_n: true,
            halt_n: true,
            reset_n: true,
        }
    }
}

/// Bus cycle phase within the S0-S7 state machine.
#[derive(Clone, Copy, Debug, PartialEq)]
enum BusPhase {
    Idle,
    S0,
    S2,
    S4,
    S6,
}

/// Pin-level state.
pub struct PinState {
    pub pins: Pins68k,
    pub e_clock: u8,
    pub bus_phase: BusPhase,
    pub bus_granted: bool,
    pub vpa_pending: bool,
    pub vma_active: bool,
    pub pending_byte: bool,
    pub internal_clocks: u32,
    bus_request: bool,  // handler requested a bus cycle
}

impl Default for PinState {
    fn default() -> Self {
        Self {
            pins: Pins68k {
                as_n: true, uds_n: true, lds_n: true,
                rw: true, bg_n: true, vma_n: true, e: true,
                ..Default::default()
            },
            e_clock: 0,
            bus_phase: BusPhase::Idle,
            bus_granted: false,
            vpa_pending: false,
            vma_active: false,
            pending_byte: false,
            internal_clocks: 0,
            bus_request: false,
        }
    }
}

/// Dummy bus — never called, required by dispatch_partial signature.
struct NullBus;
impl Bus for NullBus {
    fn read_program(&mut self, _: u32) -> u16 { 0 }
    fn read_data(&mut self, _: u32) -> u16 { 0 }
    fn write_data(&mut self, _: u32, _: u16) {}
    fn write_data_byte(&mut self, _: u32, _: u8) {}
    fn read_cpu(&mut self, _: u32) -> u16 { 0 }
}

impl M68000 {
    /// Advance CPU by one CLK period. Call at system clock rate.
    pub fn clock(&mut self, inputs: &Inputs68k) {
        // E clock — free-running, always advances
        self.pin_state.e_clock = (self.pin_state.e_clock + 1) % 10;
        self.pin_state.pins.e = self.pin_state.e_clock < 6;

        // Reset
        if !inputs.reset_n {
            self.pin_state.bus_phase = BusPhase::Idle;
            self.pin_state.pins.as_n = true;
            self.pin_state.pins.uds_n = true;
            self.pin_state.pins.lds_n = true;
            self.pin_state.bus_request = false;
            self.pin_state.internal_clocks = 0;
            self.inst_state = S_RESET;
            self.inst_substate = 0;
            return;
        }

        // Halt
        if !inputs.halt_n && self.pin_state.bus_phase == BusPhase::Idle {
            return;
        }

        // Bus arbitration
        if !inputs.br_n && self.pin_state.bus_phase == BusPhase::Idle
            && !self.pin_state.bus_request
        {
            self.pin_state.pins.bg_n = false;
        }
        if !inputs.bgack_n {
            self.pin_state.bus_granted = true;
            self.pin_state.pins.bg_n = true;
        }
        if inputs.bgack_n && self.pin_state.bus_granted {
            self.pin_state.bus_granted = false;
        }
        if self.pin_state.bus_granted {
            return;
        }

        // IPL sampling
        let ipl = ((!inputs.ipl2_n as u8) << 2)
                | ((!inputs.ipl1_n as u8) << 1)
                | (!inputs.ipl0_n as u8);
        if ipl != self.int_level as u8 {
            self.set_irq(ipl);
        }

        // Bus state machine
        match self.pin_state.bus_phase {
            BusPhase::Idle => {
                if self.pin_state.bus_request {
                    // Start bus cycle
                    self.pin_state.bus_phase = BusPhase::S0;
                    self.pin_state.pins.addr = self.aob & 0x00FF_FFFE;
                    self.pin_state.pins.fc = self.fc();
                    self.pin_state.pins.rw = self.base_ssw & SSW_R != 0;
                } else if self.pin_state.internal_clocks > 0 {
                    self.pin_state.internal_clocks -= 1;
                    if self.pin_state.internal_clocks == 0 {
                        self.run_core_step();
                    }
                } else {
                    self.run_core_step();
                }
            }

            BusPhase::S0 => {
                self.pin_state.bus_phase = BusPhase::S2;
                self.pin_state.pins.as_n = false;
                // Read cycle: assert data strobes now (AS and UDS/LDS fall together)
                if self.pin_state.pins.rw {
                    if self.pin_state.pending_byte {
                        if self.aob & 1 == 0 {
                            self.pin_state.pins.uds_n = false;
                            self.pin_state.pins.lds_n = true;
                        } else {
                            self.pin_state.pins.uds_n = true;
                            self.pin_state.pins.lds_n = false;
                        }
                    } else {
                        self.pin_state.pins.uds_n = false;
                        self.pin_state.pins.lds_n = false;
                    }
                } else {
                    // Write cycle: drive data, but UDS/LDS stay HIGH until S4
                    self.pin_state.pins.data_out = self.dbout;
                }
            }

            BusPhase::S2 => {
                self.pin_state.bus_phase = BusPhase::S4;
                // Write cycle: assert data strobes now (delayed to S4)
                if !self.pin_state.pins.rw {
                    if self.pin_state.pending_byte {
                        if self.aob & 1 == 0 {
                            self.pin_state.pins.uds_n = false;
                            self.pin_state.pins.lds_n = true;
                        } else {
                            self.pin_state.pins.uds_n = true;
                            self.pin_state.pins.lds_n = false;
                        }
                    } else {
                        self.pin_state.pins.uds_n = false;
                        self.pin_state.pins.lds_n = false;
                    }
                }
            }

            BusPhase::S4 => {
                // VPA handling
                if !inputs.vpa_n && !self.pin_state.vpa_pending {
                    self.pin_state.vpa_pending = true;
                }

                if self.pin_state.vpa_pending {
                    if self.pin_state.e_clock == 6 {
                        self.pin_state.pins.vma_n = false;
                        self.pin_state.vma_active = true;
                    }
                    if self.pin_state.vma_active && self.pin_state.e_clock == 0 {
                        self.pin_state.vpa_pending = false;
                        self.pin_state.vma_active = false;
                        self.pin_state.pins.vma_n = true;
                        self.pin_state.bus_phase = BusPhase::S6;
                    }
                } else if !inputs.dtack_n {
                    self.pin_state.bus_phase = BusPhase::S6;
                } else if !inputs.berr_n {
                    self.pin_state.bus_phase = BusPhase::S6;
                    self.inst_state = S_BUS_ERROR;
                }
                // else: wait state
            }

            BusPhase::S6 => {
                // Latch read data
                if self.pin_state.pins.rw {
                    self.edb = inputs.data_in;
                }
                // Negate strobes
                self.pin_state.pins.as_n = true;
                self.pin_state.pins.uds_n = true;
                self.pin_state.pins.lds_n = true;
                // Bus cycle complete
                self.pin_state.bus_request = false;
                self.pin_state.bus_phase = BusPhase::Idle;
                self.icount -= 4;
                // Resume execution
                self.run_core_step();
            }
        }
    }

    /// Run the partial handler. Computes internal clocks and detects bus requests.
    fn run_core_step(&mut self) {
        let icount_before = self.icount;

        // Set up instruction context at instruction boundary
        if self.inst_substate == 0 {
            self.ipc = self.pc.wrapping_sub(2);
            self.irdi = self.ird;
        }

        self.dispatch_partial(&mut NullBus);

        // Compute internal clocks consumed (icount decrements by 2 per internal step)
        let consumed = icount_before - self.icount;
        if consumed > 0 {
            // Internal clocks burn before the bus request (or before instruction end)
            // Each unit of icount = 1 clock period
            self.pin_state.internal_clocks = consumed as u32;
        }

        // Check if handler requested a bus cycle (inst_substate != 0 means suspended at bus_end)
        if self.inst_substate != 0 {
            self.pin_state.bus_request = true;
        } else {
            self.pin_state.bus_request = false;
        }
    }

    /// Get current output pins (no state change).
    pub fn pins(&self) -> &Pins68k {
        &self.pin_state.pins
    }
}
