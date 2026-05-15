use crate::cpu::M68000;
use crate::types::*;

/// Rotate operations and SR update methods
impl M68000 {
    // --- ROL ---

    #[inline(always)]
    pub fn alu_rol(&mut self, a: u16) {
        let mut r = a << 1;
        self.isr = 0;
        if a & 0x8000 != 0 { self.isr |= SR_X | SR_C; r |= 0x0001; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_rol8(&mut self, a: u8) {
        let mut r = a << 1;
        self.isr = 0;
        if a & 0x80 != 0 { self.isr |= SR_X | SR_C; r |= 0x01; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_rol32(&mut self, a: u16) {
        let mut r = ((self.alue as u32) << 17) | ((a as u32) << 1);
        self.isr = 0;
        if self.alue & 0x8000 != 0 { self.isr |= SR_X | SR_C; r |= 0x00000001; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r as u16;
        self.alue = (r >> 16) as u16;
    }

    // --- ROR ---

    #[inline(always)]
    pub fn alu_ror(&mut self, a: u16) {
        let mut r = a >> 1;
        self.isr = 0;
        if a & 0x0001 != 0 { self.isr |= SR_X | SR_C | SR_N; r |= 0x8000; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_ror8(&mut self, a: u8) {
        let mut r = a >> 1;
        self.isr = 0;
        if a & 0x01 != 0 { self.isr |= SR_X | SR_C | SR_N; r |= 0x80; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_ror32(&mut self, a: u16) {
        let mut r = ((self.alue as u32) << 15) | ((a as u32) >> 1);
        self.isr = 0;
        if a & 0x0001 != 0 { self.isr |= SR_X | SR_C | SR_N; r |= 0x80000000; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r as u16;
        self.alue = (r >> 16) as u16;
    }

    // --- ROXL ---

    #[inline(always)]
    pub fn alu_roxl(&mut self, a: u16) {
        let r = (a << 1) | if self.sr & SR_X != 0 { 1 } else { 0 };
        self.isr = 0;
        if a & 0x8000 != 0 { self.isr |= SR_X | SR_C; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_roxl8(&mut self, a: u8) {
        let r = (a << 1) | if self.sr & SR_X != 0 { 1 } else { 0 };
        self.isr = 0;
        if a & 0x80 != 0 { self.isr |= SR_X | SR_C; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_roxl32(&mut self, a: u16) {
        let r = ((self.alue as u32) << 17) | ((a as u32) << 1) | if self.sr & SR_X != 0 { 1 } else { 0 };
        self.isr = 0;
        if self.alue & 0x8000 != 0 { self.isr |= SR_X | SR_C; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r as u16;
        self.alue = (r >> 16) as u16;
    }

    // --- ROXR ---

    #[inline(always)]
    pub fn alu_roxr(&mut self, a: u16) {
        let r = (a >> 1) | if self.sr & SR_X != 0 { 0x8000 } else { 0 };
        self.isr = 0;
        if a & 0x0001 != 0 { self.isr |= SR_X | SR_C; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_roxr8(&mut self, a: u8) {
        let r = (a >> 1) | if self.sr & SR_X != 0 { 0x80 } else { 0 };
        self.isr = 0;
        if a & 0x01 != 0 { self.isr |= SR_X | SR_C; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_roxr32(&mut self, a: u32) {
        let r = ((self.alue as u32) << 15) | (a >> 1) | if self.sr & SR_X != 0 { 0x80000000 } else { 0 };
        self.isr = 0;
        if a & 0x0001 != 0 { self.isr |= SR_X | SR_C; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if r == 0 { self.isr |= SR_Z; }
        self.aluo = r as u16;
        self.alue = (r >> 16) as u16;
    }

    // --- MUL/DIV shift helpers ---

    #[inline(always)]
    pub fn alu_roxr32ms(&mut self, a: u16) {
        let sign_bit = if (self.isr & (SR_N | SR_V)) == SR_N || (self.isr & (SR_N | SR_V)) == SR_V {
            0x80000000u32
        } else {
            0
        };
        let r = ((a as u32) << 15) | ((self.alue as u32) >> 1) | sign_bit;
        self.isr = 0;
        if a & 0x0001 != 0 { self.isr |= SR_X; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if r & 0xffff0000 == 0 { self.isr |= SR_Z; }
        self.aluo = (r >> 16) as u16;
        self.alue = r as u16;
    }

    #[inline(always)]
    pub fn alu_roxr32mu(&mut self, a: u16) {
        let carry_bit = if self.isr & SR_C != 0 { 0x80000000u32 } else { 0 };
        let r = ((a as u32) << 15) | ((self.alue as u32) >> 1) | carry_bit;
        self.isr = 0;
        if a & 0x0001 != 0 { self.isr |= SR_X; }
        if r & 0x80000000 != 0 { self.isr |= SR_N; }
        if r & 0xffff0000 == 0 { self.isr |= SR_Z; }
        self.aluo = (r >> 16) as u16;
        self.alue = r as u16;
    }

    // --- SR update methods ---

    #[inline(always)]
    pub fn sr_z(&mut self) {
        self.sr = (self.sr & !SR_Z) | (self.isr & SR_Z);
    }

    #[inline(always)]
    pub fn sr_nz_u(&mut self) {
        self.sr = (self.sr & !SR_N & (self.isr | !SR_Z)) | (self.isr & SR_N);
    }

    #[inline(always)]
    pub fn sr_nzvc(&mut self) {
        self.sr = (self.sr & !(SR_N | SR_Z | SR_V | SR_C)) | (self.isr & (SR_N | SR_Z | SR_V | SR_C));
    }

    #[inline(always)]
    pub fn sr_nzvc_u(&mut self) {
        self.sr = (self.sr & !(SR_N | SR_V | SR_C) & (self.isr | !SR_Z)) | (self.isr & (SR_N | SR_V | SR_C));
    }

    #[inline(always)]
    pub fn sr_xnzvc(&mut self) {
        self.sr = (self.sr & !(SR_X | SR_N | SR_Z | SR_V | SR_C)) | (self.isr & (SR_X | SR_N | SR_Z | SR_V | SR_C));
    }

    #[inline(always)]
    pub fn sr_xnzvc_u(&mut self) {
        self.sr = (self.sr & !(SR_X | SR_N | SR_V | SR_C) & (self.isr | !SR_Z)) | (self.isr & (SR_X | SR_N | SR_V | SR_C));
    }
}
