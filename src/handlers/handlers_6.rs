use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_assignments, unused_mut, unreachable_code)]
impl M68000 {
    pub fn handler_seq_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_Z;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dbeq_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_Z;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_seq_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_Z;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_seq_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = self.sr & SR_Z;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_seq_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = self.sr & SR_Z;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_seq_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_Z;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_seq_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_Z;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_seq_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = self.sr & SR_Z;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_seq_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = self.sr & SR_Z;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svc_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = !(self.sr & SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dbvc_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_V);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_svc_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = !(self.sr & SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svc_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = !(self.sr & SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svc_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = !(self.sr & SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svc_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = !(self.sr & SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svc_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = !(self.sr & SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_svc_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = !(self.sr & SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svc_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = !(self.sr & SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svs_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_V;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dbvs_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_V;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_svs_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_V;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svs_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = self.sr & SR_V;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svs_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = self.sr & SR_V;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svs_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_V;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svs_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_V;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_svs_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = self.sr & SR_V;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_svs_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = self.sr & SR_V;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_spl_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = !(self.sr & SR_N);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dbpl_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_N);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_spl_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = !(self.sr & SR_N);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_spl_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = !(self.sr & SR_N);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_spl_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = !(self.sr & SR_N);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_spl_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = !(self.sr & SR_N);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_spl_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = !(self.sr & SR_N);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_spl_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = !(self.sr & SR_N);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_spl_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = !(self.sr & SR_N);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_smi_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_N;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dbmi_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_N;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_smi_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_N;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_smi_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = self.sr & SR_N;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_smi_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = self.sr & SR_N;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_smi_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_N;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_smi_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = self.sr & SR_N;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_smi_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = self.sr & SR_N;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_smi_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = self.sr & SR_N;
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sge_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dbge_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sge_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sge_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sge_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sge_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sge_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_sge_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sge_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_slt_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dblt_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_slt_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_slt_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_slt_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_slt_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_slt_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_slt_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_slt_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sgt_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dbgt_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sgt_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sgt_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sgt_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sgt_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sgt_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_sgt_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sgt_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sle_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            M68000::set_8_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_dble_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.aob = self.au;
            self.alub = self.da[ry];
            self.at = self.au;
            self.au = self.pc + 2;
            self.alu_and(self.da[ry], 0xffff);
            self.t = self.isr & SR_Z;
            self.pc = self.au;
            self.au = self.at + 2;
            self.alu_add(self.alub, 0xffff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; continue; } else { __state = 3; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au + 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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
        4 => {
            self.aob = self.pc;
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_sle_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sle_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = self.dt;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sle_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = self.dt;
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(self.dt, self.dbin);
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
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sle_das(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_and8(self.dbin, 0xffff);
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sle_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 5; continue; } else { __state = 4; continue; }
        }
        4 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 6; continue;
        }
        6 => {
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

    pub fn handler_sle_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = self.dt;
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_sle_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = self.dt;
            M68000::set_16l_32(&mut self.at, self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(self.dt, self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.pc = self.au;
            self.alu_and8(0x0000, 0x0000);
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.pc = self.au;
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
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.at;
            self.alu_or8(self.aluo, 0xff);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            __state = 3; continue;
        }
        3 => {
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

    pub fn handler_bra_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = 1;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bra_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = 1;
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bsr_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        self.at = self.au;
        self.alu_and(self.au, 0xffff);
        self.au = self.da[self.sp as usize] - 4;
        self.icount -= 2;
        self.aob = self.au;
        self.dbout = M68000::high16(self.at);
        self.da[self.sp as usize] = self.au;
        self.au = self.au + 2;
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
        self.au = M68000::ext32(self.dbin) + self.pc;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
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

    pub fn handler_bsr_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        self.at = self.pc;
        self.au = self.da[self.sp as usize] - 4;
        self.alu_and(self.pc, 0xffff);
        self.icount -= 2;
        self.aob = self.au;
        self.dbout = M68000::high16(self.at);
        self.da[self.sp as usize] = self.au;
        self.au = self.au + 2;
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
        self.au = M68000::ext32(self.ftu) + self.pc;
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount -= 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
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

    pub fn handler_bhi_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = (self.sr & (SR_C|SR_Z)) == 0;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bhi_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = (self.sr & (SR_C|SR_Z)) == 0;
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bls_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = (self.sr & (SR_C|SR_Z)) != 0;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bls_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = (self.sr & (SR_C|SR_Z)) != 0;
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bcc_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_C);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bcc_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_C);
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bcs_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_C;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bcs_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_C;
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bne_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_Z);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bne_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_Z);
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_beq_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_Z;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_beq_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_Z;
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bvc_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_V);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bvc_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_V);
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bvs_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_V;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bvs_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_V;
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bpl_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_N);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bpl_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = !(self.sr & SR_N);
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bmi_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_N;
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bmi_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = self.sr & SR_N;
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bge_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bge_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V)) == 0);
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_blt_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_blt_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bgt_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_bgt_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = ((self.sr & (SR_N|SR_V|SR_Z)) == (SR_N|SR_V)) || ((self.sr & (SR_N|SR_V|SR_Z)) == 0);
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_ble_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.au = M68000::ext32(self.dbin) + self.pc;
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
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
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_ble_rel8(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.t = (self.sr & SR_Z) || ((self.sr & (SR_N|SR_V)) == SR_N) || ((self.sr & (SR_N|SR_V)) == SR_V);
            self.au = self.pc + M68000::ext32(self.ftu);
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        2 => {
            self.aob = self.au;
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
            __state = 3; continue;
        }
        3 => {
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
        _ => return,
        }} // end match+loop
    }

    pub fn handler_moveq_imm8o_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.da[rx] = M68000::ext32(self.ftu);
        self.au = self.au + 2;
        self.alu_and(self.ftu, 0xffff);
        self.sr_nzvc();
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

    pub fn handler_or_b_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_or8(self.da[ry], self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_b_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or8(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_b_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or8(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_b_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or8(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_b_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or8(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_b_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_or8(self.dbin, self.da[rx]);
            self.sr_nzvc();
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

    pub fn handler_or_b_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or8(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_b_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or8(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_b_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or8(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_b_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_or8(self.dbin, self.da[rx]);
            self.sr_nzvc();
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

    pub fn handler_or_b_imm8_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or8(self.dt, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_or(self.da[ry], self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_or(self.dbin, self.da[rx]);
            self.sr_nzvc();
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

    pub fn handler_or_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_or(self.dbin, self.da[rx]);
            self.sr_nzvc();
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

    pub fn handler_or_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dt, self.da[rx]);
        self.sr_nzvc();
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

    pub fn handler_or_l_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_or(self.da[ry], self.da[rx]);
        self.sr_nzvc();
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
        self.alu_or(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
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

    pub fn handler_or_l_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_or(self.alub, M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_or(self.alub, M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_or(self.alub, M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_or(self.alub, M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_or(self.dbin, self.da[rx]);
            self.sr_nzvc();
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_or(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_or(self.alub, M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.at = M68000::ext32(self.dbin);
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_or(self.alub, M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dbin, self.da[rx]);
        self.sr_nzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_or(self.alub, M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_or(self.dbin, self.da[rx]);
            self.sr_nzvc();
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_or(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_or_l_imm32_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_or(self.dt, self.da[rx]);
        self.sr_nzvc();
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
        self.alu_or(M68000::high16(self.dt), M68000::high16(self.da[rx]));
        self.sr_nzvc_u();
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

    pub fn handler_divu_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
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

    pub fn handler_divu_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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

    pub fn handler_divu_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
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

}
