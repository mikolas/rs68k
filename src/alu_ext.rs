use crate::cpu::M68000;
use crate::types::*;

/// Extended ALU operations: ext, not, bcd, shifts, rotates, SR updates
impl M68000 {
    #[inline(always)]
    pub fn alu_ext(&mut self, a: u16) {
        let r = (a as i8) as i16 as u16;
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_not(&mut self, a: u16) {
        let r = !a;
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_not8(&mut self, a: u8) {
        let r = !a;
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_abcd8(&mut self, a: u8, b: u8) {
        let x = if self.sr & SR_X != 0 { 1u16 } else { 0 };
        let hr = (b & 0xf) as u16 + (a & 0xf) as u16 + x;
        let lcor = hr > 9;
        let r1 = b as u16 + a as u16 + x;
        let mut r = r1.wrapping_add(if lcor { 6 } else { 0 });
        if r > 0x9f { r = r.wrapping_add(0x60); }
        self.isr = 0;
        if r & 0xff == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r & 0x300 != 0 { self.isr |= SR_X | SR_C; }
        if (r & 0x80 != 0) && (r1 & 0x80 == 0) { self.isr |= SR_V; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_sbcd8(&mut self, a: u8, b: u8) {
        let x = if self.sr & SR_X != 0 { 1u16 } else { 0 };
        let hr = (b & 0xf).wrapping_sub(a & 0xf).wrapping_sub(x as u8);
        let lcor = hr & 0x10 != 0;
        let r1 = (b as u16).wrapping_sub(a as u16).wrapping_sub(x);
        let mut r = r1.wrapping_sub(if lcor { 6 } else { 0 });
        if r1 & 0x100 != 0 { r = r.wrapping_sub(0x60); }
        self.isr = 0;
        if r & 0xff == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r & 0x300 != 0 { self.isr |= SR_X | SR_C; }
        if (r & 0x80 == 0) && (r1 & 0x80 != 0) { self.isr |= SR_V; }
        self.aluo = r;
    }

    // --- Shift/Rotate for MUL/DIV ---

    #[inline(always)]
    pub fn alu_sla0(&mut self, a: u16) {
        let r = ((a as u32) << 17) | ((self.alue as u32) << 1);
        self.isr = self.sr & SR_X;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if a & 0x8000 != 0 { self.isr |= SR_C; }
        self.alue = r as u16;
        self.aluo = (r >> 16) as u16;
    }

    #[inline(always)]
    pub fn alu_sla1(&mut self, a: u16) {
        let r = ((a as u32) << 17) | ((self.alue as u32) << 1) | 1;
        self.isr = self.sr & SR_X;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if a & 0x8000 != 0 { self.isr |= SR_C; }
        self.alue = r as u16;
        self.aluo = (r >> 16) as u16;
    }

    #[inline(always)]
    pub fn alu_over(&mut self, a: u16) {
        self.isr = SR_V | SR_N;
        self.aluo = (a as i8) as i16 as u16;
    }

    // --- ASL ---

    #[inline(always)]
    pub fn alu_asl(&mut self, a: u16) {
        let r = a << 1;
        self.isr = self.sr & SR_V;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if a & 0x8000 != 0 { self.isr |= SR_X | SR_C; }
        if (r ^ a) & 0x8000 != 0 { self.isr |= SR_V; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_asl8(&mut self, a: u8) {
        let r = a << 1;
        self.isr = self.sr & SR_V;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if a & 0x80 != 0 { self.isr |= SR_X | SR_C; }
        if (r ^ a) & 0x80 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_asl32(&mut self, a: u16) {
        let r = ((self.alue as u32) << 17) | ((a as u32) << 1);
        self.isr = self.sr & SR_V;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if self.alue & 0x8000 != 0 { self.isr |= SR_X | SR_C; }
        if ((r >> 16) as u16 ^ self.alue) & 0x8000 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
        self.alue = (r >> 16) as u16;
    }

    // --- ASR ---

    #[inline(always)]
    pub fn alu_asr(&mut self, a: u16) {
        let mut r = a >> 1;
        self.isr = 0;
        if a & 0x8000 != 0 { r |= 0x8000; self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        if a & 0x0001 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_asr8(&mut self, a: u8) {
        let mut r = a >> 1;
        self.isr = 0;
        if a & 0x80 != 0 { r |= 0x80; self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        if a & 0x01 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_asr32(&mut self, a: u16) {
        let mut r = ((self.alue as u32) << 15) | ((a as u32) >> 1);
        self.isr = 0;
        if self.alue & 0x8000 != 0 { r |= 0x80000000; self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        if a & 0x0001 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r as u16;
        self.alue = (r >> 16) as u16;
    }

    // --- LSL ---

    #[inline(always)]
    pub fn alu_lsl(&mut self, a: u16) {
        let r = a << 1;
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if a & 0x8000 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_lsl8(&mut self, a: u8) {
        let r = a << 1;
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if a & 0x80 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_lsl32(&mut self, a: u16) {
        let r = ((self.alue as u32) << 17) | ((a as u32) << 1);
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if self.alue & 0x8000 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r as u16;
        self.alue = (r >> 16) as u16;
    }

    // --- LSR ---

    #[inline(always)]
    pub fn alu_lsr(&mut self, a: u16) {
        let r = a >> 1;
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if a & 0x0001 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_lsr8(&mut self, a: u8) {
        let r = a >> 1;
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if a & 0x01 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_lsr32(&mut self, a: u16) {
        let r = ((self.alue as u32) << 15) | ((a as u32) >> 1);
        self.isr = 0;
        if r == 0 { self.isr |= SR_Z; }
        if a & 0x0001 != 0 { self.isr |= SR_X | SR_C; }
        self.aluo = r as u16;
        self.alue = (r >> 16) as u16;
    }
}
