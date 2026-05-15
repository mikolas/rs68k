use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_mut, unreachable_code, unused_assignments)]
impl M68000 {
    pub fn handler_lsl_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsl(self.aluo);
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

    pub fn handler_lsl_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(((self.da[rx]) as u16), self.dbin);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsl(self.aluo);
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

    pub fn handler_lsl_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.au = self.da[ry].wrapping_sub(2);
        self.alu_and(((self.da[rx]) as u16), self.dbin);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsl(self.aluo);
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

    pub fn handler_lsl_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsl(self.aluo);
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

    pub fn handler_lsl_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.au = self.at;
            self.alu_lsl(self.aluo);
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

    pub fn handler_lsl_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_and(((self.da[rx]) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsl(self.aluo);
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

    pub fn handler_lsl_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_and(((self.da[rx]) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsl(self.aluo);
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

    pub fn handler_roxr_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_andx(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxr(self.aluo);
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

    pub fn handler_roxr_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_andx(self.alub, 0xffffu16);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxr(self.aluo);
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

    pub fn handler_roxr_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.au = self.da[ry].wrapping_sub(2);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_andx(self.alub, 0xffffu16);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxr(self.aluo);
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

    pub fn handler_roxr_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_andx(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxr(self.aluo);
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

    pub fn handler_roxr_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_andx(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
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
            self.au = self.au.wrapping_add(2);
            self.alu_andx(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.au = self.at;
            self.alu_roxr(self.aluo);
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

    pub fn handler_roxr_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
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
        self.alu_andx(self.dbin, 0xffffu16);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxr(self.aluo);
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

    pub fn handler_roxr_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.base_ssw = SSW_PROGRAM | SSW_R;
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
        self.alu_andx(self.dbin, 0xffffu16);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxr(self.aluo);
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

    pub fn handler_roxl_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_andx(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxl(self.aluo);
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

    pub fn handler_roxl_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_andx(self.alub, 0xffffu16);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxl(self.aluo);
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

    pub fn handler_roxl_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
        self.au = self.da[ry].wrapping_sub(2);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_andx(self.alub, 0xffffu16);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxl(self.aluo);
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

    pub fn handler_roxl_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_andx(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxl(self.aluo);
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

    pub fn handler_roxl_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_andx(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
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
            self.au = self.au.wrapping_add(2);
            self.alu_andx(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.au = self.at;
            self.alu_roxl(self.aluo);
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

    pub fn handler_roxl_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
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
        self.alu_andx(self.dbin, 0xffffu16);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxl(self.aluo);
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

    pub fn handler_roxl_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.base_ssw = SSW_PROGRAM | SSW_R;
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
        self.alu_andx(self.dbin, 0xffffu16);
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
        self.au = self.au.wrapping_add(2);
        self.alu_andx(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_roxl(self.aluo);
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

    pub fn handler_ror_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_ror(self.aluo);
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

    pub fn handler_ror_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_ror(self.aluo);
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

    pub fn handler_ror_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_ror(self.aluo);
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

    pub fn handler_ror_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_ror(self.aluo);
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

    pub fn handler_ror_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.au = self.at;
            self.alu_ror(self.aluo);
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

    pub fn handler_ror_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_ror(self.aluo);
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

    pub fn handler_ror_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.base_ssw = SSW_PROGRAM | SSW_R;
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_ror(self.aluo);
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

    pub fn handler_rol_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_rol(self.aluo);
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

    pub fn handler_rol_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_rol(self.aluo);
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

    pub fn handler_rol_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_rol(self.aluo);
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

    pub fn handler_rol_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_rol(self.aluo);
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

    pub fn handler_rol_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.au = self.at;
            self.alu_rol(self.aluo);
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

    pub fn handler_rol_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[rx]) as u8);
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_rol(self.aluo);
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

    pub fn handler_rol_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.base_ssw = SSW_PROGRAM | SSW_R;
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
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_rol(self.aluo);
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

}
