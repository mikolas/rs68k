//! Pin-level 68000 bus interface.
//!
//! Provides `clock()` method that advances the CPU by one CLK period,
//! modeling all real 68000 bus signals including E clock, VPA sync,
//! and bus arbitration.

use crate::cpu::M68000;
use crate::types::*;

/// CPU output pins
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

/// CPU input pins
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
            dtack_n: true,   // not acknowledged
            berr_n: true,    // no error
            vpa_n: true,     // no peripheral
            ipl0_n: true,    // no interrupt (level 0)
            ipl1_n: true,
            ipl2_n: true,
            br_n: true,      // no bus request
            bgack_n: true,   // no bus grant ack
            halt_n: true,    // not halted
            reset_n: true,   // not reset
        }
    }
}

/// Bus cycle phase within the S0-S7 state machine
#[derive(Clone, Copy, Debug, PartialEq)]
enum BusPhase {
    /// Idle — no bus cycle active, executing internal operations
    Idle,
    /// S0-S1: drive address and FC
    S0,
    /// S2-S3: assert AS, UDS/LDS
    S2,
    /// S4-S5: wait for DTACK (may repeat for wait states)
    S4,
    /// S6-S7: latch data, negate strobes
    S6,
}

/// Pin-level state (added to M68000)
pub struct PinState {
    pub pins: Pins68k,
    pub e_clock: u8,         // 0-9 counter for E generation
    pub bus_phase: BusPhase,
    pub bus_granted: bool,   // CPU has released bus to DMA
    pub vpa_pending: bool,   // VPA was asserted, waiting for E sync
    pub vma_active: bool,    // VMA asserted during E-sync cycle

    // Pending bus cycle from execution core
    pub pending_read: bool,
    pub pending_write: bool,
    pub pending_byte: bool,  // byte access (only one strobe)
    pub pending_addr: u32,
    pub pending_data: u16,   // write data
    pub pending_fc: u8,

    // Internal clock counter for tracking 2-clock internal cycles
    pub internal_clocks: u32,
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
            pending_read: false,
            pending_write: false,
            pending_byte: false,
            pending_addr: 0,
            pending_data: 0,
            pending_fc: 0,
            internal_clocks: 0,
        }
    }
}

