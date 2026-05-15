use crate::types::*;

pub struct M68000 {
    // Registers: D0-D7 (0-7), A0-A6 (8-14), USP (15), SSP (16)
    pub da: [u32; 17],
    pub ipc: u32,
    pub pc: u32,
    pub au: u32,
    pub at: u32,
    pub aob: u32,
    pub dt: u32,
    pub int_vector: u32,
    pub sp: u32,       // index 15 or 16
    pub t: i32,        // temp counter (mul/div loops)
    pub movems: u32,

    pub isr: u16,
    pub sr: u16,
    pub new_sr: u16,
    pub dbin: u16,
    pub dbout: u16,
    pub edb: u16,
    pub irc: u16,
    pub ir: u16,
    pub ird: u16,
    pub ftu: u16,
    pub aluo: u16,
    pub alue: u16,
    pub alub: u16,
    pub movemr: u16,
    pub irdi: u16,
    pub base_ssw: u16,
    pub ssw: u16,

    pub dcr: u8,

    // IRQ state
    pub virq_state: u32,
    pub nmi_pending: bool,
    pub int_level: u32,
    pub int_next_state: u32,

    // State machine
    pub inst_state: u16,
    pub next_state: u32,

    // Decode table (64K entries)
    pub decode_table: Vec<u16>,

    // Cycle accumulator for current instruction
    pub icount: i32,
}

impl M68000 {
    pub fn new() -> Self {
        let mut cpu = M68000 {
            da: [0; 17],
            ipc: 0, pc: 0, au: 0, at: 0, aob: 0, dt: 0, int_vector: 0,
            sp: 16, t: 0, movems: 0,
            isr: 0, sr: 0, new_sr: 0, dbin: 0, dbout: 0, edb: 0,
            irc: 0, ir: 0, ird: 0, ftu: 0, aluo: 0, alue: 0, alub: 0,
            movemr: 0, irdi: 0, base_ssw: 0, ssw: 0,
            dcr: 0,
            virq_state: 0, nmi_pending: false, int_level: 0, int_next_state: 0,
            inst_state: S_RESET, next_state: 0,
            decode_table: Vec::new(),
            icount: 0,
        };
        cpu.init_decode_table();
        cpu
    }

    // --- Helpers (exact match of C++ inlines) ---

    #[inline(always)]
    pub fn map_sp(&self, r: usize) -> usize {
        if r == 15 { self.sp as usize } else { r }
    }

    #[inline(always)]
    pub fn merge_16_32(h: u16, l: u16) -> u32 {
        ((h as u32) << 16) | (l as u32)
    }

    #[inline(always)]
    pub fn high16(v: u32) -> u16 {
        (v >> 16) as u16
    }

    #[inline(always)]
    pub fn ext32(v: u16) -> u32 {
        (v as i16) as i32 as u32
    }

    #[inline(always)]
    pub fn ext32h(v: u16) -> u16 {
        if v & 0x8000 != 0 { 0xffff } else { 0x0000 }
    }

    #[inline(always)]
    pub fn set_16h(r: &mut u32, v: u16) {
        *r = (*r & 0x0000ffff) | ((v as u32) << 16);
    }

    #[inline(always)]
    pub fn set_16l_32(r: &mut u32, v: u16) {
        *r = (*r & 0xffff0000) | (v as u32);
    }

    #[inline(always)]
    pub fn set_16l_16(r: &mut u16, v: u16) {
        *r = v;
    }

    #[inline(always)]
    pub fn set_8_32(r: &mut u32, v: u8) {
        *r = (*r & 0xffffff00) | (v as u32);
    }

    #[inline(always)]
    pub fn set_8_16(r: &mut u16, v: u8) {
        *r = (*r & 0xff00) | (v as u16);
    }

    #[inline(always)]
    pub fn set_8h(r: &mut u16, v: u8) {
        *r = (*r & 0x00ff) | ((v as u16) << 8);
    }

    #[inline(always)]
    pub fn set_8xl(r: &mut u16, v: u16) {
        *r = (v & 0x00ff) | (v << 8);
    }

    #[inline(always)]
    pub fn set_8xh(r: &mut u16, v: u16) {
        *r = (v & 0xff00) | (v >> 8);
    }

    /// set_ftu_const — matches C++ exactly
    pub fn set_ftu_const(&mut self) {
        self.ftu = match self.ird >> 12 {
            0x4 => 0x80,
            0x5 | 0xe => {
                let v = (self.ird >> 9) & 7;
                if v == 0 { 8 } else { v }
            }
            0x6 | 0x7 => (self.ird as i8) as u16,
            0x8 | 0xc => 0x0f,
            _ => 0,
        };
    }

    /// step_movem — matches C++ exactly
    #[inline]
    pub fn step_movem(&mut self) {
        let r = Self::countr_zero_16(self.movemr);
        let r = if r > 15 { 0 } else { r };
        self.movems = self.map_sp(r) as u32;
        self.movemr &= !(1 << r);
    }

    /// step_movem_predec — matches C++ exactly
    #[inline]
    pub fn step_movem_predec(&mut self) {
        let r = Self::countr_zero_16(self.movemr);
        let r = if r > 15 { 0 } else { r };
        self.movems = self.map_sp(r ^ 0xf) as u32;
        self.movemr &= !(1 << r);
    }

    #[inline(always)]
    fn countr_zero_16(v: u16) -> usize {
        if v == 0 { 16 } else { v.trailing_zeros() as usize }
    }

    /// update_user_super — switch sp index based on SR S bit
    pub fn update_user_super(&mut self) {
        if self.sr & SR_S != 0 {
            self.sp = 16;
        } else {
            self.sp = 15;
        }
    }

    /// update_interrupt — recompute int_next_state from current levels
    pub fn update_interrupt(&mut self) {
        if self.nmi_pending {
            self.int_next_state = (7 << 24) | S_INTERRUPT as u32;
        } else if self.int_level > ((self.sr >> 8) & 7) as u32 {
            self.int_next_state = (self.int_level << 24) | S_INTERRUPT as u32;
        } else {
            self.int_next_state = 0;
        }
    }

    /// set_irq — external interrupt level change (0-7)
    pub fn set_irq(&mut self, level: u8) {
        let level = level.min(7) as u32;
        let old_level = self.int_level;

        // Simple priority encoder (interrupt mixer mode)
        self.int_level = level;

        // NMI: transition from <7 to 7
        if old_level != 7 && self.int_level == 7 {
            self.nmi_pending = true;
        }

        self.update_interrupt();
    }

    /// Function code derived from base_ssw + SR supervisor bit
    #[inline(always)]
    pub fn fc(&self) -> u8 {
        ((self.base_ssw | if self.sr & SR_S != 0 { SSW_S } else { 0 }) & 7) as u8
    }

    /// Start interrupt vector lookup — called at beginning of interrupt ack
    pub fn start_interrupt_vector_lookup(&mut self) {
        let level = self.next_state >> 24;
        // Clear NMI if level 7
        if level == 7 {
            self.nmi_pending = false;
            self.update_interrupt();
        }
    }

    /// End interrupt vector lookup — called after reading vector from bus
    pub fn end_interrupt_vector_lookup(&mut self) {
        self.int_vector = ((self.edb & 0xff) as u32) << 2;
        self.int_next_state = 0;
    }
}
