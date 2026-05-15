use crate::cpu::M68000;
use crate::types::*;

impl M68000 {
    #[inline(always)]
    pub fn alu_add(&mut self, a: u16, b: u16) {
        let r = (b as u32).wrapping_add(a as u32);
        self.isr = 0;
        if r & 0xffff == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r & 0x10000 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & a & !r as u16) | ((!b) & (!a) & r as u16)) & 0x8000 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_add8(&mut self, a: u8, b: u8) {
        let r = (b as u16).wrapping_add(a as u16);
        self.isr = 0;
        if r & 0xff == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r & 0x100 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & a & !(r as u8)) | ((!b) & (!a) & r as u8)) & 0x80 != 0 { self.isr |= SR_V; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_addc(&mut self, a: u16, b: u16) {
        let c = if self.isr & SR_C != 0 { 1u32 } else { 0 };
        let r = (b as u32).wrapping_add(a as u32).wrapping_add(c);
        self.isr = 0;
        if r & 0xffff == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r & 0x10000 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & a & !r as u16) | ((!b) & (!a) & r as u16)) & 0x8000 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_addx(&mut self, a: u16, b: u16) {
        let x = if self.sr & SR_X != 0 { 1u32 } else { 0 };
        let r = (b as u32).wrapping_add(a as u32).wrapping_add(x);
        self.isr = 0;
        if r & 0xffff == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r & 0x10000 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & a & !r as u16) | ((!b) & (!a) & r as u16)) & 0x8000 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_addx8(&mut self, a: u8, b: u8) {
        let x = if self.sr & SR_X != 0 { 1u16 } else { 0 };
        let r = (b as u16).wrapping_add(a as u16).wrapping_add(x);
        self.isr = 0;
        if r & 0xff == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r & 0x100 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & a & !(r as u8)) | ((!b) & (!a) & r as u8)) & 0x80 != 0 { self.isr |= SR_V; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_sub(&mut self, a: u16, b: u16) {
        let r = (b as u32).wrapping_sub(a as u32);
        self.isr = 0;
        if r & 0xffff == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r & 0x10000 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & (!a) & !(r as u16)) | ((!b) & a & r as u16)) & 0x8000 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_sub8(&mut self, a: u8, b: u8) {
        let r = (b as u16).wrapping_sub(a as u16);
        self.isr = 0;
        if r & 0xff == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r & 0x100 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & (!a) & !(r as u8)) | ((!b) & a & r as u8)) & 0x80 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_subc(&mut self, a: u16, b: u16) {
        let c = if self.isr & SR_C != 0 { 1u32 } else { 0 };
        let r = (b as u32).wrapping_sub(a as u32).wrapping_sub(c);
        self.isr = 0;
        if r & 0xffff == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r & 0x10000 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & (!a) & !(r as u16)) | ((!b) & a & r as u16)) & 0x8000 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_subx(&mut self, a: u16, b: u16) {
        let x = if self.sr & SR_X != 0 { 1u32 } else { 0 };
        let r = (b as u32).wrapping_sub(a as u32).wrapping_sub(x);
        self.isr = 0;
        if r & 0xffff == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        if r & 0x10000 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & (!a) & !(r as u16)) | ((!b) & a & r as u16)) & 0x8000 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_subx8(&mut self, a: u8, b: u8) {
        let x = if self.sr & SR_X != 0 { 1u16 } else { 0 };
        let r = (b as u16).wrapping_sub(a as u16).wrapping_sub(x);
        self.isr = 0;
        if r & 0xff == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        if r & 0x100 != 0 { self.isr |= SR_X | SR_C; }
        if ((b & (!a) & !(r as u8)) | ((!b) & a & r as u8)) & 0x80 != 0 { self.isr |= SR_V; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_and(&mut self, a: u16, b: u16) {
        let r = b & a;
        self.isr = self.sr & SR_X;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_andx(&mut self, a: u16, b: u16) {
        let r = b & a;
        self.isr = if self.sr & SR_X != 0 { SR_X | SR_C } else { 0 };
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_and8(&mut self, a: u16, b: u16) {
        let r = b & a;
        self.isr = self.sr & SR_X;
        if r & 0xff == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_and8x(&mut self, a: u8, b: u8) {
        let r = b & a;
        self.isr = if self.sr & SR_X != 0 { SR_X | SR_C } else { 0 };
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_or(&mut self, a: u16, b: u16) {
        let r = b | a;
        self.isr = self.sr & SR_X;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_or8(&mut self, a: u8, b: u8) {
        let r = b | a;
        self.isr = self.sr & SR_X;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        self.aluo = r as u16;
    }

    #[inline(always)]
    pub fn alu_eor(&mut self, a: u16, b: u16) {
        let r = b ^ a;
        self.isr = self.sr & SR_X;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x8000 != 0 { self.isr |= SR_N; }
        self.aluo = r;
    }

    #[inline(always)]
    pub fn alu_eor8(&mut self, a: u8, b: u8) {
        let r = b ^ a;
        self.isr = self.sr & SR_X;
        if r == 0 { self.isr |= SR_Z; }
        if r & 0x80 != 0 { self.isr |= SR_N; }
        self.aluo = r as u16;
    }
}