impl M68000 {
    /// Advance CPU by one CLK period. Updates all output pins.
    pub fn clock(&mut self, inputs: &Inputs68k) {
        // Advance E clock (free-running, always ticks)
        self.pin_state.e_clock = (self.pin_state.e_clock + 1) % 10;
        self.pin_state.pins.e = self.pin_state.e_clock < 6;

        // Handle reset
        if !inputs.reset_n {
            self.pin_state.bus_phase = BusPhase::Idle;
            self.pin_state.pins.as_n = true;
            self.pin_state.pins.uds_n = true;
            self.pin_state.pins.lds_n = true;
            self.inst_state = S_RESET;
            return;
        }

        // Handle halt
        if !inputs.halt_n && self.pin_state.bus_phase == BusPhase::Idle {
            return; // frozen
        }

        // Bus arbitration
        if !inputs.br_n && self.pin_state.bus_phase == BusPhase::Idle {
            self.pin_state.pins.bg_n = false; // grant bus
        }
        if !inputs.bgack_n {
            self.pin_state.bus_granted = true;
            self.pin_state.pins.bg_n = true; // negate grant
        }
        if inputs.bgack_n && self.pin_state.bus_granted {
            self.pin_state.bus_granted = false; // DMA released bus
        }
        if self.pin_state.bus_granted {
            return; // CPU tri-stated, DMA owns bus
        }

        // Update interrupt level from IPL pins
        let ipl = ((!inputs.ipl2_n as u8) << 2)
                | ((!inputs.ipl1_n as u8) << 1)
                | (!inputs.ipl0_n as u8);
        if ipl != self.int_level as u8 {
            self.set_irq(ipl);
        }

        // Bus state machine
        match self.pin_state.bus_phase {
            BusPhase::Idle => {
                // Check if execution core needs a bus cycle
                if self.pin_state.pending_read || self.pin_state.pending_write {
                    self.pin_state.bus_phase = BusPhase::S0;
                    // Drive address and FC
                    self.pin_state.pins.addr = self.pin_state.pending_addr & 0x00FFFFFE;
                    self.pin_state.pins.fc = self.pin_state.pending_fc;
                    self.pin_state.pins.rw = self.pin_state.pending_read;
                } else if self.pin_state.internal_clocks > 0 {
                    // Internal cycle (2 clocks per step)
                    self.pin_state.internal_clocks -= 1;
                    if self.pin_state.internal_clocks == 0 {
                        // Internal cycle complete, run more of the instruction
                        self.run_core_step();
                    }
                } else {
                    // Need more work from execution core
                    self.run_core_step();
                }
            }

            BusPhase::S0 => {
                // S0→S2: assert strobes
                self.pin_state.bus_phase = BusPhase::S2;
                self.pin_state.pins.as_n = false;
                if self.pin_state.pending_byte {
                    // Byte access: only one strobe
                    if self.pin_state.pending_addr & 1 == 0 {
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
                // Drive write data
                if !self.pin_state.pins.rw {
                    self.pin_state.pins.data_out = self.pin_state.pending_data;
                }
            }

            BusPhase::S2 => {
                // S2→S4: wait for DTACK
                self.pin_state.bus_phase = BusPhase::S4;
            }

            BusPhase::S4 => {
                // Check VPA (peripheral access — need E-clock sync)
                if !inputs.vpa_n && !self.pin_state.vpa_pending {
                    self.pin_state.vpa_pending = true;
                }

                if self.pin_state.vpa_pending {
                    // Wait for E clock falling edge (transition from high to low)
                    if self.pin_state.e_clock == 6 {
                        // E just went low — assert VMA
                        self.pin_state.pins.vma_n = false;
                        self.pin_state.vma_active = true;
                    }
                    if self.pin_state.vma_active && self.pin_state.e_clock == 0 {
                        // E rose again — cycle complete
                        self.pin_state.vpa_pending = false;
                        self.pin_state.vma_active = false;
                        self.pin_state.pins.vma_n = true;
                        self.pin_state.bus_phase = BusPhase::S6;
                    }
                    // else: keep waiting (insert wait states)
                } else if !inputs.dtack_n {
                    // Normal DTACK — proceed to S6
                    self.pin_state.bus_phase = BusPhase::S6;
                } else if !inputs.berr_n {
                    // Bus error
                    self.pin_state.bus_phase = BusPhase::S6;
                    self.inst_state = S_BUS_ERROR;
                }
                // else: wait state (stay in S4)
            }

            BusPhase::S6 => {
                // Latch read data, negate strobes, complete cycle
                if self.pin_state.pending_read {
                    // Latch data from bus
                    self.edb = inputs.data_in;
                }
                // Negate all strobes
                self.pin_state.pins.as_n = true;
                self.pin_state.pins.uds_n = true;
                self.pin_state.pins.lds_n = true;
                // Clear pending
                self.pin_state.pending_read = false;
                self.pin_state.pending_write = false;
                self.pin_state.bus_phase = BusPhase::Idle;
                // Continue execution
                self.run_core_step();
            }
        }
    }

    /// Run one step of the execution core, capturing any bus requests.
    fn run_core_step(&mut self) {
        // The execution core uses the simple Bus trait internally.
        // We provide a capturing adapter that records bus requests
        // instead of executing them immediately.
        let mut capture = BusCapture::new();
        
        // Run one micro-step of the current instruction
        // This is a simplified model — in practice we'd need to
        // suspend/resume the handler at each bus access point.
        // For now, this is a placeholder for the full implementation.
        //
        // TODO: Implement coroutine-style execution that yields at bus accesses
        // For now, the pin-level interface is structural — the full implementation
        // requires splitting handlers into suspendable steps.
        let _ = capture;
    }

    /// Get current output pins
    pub fn pins(&self) -> &Pins68k {
        &self.pin_state.pins
    }
}

/// Placeholder for bus capture adapter
struct BusCapture;
impl BusCapture {
    fn new() -> Self { Self }
}
