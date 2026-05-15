use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_assignments, unused_mut, unreachable_code)]
impl M68000 {
    pub fn handler_divu_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.da[rx];
            self.au = self.da[ry] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_Z;
            self.alu_sub(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = self.isr & SR_C;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        3 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 8; continue; } else { __state = 7; continue; }
        }
        6 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 10; continue; } else { __state = 9; continue; }
        }
        7 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 11; continue; } else { __state = 9; continue; }
        }
        8 => {
            self.t = self.isr & SR_C;
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        9 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        10 => {
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            __state = 12; continue;
        }
        11 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        12 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            self.au = self.pc + 2;
            self.alu_sla1(self.aluo);
            __state = 14; continue;
        }
        13 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            M68000::set_16h(&mut self.da[rx], self.at);
            self.au = self.pc + 2;
            self.alu_sla0(self.aluo);
            __state = 14; continue;
        }
        14 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.alue);
            self.alu_sub(self.alub, self.alue);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_divu_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin) + self.da[ry];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_Z;
            self.alu_sub(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = self.isr & SR_C;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        3 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 8; continue; } else { __state = 7; continue; }
        }
        6 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 10; continue; } else { __state = 9; continue; }
        }
        7 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 11; continue; } else { __state = 9; continue; }
        }
        8 => {
            self.t = self.isr & SR_C;
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        9 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        10 => {
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            __state = 12; continue;
        }
        11 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        12 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            self.au = self.pc + 2;
            self.alu_sla1(self.aluo);
            __state = 14; continue;
        }
        13 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            M68000::set_16h(&mut self.da[rx], self.at);
            self.au = self.pc + 2;
            self.alu_sla0(self.aluo);
            __state = 14; continue;
        }
        14 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.alue);
            self.alu_sub(self.alub, self.alue);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_divu_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_over(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_Z;
            self.alu_sub(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.t = self.isr & SR_C;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 7; continue; } else { __state = 6; continue; }
        }
        5 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        6 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        7 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 11; continue; } else { __state = 10; continue; }
        }
        9 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        10 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 14; continue; } else { __state = 12; continue; }
        }
        11 => {
            self.t = self.isr & SR_C;
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 16; continue; } else { __state = 15; continue; }
        }
        12 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        13 => {
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            __state = 15; continue;
        }
        14 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        15 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            self.au = self.pc + 2;
            self.alu_sla1(self.aluo);
            __state = 17; continue;
        }
        16 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            M68000::set_16h(&mut self.da[rx], self.at);
            self.au = self.pc + 2;
            self.alu_sla0(self.aluo);
            __state = 17; continue;
        }
        17 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.alue);
            self.alu_sub(self.alub, self.alue);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_divu_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.da[rx];
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_Z;
            self.alu_sub(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = self.isr & SR_C;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        3 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 8; continue; } else { __state = 7; continue; }
        }
        6 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 10; continue; } else { __state = 9; continue; }
        }
        7 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 11; continue; } else { __state = 9; continue; }
        }
        8 => {
            self.t = self.isr & SR_C;
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        9 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        10 => {
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            __state = 12; continue;
        }
        11 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        12 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            self.au = self.pc + 2;
            self.alu_sla1(self.aluo);
            __state = 14; continue;
        }
        13 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            M68000::set_16h(&mut self.da[rx], self.at);
            self.au = self.pc + 2;
            self.alu_sla0(self.aluo);
            __state = 14; continue;
        }
        14 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.alue);
            self.alu_sub(self.alub, self.alue);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_divu_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.da[rx];
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_Z;
            self.alu_sub(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = self.isr & SR_C;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        3 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 8; continue; } else { __state = 7; continue; }
        }
        6 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 10; continue; } else { __state = 9; continue; }
        }
        7 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 11; continue; } else { __state = 9; continue; }
        }
        8 => {
            self.t = self.isr & SR_C;
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        9 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        10 => {
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            __state = 12; continue;
        }
        11 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        12 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            self.au = self.pc + 2;
            self.alu_sla1(self.aluo);
            __state = 14; continue;
        }
        13 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            M68000::set_16h(&mut self.da[rx], self.at);
            self.au = self.pc + 2;
            self.alu_sla0(self.aluo);
            __state = 14; continue;
        }
        14 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.alue);
            self.alu_sub(self.alub, self.alue);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_divu_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_Z;
            self.alu_sub(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = self.isr & SR_C;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        3 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 8; continue; } else { __state = 7; continue; }
        }
        6 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 10; continue; } else { __state = 9; continue; }
        }
        7 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 11; continue; } else { __state = 9; continue; }
        }
        8 => {
            self.t = self.isr & SR_C;
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        9 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        10 => {
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            __state = 12; continue;
        }
        11 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        12 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            self.au = self.pc + 2;
            self.alu_sla1(self.aluo);
            __state = 14; continue;
        }
        13 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            M68000::set_16h(&mut self.da[rx], self.at);
            self.au = self.pc + 2;
            self.alu_sla0(self.aluo);
            __state = 14; continue;
        }
        14 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.alue);
            self.alu_sub(self.alub, self.alue);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_divu_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_over(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_Z;
            self.alu_sub(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.t = self.isr & SR_C;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 7; continue; } else { __state = 6; continue; }
        }
        5 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        6 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        7 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 11; continue; } else { __state = 10; continue; }
        }
        9 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        10 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 14; continue; } else { __state = 12; continue; }
        }
        11 => {
            self.t = self.isr & SR_C;
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 16; continue; } else { __state = 15; continue; }
        }
        12 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        13 => {
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            __state = 15; continue;
        }
        14 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        15 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            self.au = self.pc + 2;
            self.alu_sla1(self.aluo);
            __state = 17; continue;
        }
        16 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            M68000::set_16h(&mut self.da[rx], self.at);
            self.au = self.pc + 2;
            self.alu_sla0(self.aluo);
            __state = 17; continue;
        }
        17 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.alue);
            self.alu_sub(self.alub, self.alue);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_divu_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            M68000::set_16l_32(&mut self.dt, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alub = self.dt;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dt);
            self.alu_and(self.dt, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_Z;
            self.alu_sub(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = self.isr & SR_C;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        3 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 8; continue; } else { __state = 7; continue; }
        }
        6 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 10; continue; } else { __state = 9; continue; }
        }
        7 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 11; continue; } else { __state = 9; continue; }
        }
        8 => {
            self.t = self.isr & SR_C;
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        9 => {
            self.t = self.isr & SR_N;
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        10 => {
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.alu_and(self.aluo, 0x0000);
            self.icount -= 2;
            __state = 12; continue;
        }
        11 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        12 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            self.au = self.pc + 2;
            self.alu_sla1(self.aluo);
            __state = 14; continue;
        }
        13 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.alub = self.aluo;
            M68000::set_16h(&mut self.da[rx], self.at);
            self.au = self.pc + 2;
            self.alu_sla0(self.aluo);
            __state = 14; continue;
        }
        14 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.alue);
            self.alu_sub(self.alub, self.alue);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_sbcd_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sbcd8(self.da[ry], self.da[rx]);
        self.sr_xnzvc_u();
        self.au = self.at;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sbcd_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = 0;
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.alu_and8(self.dbin, 0xffff);
        self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.at = self.au;
        self.da[rx] = self.au;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.pc;
        self.alu_sbcd8(self.alub, self.dbin);
        self.sr_xnzvc_u();
        self.au = self.at;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_b_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_b_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = self.da[rx];
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.alu_or8(self.da[rx], self.dbin);
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and8(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_b_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.alu_or8(self.da[rx], self.dbin);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_b_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_b_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(self.da[rx], self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc + 2;
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_or_b_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_or8(self.da[rx], self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and8(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_b_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_or8(self.da[rx], self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and8(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or8(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_w_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_w_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = self.da[rx];
        self.at = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_or(self.da[rx], self.dbin);
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_w_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.au = self.da[ry] - 2;
        self.alu_or(self.da[rx], self.dbin);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_w_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_w_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or(self.da[rx], self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc + 2;
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_or_w_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_or(self.da[rx], self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_w_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_or(self.da[rx], self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_l_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_or(M68000::high16(self.da[rx]), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_l_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au + 2;
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_or(M68000::high16(self.da[rx]), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_l_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.au = self.da[ry] - 4;
        self.alu_and(self.dbin, 0xffff);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_or(M68000::high16(self.da[rx]), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_l_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 4;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_or(M68000::high16(self.da[rx]), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_l_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc + 4;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or(self.da[rx], self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 2;
            self.alu_or(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc + 2;
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_or_l_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_or(self.da[rx], self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_or(M68000::high16(self.da[rx]), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_or_l_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_or(self.da[rx], self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_or(self.da[rx], self.dbin);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_or(M68000::high16(self.da[rx]), self.alue);
        self.sr_nzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_divs_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.da[ry];
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.da[ry]);
            self.alu_and(self.da[ry], 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.da[rx];
            self.at = self.da[ry];
            self.au = self.da[ry] + 2;
            self.da[ry] = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.da[rx];
            self.au = self.da[ry] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin) + self.da[ry];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_over(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 4; continue; }
            else if(self.t == 1)
            __state = 5; continue;
            else
            __state = 6; continue;
        }
        4 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        5 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        7 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 10; continue;
        }
        9 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 10; continue;
        }
        10 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 12; continue; } else { __state = 11; continue; }
        }
        11 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 13; continue;
        }
        12 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 14; continue;
        }
        13 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        14 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 16; continue; } else { __state = 15; continue; }
        }
        15 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 18; continue; } else { __state = 17; continue; }
        }
        16 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 14; continue;
        }
        18 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 12; continue;
        }
        19 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 21; continue;
        }
        20 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 21; continue;
        }
        21 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 23; continue; } else { __state = 22; continue; }
        }
        22 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 25; continue; } else { __state = 24; continue; }
        }
        23 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        24 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 13; continue; } else { __state = 28; continue; }
        }
        25 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 30; continue; } else { __state = 29; continue; }
        }
        26 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 28; continue; } else { __state = 13; continue; }
        }
        27 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 29; continue; } else { __state = 30; continue; }
        }
        28 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        29 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 13; continue;
        }
        30 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 28; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.da[rx];
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.da[rx];
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_over(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.alub = self.dbin;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 4; continue; }
            else if(self.t == 1)
            __state = 5; continue;
            else
            __state = 6; continue;
        }
        4 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        5 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        7 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 10; continue;
        }
        9 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 10; continue;
        }
        10 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 12; continue; } else { __state = 11; continue; }
        }
        11 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 13; continue;
        }
        12 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 14; continue;
        }
        13 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        14 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 16; continue; } else { __state = 15; continue; }
        }
        15 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 18; continue; } else { __state = 17; continue; }
        }
        16 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 14; continue;
        }
        18 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 12; continue;
        }
        19 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 21; continue;
        }
        20 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 21; continue;
        }
        21 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 23; continue; } else { __state = 22; continue; }
        }
        22 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 25; continue; } else { __state = 24; continue; }
        }
        23 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        24 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 13; continue; } else { __state = 28; continue; }
        }
        25 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 30; continue; } else { __state = 29; continue; }
        }
        26 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 28; continue; } else { __state = 13; continue; }
        }
        27 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 29; continue; } else { __state = 30; continue; }
        }
        28 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        29 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 13; continue;
        }
        30 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 28; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_divs_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            M68000::set_16l_32(&mut self.dt, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alub = self.dt;
            self.alue = self.da[rx];
            self.pc = self.au;
            M68000::set_16h(&mut self.at, self.dt);
            self.alu_and(self.dt, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 };
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t == 0 { __state = 1; continue; }
            else if(self.t == 1)
            __state = 2; continue;
            else
            __state = 3; continue;
        }
        1 => {
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        2 => {
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, M68000::high16(self.da[rx]));
            self.au = M68000::merge_16_32(0, self.ftu) + 1;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 4; continue;
        }
        3 => {
            self.ftu = self.sr;
            self.au = self.pc - 2;
            self.icount -= 2;
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.pc = self.au;
            self.alu_and(self.au, 0xffff);
            self.au = self.da[16] - 2;
            self.icount -= 2;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 4;
            self.alu_and(self.ftu, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0014;
            self.au = self.au + 2;
            self.alu_and(M68000::high16(self.pc), 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au - 4;
            self.alu_and(self.alub, 0xffff);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            M68000::set_16h(&mut self.at, self.dbin);
            self.au = self.au + 2;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin) + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        4 => {
            self.t = self.isr & SR_N;
            self.alu_sub(self.da[rx], 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 6; continue; } else { __state = 5; continue; }
        }
        5 => {
            self.alu_sub(self.alub, self.at);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        6 => {
            self.alue = self.aluo;
            self.alu_subc(M68000::high16(self.da[rx]), 0x0000);
            self.icount -= 2;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 7; continue;
        }
        7 => {
            self.t = self.isr & SR_C;
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 9; continue; } else { __state = 8; continue; }
        }
        8 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        9 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        10 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_over(0x0000);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        11 => {
            self.t = !(self.au & 0x3f);
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(self.alub, self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 13; continue; } else { __state = 12; continue; }
        }
        12 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 15; continue; } else { __state = 14; continue; }
        }
        13 => {
            self.t = self.isr & SR_C;
            self.icount -= 2;
            if self.t != 0 { __state = 17; continue; } else { __state = 16; continue; }
        }
        14 => {
            self.au = self.au - 1;
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 11; continue;
        }
        15 => {
            self.alu_and(self.at, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            __state = 9; continue;
        }
        16 => {
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sla1(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        17 => {
            self.au = self.au - 1;
            self.alu_sla0(self.aluo);
            self.icount -= 2;
            __state = 18; continue;
        }
        18 => {
            self.alu_and(M68000::high16(self.at), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = self.isr & SR_N;
            self.alub = self.alue;
            self.alu_and(M68000::high16(self.da[rx]), 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 20; continue; } else { __state = 19; continue; }
        }
        19 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 22; continue; } else { __state = 21; continue; }
        }
        20 => {
            self.t = self.isr & SR_N;
            M68000::set_16h(&mut self.at, self.at);
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 24; continue; } else { __state = 23; continue; }
        }
        21 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.t = self.isr & SR_N;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 10; continue; } else { __state = 25; continue; }
        }
        22 => {
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 27; continue; } else { __state = 26; continue; }
        }
        23 => {
            self.ir = self.irc;
            self.alu_sub(self.alub, 0x0000);
            self.sr_nzvc();
            self.icount -= 2;
            self.aob = self.pc;
            self.t = (self.isr & (SR_Z|SR_N)) != 0;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.aluo, 0xffff);
            self.sr_nzvc();
            if self.t != 0 { __state = 25; continue; } else { __state = 10; continue; }
        }
        24 => {
            self.t = self.isr & SR_N;
            self.alub = self.aluo;
            M68000::set_16l_32(&mut self.at, self.aluo);
            self.alu_sub(M68000::high16(self.at), 0x0000);
            self.icount -= 2;
            if self.t != 0 { __state = 26; continue; } else { __state = 27; continue; }
        }
        25 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.at;
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        26 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc + 2;
            __state = 10; continue;
        }
        27 => {
            self.aob = self.pc;
            self.ir = self.irc;
            M68000::set_16h(&mut self.at, self.aluo);
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nzvc();
            __state = 25; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sub_b_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub8(self.da[ry], self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_b_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_b_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = self.da[rx];
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and8(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_b_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_b_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_b_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub8(self.dbin, self.da[rx]);
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_sub_b_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and8(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_b_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and8(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_b_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.pc;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_b_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub8(self.dbin, self.da[rx]);
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_sub_b_imm8_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        M68000::set_16l_32(&mut self.dt, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dt);
        self.alu_sub8(self.dt, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_as_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = self.da[rx];
        self.at = self.da[ry];
        self.au = self.da[ry] + 2;
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.au = self.da[ry] - 2;
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_sub_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.pc;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_sub_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        M68000::set_16l_32(&mut self.dt, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dt);
        self.alu_sub(self.dt, self.da[rx]);
        self.sr_xnzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_sub_l_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_as_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au + 2;
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.au = self.da[ry] - 4;
        self.alu_and(self.dbin, 0xffff);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 4;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc + 4;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            self.sr_xnzvc();
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_xnzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sub_l_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.pc;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 4;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc + 4;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            self.sr_xnzvc();
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_xnzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sub_l_imm32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.dt, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        M68000::set_16l_32(&mut self.dt, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dt);
        self.alu_sub(self.dt, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.dt), M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_ds_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_as_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_ais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffff);
        self.dcr = 0;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_aips_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = 0;
        self.at = self.da[ry];
        self.au = self.da[ry] + 2;
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_pais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = 0;
        self.au = self.da[ry] - 2;
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_das_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and(self.dbin, 0xffff);
        self.dcr = 0;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_dais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = 0;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_suba_w_adr16_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = 0;
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_adr32_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = 0;
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_dpc_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.pc;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and(self.dbin, 0xffff);
        self.dcr = 0;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_w_dpci_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = 0;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_suba_w_imm16_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        M68000::set_16l_32(&mut self.dt, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dt);
        self.alu_sub(self.dt, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subx_b_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_subx8(self.da[ry], self.da[rx]);
        self.sr_xnzvc_u();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_subx_b_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = 0;
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.alu_and8(self.dbin, 0xffff);
        self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.at = self.au;
        self.da[rx] = self.au;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.pc;
        self.au = self.at;
        self.alu_subx8(self.alub, self.dbin);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_b_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_b_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = self.da[rx];
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and8(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_b_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_b_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_b_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(self.da[rx], self.dbin);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc + 2;
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sub_b_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and8(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_b_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and8(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub8(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8xl(&mut self.dbout, self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subx_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_subx(self.da[ry], self.da[rx]);
        self.sr_xnzvc_u();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_subx_w_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = 0;
        self.au = self.da[ry] - 2;
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.alu_and(self.dbin, 0xffff);
        self.au = self.da[rx] - 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.alub = self.dbin;
        self.at = self.au;
        self.da[rx] = self.au;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.pc;
        self.au = self.at;
        self.alu_subx(self.alub, self.dbin);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_w_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_w_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = self.da[rx];
        self.at = self.da[ry];
        self.au = self.da[ry] + 2;
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_w_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.au = self.da[ry] - 2;
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_w_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_w_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.da[rx], self.dbin);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc + 2;
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sub_w_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_w_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subx_l_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_subx(self.da[ry], self.da[rx]);
        self.sr_xnzvc_u();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
        self.sr_xnzvc_u();
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subx_l_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = 0;
        self.au = self.da[ry] - 2;
        self.icount -= 2;
        self.aob = self.au;
        self.alub = self.dbin;
        self.au = self.au - 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.alu_and(self.dbin, 0xffff);
        self.au = self.da[rx] - 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.au = self.au - 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.da[rx] = self.au;
        self.au = self.au + 2;
        self.alu_subx(self.aluo, self.dbin);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = self.aluo;
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.pc;
        self.au = self.at;
        self.alu_subc(self.alub, self.dbin);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_subc(M68000::high16(self.da[rx]), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au + 2;
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_subc(M68000::high16(self.da[rx]), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.au = self.da[ry] - 4;
        self.alu_and(self.dbin, 0xffff);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_subc(M68000::high16(self.da[rx]), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 4;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_subc(M68000::high16(self.da[rx]), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc + 4;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.da[rx], self.dbin);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au - 2;
            self.alu_subc(M68000::high16(self.da[rx]), self.alue);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc + 2;
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sub_l_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_subc(M68000::high16(self.da[rx]), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_sub_l_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_sub(self.da[rx], self.dbin);
        self.sr_xnzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au - 2;
        self.alu_subc(M68000::high16(self.da[rx]), self.alue);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_ds_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_as_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_ais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_aips_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry] + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au + 2;
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_pais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.au = self.da[ry] - 4;
        self.alu_and(self.dbin, 0xffff);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_das_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 4;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_dais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc + 4;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_suba_l_adr16_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = 0;
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_adr32_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = 0;
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 2;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_dpc_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.pc;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc + 4;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_subc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_suba_l_dpci_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc + 4;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16h(&mut self.da[rx], self.aluo);
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_suba_l_imm32_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        M68000::set_16h(&mut self.dt, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        M68000::set_16l_32(&mut self.dt, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dt);
        self.alu_sub(self.dt, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.alu_subc(M68000::high16(self.dt), M68000::high16(self.da[rx]));
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_16h(&mut self.da[rx], self.aluo);
        self.au = self.pc + 2;
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_cmp_b_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub8(self.da[ry], self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_b_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_b_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = self.da[rx];
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and8(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_b_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and8(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_b_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_b_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub8(self.dbin, self.da[rx]);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_cmp_b_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and8(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_b_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and8(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_b_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.pc;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and8(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub8(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_b_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub8(self.dbin, self.da[rx]);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_cmp_b_imm8_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        M68000::set_16l_32(&mut self.dt, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dt);
        self.alu_sub8(self.dt, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_as_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_sub(self.da[ry], self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = self.da[rx];
        self.at = self.da[ry];
        self.au = self.da[ry] + 2;
        self.da[ry] = self.au;
        self.au = self.pc + 2;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.au = self.da[ry] - 2;
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.da[ry];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.da[ry] + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

    pub fn handler_cmp_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        M68000::set_16h(&mut self.at, self.dbin);
        self.au = self.au + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = self.da[rx];
        M68000::set_16l_32(&mut self.at, self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc + 2;
        self.alu_and(self.dbin, 0xffff);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin) + self.pc;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc + 4;
        self.alu_and(self.dbin, 0xffff);
        self.dcr = self.da[rx];
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.dbin);
        self.alu_sub(self.dbin, self.da[rx]);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc + 2;
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
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

    pub fn handler_cmp_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = self.irc & 0x0800;
            self.au = self.pc + M68000::ext32(self.aluo);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.au = M68000::ext32(self.da[self.map_sp((self.irc >> 12) as usize)]) + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)] + self.au;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
            self.au = self.pc + 4;
            self.alu_and(self.dbin, 0xffff);
            self.dcr = self.da[rx];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, self.da[rx]);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
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
        }} // end match+loop
    }

}
