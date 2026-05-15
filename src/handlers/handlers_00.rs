use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_mut, unreachable_code, unused_assignments)]
impl M68000 {
    pub fn handler_state_reset(&mut self, bus: &mut impl crate::types::Bus) {
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.set_ftu_const();
        self.icount -= 2;
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.ftu = 0x0000u16;
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.sr |= 0x0700;
        self.update_interrupt();
        self.au = M68000::ext32(self.ftu);
        self.icount -= 2;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.da[16], v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.da[16], v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        { let v = self.dbin; M68000::set_16l_32(&mut self.da[16], v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        if self.next_state == S_TRACE as u32 { self.next_state = 0; }
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_state_bus_error(&mut self, bus: &mut impl crate::types::Bus) {
        self.ssw = self.base_ssw | (if self.sr & SR_S != 0 { SSW_S } else { 0 });
        self.at = self.aob;
        self.ftu = self.sr;
        self.au = self.pc;
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.pc = self.au;
        self.alu_and(((self.au) as u16), 0xffffu16);
        self.au = self.da[16].wrapping_sub(2);
        self.icount -= 2;
        if self.next_state == S_TRACE as u32 { self.next_state = 0; }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.ftu = self.ird;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.pc = self.at;
        self.ftu = ((self.ftu & !0x1fu16) | self.ssw);
        self.au = self.au.wrapping_sub(2);
        self.alu_and(((self.at) as u16), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.da[16] = self.au;
        self.ftu = 0x0008u16;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.pc = self.at;
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_state_address_error(&mut self, bus: &mut impl crate::types::Bus) {
        self.ssw = self.base_ssw | (if self.sr & SR_S != 0 { SSW_S } else { 0 });
        self.at = self.aob;
        self.ftu = self.sr;
        self.au = self.pc;
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.pc = self.au;
        self.alu_and(((self.au) as u16), 0xffffu16);
        self.au = self.da[16].wrapping_sub(2);
        self.icount -= 2;
        if self.next_state == S_TRACE as u32 { self.next_state = 0; }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.ftu = self.ird;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.pc = self.at;
        self.ftu = ((self.ftu & !0x1fu16) | self.ssw);
        self.au = self.au.wrapping_sub(2);
        self.alu_and(((self.at) as u16), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.da[16] = self.au;
        self.ftu = 0x000cu16;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N | SSW_CRITICAL;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.pc = self.at;
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N | SSW_CRITICAL;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_state_double_fault(&mut self, bus: &mut impl crate::types::Bus) {
        self.icount = 0; return;
    }

    pub fn handler_state_interrupt(&mut self, bus: &mut impl crate::types::Bus) {
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.alub = self.ftu;
        self.pc = self.au;
        self.sr = (self.sr & !SR_I) | (((self.next_state >> 16) as u16) & SR_I);
        self.update_interrupt();
        self.alu_and(((self.au) as u16), 0xffffu16);
        self.icount -= 2;
        self.ftu = (0xfff0u16 | (((self.next_state >> 23) as u16) & 0xe));
        self.au = self.da[16].wrapping_sub(2);
        self.icount -= 2;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.base_ssw = SSW_CPU | SSW_R | SSW_N;
        self.start_interrupt_vector_lookup();
        self.edb = bus.read_cpu(self.aob);
        self.icount -= 4;
        self.end_interrupt_vector_lookup();
        self.irc = self.edb;
        self.ir = self.irc;
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.set_ftu_const();
        self.icount -= 2;
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.da[16] = self.au;
        self.ftu = ((self.int_vector) as u16);
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.pc = self.at;
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_state_trace(&mut self, bus: &mut impl crate::types::Bus) {
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.pc = self.au;
        self.alu_and(((self.au) as u16), 0xffffu16);
        self.au = self.da[16].wrapping_sub(2);
        self.icount -= 2;
        if self.next_state == S_TRACE as u32 { self.next_state = 0; }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.da[16] = self.au;
        self.ftu = 0x0024u16;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.pc = self.at;
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_state_illegal(&mut self, bus: &mut impl crate::types::Bus) {
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.pc = self.au;
        self.alu_and(((self.au) as u16), 0xffffu16);
        self.au = self.da[16].wrapping_sub(2);
        self.icount -= 2;
        if self.next_state == S_TRACE as u32 { self.next_state = 0; }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.da[16] = self.au;
        self.ftu = 0x0010u16;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.pc = self.at;
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_state_priviledge(&mut self, bus: &mut impl crate::types::Bus) {
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.pc = self.au;
        self.alu_and(((self.au) as u16), 0xffffu16);
        self.au = self.da[16].wrapping_sub(2);
        self.icount -= 2;
        if self.next_state == S_TRACE as u32 { self.next_state = 0; }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.da[16] = self.au;
        self.ftu = 0x0020u16;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.pc = self.at;
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_state_linea(&mut self, bus: &mut impl crate::types::Bus) {
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.pc = self.au;
        self.alu_and(((self.au) as u16), 0xffffu16);
        self.au = self.da[16].wrapping_sub(2);
        self.icount -= 2;
        if self.next_state == S_TRACE as u32 { self.next_state = 0; }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.da[16] = self.au;
        self.ftu = 0x0028u16;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.pc = self.at;
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_state_linef(&mut self, bus: &mut impl crate::types::Bus) {
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.pc = self.au;
        self.alu_and(((self.au) as u16), 0xffffu16);
        self.au = self.da[16].wrapping_sub(2);
        self.icount -= 2;
        if self.next_state == S_TRACE as u32 { self.next_state = 0; }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.da[16] = self.au;
        self.ftu = 0x002cu16;
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.at = M68000::ext32(self.ftu);
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_N;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.at;
        self.pc = self.at;
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R | SSW_N;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.icount -= 2;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R | SSW_N;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_b_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_or8(((self.dt) as u8), ((self.da[ry]) as u8));
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_b_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_b_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.dt) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_b_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_b_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_b_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_ori_b_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_b_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_imm8_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.alu_or8(((self.ftu) as u8), ((self.dt) as u8));
        self.icount -= 2;
        self.movemr = self.dbin;
        self.ftu = self.aluo;
        self.au = self.au.wrapping_add(2);
        self.icount -= 2;
        self.isr = (self.ftu) & SR_CCR | (self.sr & SR_SR); self.sr = self.isr;
        self.icount -= 2;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_w_imm16_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_or(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_w_imm16_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_w_imm16_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.dt) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_or(((self.dt) as u16), self.dbin);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_w_imm16_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.au = self.da[ry].wrapping_sub(2);
        self.alu_or(((self.dt) as u16), self.dbin);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_w_imm16_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_w_imm16_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_ori_w_imm16_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_or(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_w_imm16_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_or(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_i16u_sr(&mut self, bus: &mut impl crate::types::Bus) {
        if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.alu_or(self.ftu, ((self.dt) as u16));
        self.icount -= 2;
        self.movemr = self.dbin;
        self.ftu = self.aluo;
        self.au = self.au.wrapping_add(2);
        self.icount -= 2;
        self.isr = (self.ftu) & (SR_CCR|SR_SR); self.sr = self.isr;
        self.update_user_super();
        self.update_interrupt();
        self.icount -= 2;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_l_imm32_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_or(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_nzvc();
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_or(M68000::high16(self.dt), M68000::high16(self.da[ry]));
        self.sr_nzvc_u();
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_l_imm32_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_or(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_l_imm32_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au.wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_or(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_l_imm32_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.au = self.da[ry].wrapping_sub(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_or(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_l_imm32_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_or(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_l_imm32_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_or(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_ori_l_imm32_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_or(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_or(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_ori_l_imm32_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_or(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_or(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[rx];
            self.au = self.da[ry];
            self.alu_eor(((self.da[rx]) as u16), ((self.da[ry]) as u16));
            self.t = (self.dcr & 0x10) as i32;
            self.alub = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.alu_and(self.alub, (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(M68000::high16(self.da[ry]), (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_movep_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        M68000::set_8h(&mut self.dbin, ((self.edb) as u8));
        self.aob = self.au;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        M68000::set_8_16(&mut self.dbin, ((self.edb) as u8));
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.da[rx], v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_btst_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_dpc(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_dd_dpci(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.pc.wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_btst_dd_imm(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            { let v = M68000::high16(self.at); M68000::set_16h(&mut self.dt, v); }
            self.t = (self.dcr & 0x10) as i32;
            self.alub = ((self.dt) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(((self.dt) as u16), 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.dt, v); }
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and8(M68000::high16(self.dt), (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_bchg_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[rx];
            self.au = self.da[ry];
            self.alu_eor(((self.da[rx]) as u16), ((self.da[ry]) as u16));
            self.t = (self.dcr & 0x10) as i32;
            self.alub = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_eor(((self.da[ry]) as u16), (1u16 << (self.dcr & 15)));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.alu_and(self.alub, (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        2 => {
            self.alub = M68000::high16(self.da[ry]);
            self.alu_eor(M68000::high16(self.da[ry]), (1u16 << (self.dcr & 15)));
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
            self.alu_and(self.alub, (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_movep_l_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        M68000::set_8h(&mut self.dbin, ((self.edb) as u8));
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        M68000::set_8_16(&mut self.dbin, ((self.edb) as u8));
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.da[rx], v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        M68000::set_8h(&mut self.dbin, ((self.edb) as u8));
        self.aob = self.au;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        M68000::set_8_16(&mut self.dbin, ((self.edb) as u8));
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.da[rx], v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bchg_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bchg_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bchg_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bchg_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bchg_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_bchg_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bchg_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bclr_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[rx];
            self.au = self.da[ry];
            self.alu_or(((self.da[rx]) as u16), ((self.da[ry]) as u16));
            self.t = (self.dcr & 0x10) as i32;
            self.alub = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_or(((self.da[ry]) as u16), (1u16 << (self.dcr & 15)));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.alu_eor(self.aluo, (1u16 << (self.dcr & 15)));
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.alu_and(self.alub, (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        2 => {
            self.alub = M68000::high16(self.da[ry]);
            self.alu_or(M68000::high16(self.da[ry]), (1u16 << (self.dcr & 15)));
            self.icount -= 2;
            self.alu_eor(self.aluo, (1u16 << (self.dcr & 15)));
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
            self.alu_and(self.alub, (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_movep_w_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        M68000::set_8xh(&mut self.dbout, ((self.da[rx]) as u16));
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        M68000::set_8xl(&mut self.dbout, ((self.da[rx]) as u16));
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bclr_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.ftu = self.sr;
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bclr_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.alu_or8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.ftu = self.sr;
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bclr_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.alu_or8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.ftu = self.sr;
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bclr_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.ftu = self.sr;
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bclr_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.alub = self.dbin;
            self.ftu = self.sr;
            self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_bclr_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_or8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.ftu = self.sr;
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bclr_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_or8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.ftu = self.sr;
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bset_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[rx];
            self.au = self.da[ry];
            self.alu_or(((self.da[rx]) as u16), ((self.da[ry]) as u16));
            self.t = (self.dcr & 0x10) as i32;
            self.alub = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_or(((self.da[ry]) as u16), (1u16 << (self.dcr & 15)));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.alu_and(self.alub, (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        2 => {
            self.alub = M68000::high16(self.da[ry]);
            self.alu_or(M68000::high16(self.da[ry]), (1u16 << (self.dcr & 15)));
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
            self.alu_and(self.alub, (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_movep_l_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        M68000::set_8xh(&mut self.dbout, M68000::high16(self.da[rx]));
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        M68000::set_8xl(&mut self.dbout, M68000::high16(self.da[rx]));
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ir = self.irc;
        M68000::set_8xh(&mut self.dbout, ((self.da[rx]) as u16));
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        M68000::set_8xl(&mut self.dbout, ((self.da[rx]) as u16));
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bset_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bset_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.alu_or8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bset_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.alu_or8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bset_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bset_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_bset_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_or8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_bset_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_or8(((self.da[rx]) as u8), ((self.dbin) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
        self.sr_z();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_b_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_and8(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_b_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_b_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.dt) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_b_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_b_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_b_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_andi_b_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_b_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and8(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_imm8_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.alu_and8(self.ftu, ((self.dt) as u16));
        self.icount -= 2;
        self.movemr = self.dbin;
        self.ftu = self.aluo;
        self.au = self.au.wrapping_add(2);
        self.icount -= 2;
        self.isr = (self.ftu) & SR_CCR | (self.sr & SR_SR); self.sr = self.isr;
        self.icount -= 2;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_w_imm16_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_and(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_w_imm16_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_w_imm16_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.dt) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(((self.dt) as u16), self.dbin);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_w_imm16_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.au = self.da[ry].wrapping_sub(2);
        self.alu_and(((self.dt) as u16), self.dbin);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_w_imm16_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_w_imm16_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_andi_w_imm16_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_and(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_w_imm16_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_and(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_i16u_sr(&mut self, bus: &mut impl crate::types::Bus) {
        if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.alu_and(self.ftu, ((self.dt) as u16));
        self.icount -= 2;
        self.movemr = self.dbin;
        self.ftu = self.aluo;
        self.au = self.au.wrapping_add(2);
        self.icount -= 2;
        self.isr = (self.ftu) & (SR_CCR|SR_SR); self.sr = self.isr;
        self.update_user_super();
        self.update_interrupt();
        self.icount -= 2;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_l_imm32_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_and(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_nzvc();
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_and(M68000::high16(self.dt), M68000::high16(self.da[ry]));
        self.sr_nzvc_u();
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_l_imm32_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_and(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_l_imm32_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au.wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_and(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_l_imm32_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.au = self.da[ry].wrapping_sub(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_and(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_l_imm32_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_and(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_l_imm32_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_and(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_andi_l_imm32_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_and(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_and(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_andi_l_imm32_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_and(((self.dt) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(((self.dt) as u16), self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_and(M68000::high16(self.dt), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_b_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_sub8(((self.dt) as u8), ((self.da[ry]) as u8));
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_b_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_b_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.dt) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_b_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_b_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_b_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_subi_b_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_b_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_w_imm16_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_sub(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_w_imm16_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_w_imm16_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.dt) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_w_imm16_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.au = self.da[ry].wrapping_sub(2);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_w_imm16_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_w_imm16_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_subi_w_imm16_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_w_imm16_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_l_imm32_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_sub(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_xnzvc();
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.dt), M68000::high16(self.da[ry]));
        self.sr_xnzvc_u();
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_l_imm32_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_subc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_l_imm32_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au.wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_subc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_l_imm32_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.au = self.da[ry].wrapping_sub(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_subc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_l_imm32_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_subc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_l_imm32_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_subc(M68000::high16(self.dt), self.alue);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_subi_l_imm32_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_subc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subi_l_imm32_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_subc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_b_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_add8(((self.dt) as u8), ((self.da[ry]) as u8));
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_b_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_b_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.dt) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_b_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_b_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_b_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_add8(((self.dt) as u8), ((self.dbin) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_addi_b_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_b_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and8(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add8(((self.dt) as u8), ((self.dbin) as u8));
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_w_imm16_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_add(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_w_imm16_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_w_imm16_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.dt) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_w_imm16_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.au = self.da[ry].wrapping_sub(2);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_w_imm16_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_w_imm16_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_add(((self.dt) as u16), self.dbin);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_addi_w_imm16_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_w_imm16_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_l_imm32_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.alu_add(((self.dt) as u16), ((self.da[ry]) as u16));
        self.sr_xnzvc();
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_addc(M68000::high16(self.dt), M68000::high16(self.da[ry]));
        self.sr_xnzvc_u();
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_l_imm32_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_addc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_l_imm32_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au.wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_addc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_l_imm32_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.au = self.da[ry].wrapping_sub(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_addc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_l_imm32_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_addc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_l_imm32_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_add(((self.dt) as u16), self.dbin);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_addc(M68000::high16(self.dt), self.alue);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_addi_l_imm32_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_addc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addi_l_imm32_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.dt) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_add(((self.dt) as u16), self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(2);
        self.alu_addc(M68000::high16(self.dt), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_btst_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = self.dt;
            self.au = self.da[ry];
            self.alu_eor(((self.dt) as u16), ((self.da[ry]) as u16));
            self.t = (self.dcr & 0x10) as i32;
            self.alub = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.alu_and(self.alub, (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(M68000::high16(self.da[ry]), (1u16 << (self.dcr & 15)));
            self.sr_z();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

}
