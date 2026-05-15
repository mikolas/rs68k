use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_mut, unreachable_code, unused_assignments)]
impl M68000 {
    pub fn handler_movem_w_dpci_list(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.aob = self.au;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            self.da[self.movems as usize] = M68000::ext32(self.dbin);
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
            self.step_movem();
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        5 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_movem_l_ais_list(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
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
            self.aob = self.da[ry];
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.da[self.movems as usize], v); }
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
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.da[self.movems as usize], v); }
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
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        2 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_movem_l_aips_list(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
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
            self.aob = self.da[ry];
            self.ir = self.irc;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.da[self.movems as usize], v); }
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
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.da[self.movems as usize], v); }
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
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        2 => {
            self.aob = self.pc;
            self.da[ry] = self.at;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_movem_l_das_list(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.aob = self.at;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.da[self.movems as usize], v); }
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
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.da[self.movems as usize], v); }
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
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        2 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_movem_l_dais_list(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.da[self.movems as usize], v); }
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
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.da[self.movems as usize], v); }
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
            self.step_movem();
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        5 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_movem_l_adr16_list(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.dt = M68000::ext32(self.dbin);
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
            self.aob = self.dt;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.at = self.dt;
            self.au = self.dt.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.da[self.movems as usize], v); }
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
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.da[self.movems as usize], v); }
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
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        2 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_movem_l_adr32_list(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.aob = self.dt;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.at = self.dt;
            self.au = self.dt.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.da[self.movems as usize], v); }
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
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.da[self.movems as usize], v); }
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
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        2 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_movem_l_dpc_list(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.aob = self.at;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.da[self.movems as usize], v); }
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
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.da[self.movems as usize], v); }
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
            self.step_movem();
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        2 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_movem_l_dpci_list(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.dbin = self.edb;
            self.step_movem();
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.da[self.movems as usize], v); }
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
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.at = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.da[self.movems as usize], v); }
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
            self.step_movem();
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        5 => {
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
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

    pub fn handler_trap_imm4(&mut self, bus: &mut impl crate::types::Bus) {
        self.ftu = self.sr;
        self.au = self.pc.wrapping_sub(2);
        self.icount -= 2;
        self.sr = (self.sr & !SR_T) | SR_S;
        self.update_user_super();
        self.update_interrupt();
        self.au = self.da[16].wrapping_sub(2);
        self.alu_and(((self.pc) as u16), 0xffffu16);
        self.icount -= 2;
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = self.au.wrapping_sub(4);
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_DATA;
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
        self.ftu = (0x80u16 | ((self.ird & 0xf) << 2));
        self.au = self.au.wrapping_add(2);
        self.alu_and(M68000::high16(self.pc), 0xffffu16);
        self.base_ssw = SSW_DATA;
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
        self.base_ssw = SSW_DATA;
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
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
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
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
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
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_link_as_imm16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.at = self.da[ry];
        self.au = self.au.wrapping_add(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.pc = self.au;
        self.au = self.da[self.sp as usize].wrapping_sub(4);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = M68000::high16(self.at);
        self.da[ry] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.dbout = self.aluo;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[self.sp as usize] = self.au;
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

    pub fn handler_unlk_as(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.pc = self.au;
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.step_movem();
        self.aob = self.au;
        self.ir = self.irc;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
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
        self.aob = self.pc;
        self.da[self.sp as usize] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[ry] = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_move_as_usp(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
        self.aob = self.au;
        self.ir = self.irc;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.at = self.da[ry];
        self.ftu = self.dbin;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[15] = self.at;
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

    pub fn handler_move_usp_as(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dcr = ((self.da[15]) as u8);
        self.at = self.da[15];
        self.au = self.da[ry];
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[ry] = self.at;
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

    pub fn handler_reset(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0x0000u16);
            self.icount -= 2;
            self.ir = self.irc;
            self.au = M68000::ext32(self.aluo).wrapping_sub(2);
            self.icount -= 2;
            __state = 1; continue;
        }
        1 => {
            self.au = self.au.wrapping_sub(2);
            self.icount -= 2;
            self.t = if self.au & 0x3f == 0 { 1 } else { 0 };
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        2 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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
        _ => return,
        }}
    }

    pub fn handler_nop(&mut self, bus: &mut impl crate::types::Bus) {
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

    pub fn handler_stop_i16u(&mut self, bus: &mut impl crate::types::Bus) {
        if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0x0000u16);
        self.icount -= 2;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.pc = self.au;
        self.isr = (self.ftu) & (SR_CCR|SR_SR); self.sr = self.isr;
        self.update_user_super();
        self.update_interrupt();
        self.au = self.au.wrapping_sub(2);
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        return;
    }

    pub fn handler_rte(&mut self, bus: &mut impl crate::types::Bus) {
        if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
        self.aob = self.da[self.sp as usize];
        self.au = self.da[self.sp as usize].wrapping_add(2);
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
        self.movemr = self.dbin;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = M68000::ext32(self.dbin);
        self.au = self.au.wrapping_add(2);
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.da[self.sp as usize] = self.au;
        self.isr = (self.ftu) & (SR_CCR|SR_SR); self.new_sr = self.isr;
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.sr = self.new_sr;
        self.update_user_super();
        self.update_interrupt();
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
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

    pub fn handler_rts(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.da[self.sp as usize];
        self.au = self.da[self.sp as usize].wrapping_add(2);
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
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
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
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.da[self.sp as usize] = self.au;
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
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

    pub fn handler_trapv(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = (self.isr & SR_V) as i32;
            self.alub = self.dbin;
            self.ftu = self.sr;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.pc = self.au;
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
        2 => {
            self.sr = (self.sr & !SR_T) | SR_S;
            self.update_user_super();
            self.update_interrupt();
            self.au = self.da[16].wrapping_sub(2);
            self.alu_and(((self.pc) as u16), 0xffffu16);
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.ftu, 0xffffu16);
            self.base_ssw = SSW_DATA;
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
            self.ftu = 0x001cu16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
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
            self.base_ssw = SSW_DATA;
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
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
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
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_rtr(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.da[self.sp as usize];
        self.au = self.da[self.sp as usize].wrapping_add(2);
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
        self.movemr = self.dbin;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = M68000::ext32(self.dbin);
        self.au = self.au.wrapping_add(2);
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.da[self.sp as usize] = self.au;
        self.isr = (self.ftu) & SR_CCR | (self.sr & SR_SR); self.sr = self.isr;
        self.base_ssw = SSW_DATA | SSW_R;
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
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
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

    pub fn handler_jsr_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.au = self.da[self.sp as usize].wrapping_sub(4);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.aob = self.au;
        self.dbout = M68000::high16(self.pc);
        self.da[self.sp as usize] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = ((self.pc) as u16);
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
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

    pub fn handler_jsr_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.pc = self.au;
        self.au = self.da[self.sp as usize].wrapping_sub(4);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.aob = self.au;
        self.dbout = M68000::high16(self.pc);
        self.da[self.sp as usize] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = ((self.pc) as u16);
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
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

    pub fn handler_jsr_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.t = (self.irc & 0x0800) as i32;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[ry]);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = self.au.wrapping_add(M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)));
            self.icount -= 2;
            __state = 3; continue;
        }
        2 => {
            self.au = self.au.wrapping_add(self.da[self.map_sp((self.irc >> 12) as usize)]);
            self.icount -= 2;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.pc = self.au;
            self.au = self.da[self.sp as usize].wrapping_sub(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.pc);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.ir = self.irc;
            self.dbout = ((self.pc) as u16);
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
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
        _ => return,
        }}
    }

    pub fn handler_jsr_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.at = M68000::ext32(self.dbin);
        self.icount -= 2;
        self.aob = self.at;
        self.pc = self.au;
        self.au = self.da[self.sp as usize].wrapping_sub(4);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.aob = self.au;
        self.dbout = M68000::high16(self.pc);
        self.da[self.sp as usize] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = ((self.pc) as u16);
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
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

    pub fn handler_jsr_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = self.da[self.sp as usize].wrapping_sub(4);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.aob = self.au;
        self.dbout = M68000::high16(self.pc);
        self.da[self.sp as usize] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = ((self.pc) as u16);
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
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

    pub fn handler_jsr_dpc(&mut self, bus: &mut impl crate::types::Bus) {
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.pc = self.au;
        self.au = self.da[self.sp as usize].wrapping_sub(4);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
        self.aob = self.au;
        self.dbout = M68000::high16(self.pc);
        self.da[self.sp as usize] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = ((self.pc) as u16);
        self.au = self.at.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
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

    pub fn handler_jsr_dpci(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.t = (self.irc & 0x0800) as i32;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = self.au.wrapping_add(M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)));
            self.icount -= 2;
            __state = 3; continue;
        }
        2 => {
            self.au = self.au.wrapping_add(self.da[self.map_sp((self.irc >> 12) as usize)]);
            self.icount -= 2;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.pc = self.au;
            self.au = self.da[self.sp as usize].wrapping_sub(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.pc);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.aob = self.au;
            self.ir = self.irc;
            self.dbout = ((self.pc) as u16);
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
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
        _ => return,
        }}
    }

    pub fn handler_jmp_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
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

    pub fn handler_jmp_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.icount -= 2;
        self.aob = self.au;
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

    pub fn handler_jmp_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.t = (self.irc & 0x0800) as i32;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[ry]);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = self.au.wrapping_add(M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)));
            self.icount -= 2;
            __state = 3; continue;
        }
        2 => {
            self.au = self.au.wrapping_add(self.da[self.map_sp((self.irc >> 12) as usize)]);
            self.icount -= 2;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
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
        _ => return,
        }}
    }

    pub fn handler_jmp_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.at = M68000::ext32(self.dbin);
        self.icount -= 2;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
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

    pub fn handler_jmp_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        if self.aob & 1 != 0 {
            self.icount += 4;
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.irc = self.edb;
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

    pub fn handler_jmp_dpc(&mut self, bus: &mut impl crate::types::Bus) {
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.icount -= 2;
        self.aob = self.au;
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

    pub fn handler_jmp_dpci(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.t = (self.irc & 0x0800) as i32;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = self.au.wrapping_add(M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)));
            self.icount -= 2;
            __state = 3; continue;
        }
        2 => {
            self.au = self.au.wrapping_add(self.da[self.map_sp((self.irc >> 12) as usize)]);
            self.icount -= 2;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
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
        _ => return,
        }}
    }

    pub fn handler_addq_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_add8(((self.ftu) as u8), ((self.da[ry]) as u8));
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

    pub fn handler_addq_b_imm3_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addq_b_imm3_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addq_b_imm3_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addq_b_imm3_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addq_b_imm3_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dt = M68000::ext32(self.ftu);
            self.au = self.at;
            self.alu_add8(((self.ftu) as u8), ((self.dbin) as u8));
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
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_addq_b_imm3_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addq_b_imm3_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_addq_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_add(self.ftu, ((self.da[ry]) as u16));
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

    pub fn handler_addq_w_imm3_as(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_add(self.ftu, ((self.da[ry]) as u16));
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

    pub fn handler_addq_w_imm3_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_w_imm3_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_w_imm3_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_w_imm3_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_w_imm3_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dt = M68000::ext32(self.ftu);
            self.au = self.at;
            self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_w_imm3_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_w_imm3_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_add(self.ftu, ((self.da[ry]) as u16));
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

    pub fn handler_addq_l_imm3_as(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_add(self.ftu, ((self.da[ry]) as u16));
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

    pub fn handler_addq_l_imm3_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_l_imm3_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_l_imm3_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_l_imm3_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_l_imm3_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dt = M68000::ext32(self.ftu);
            self.au = self.at;
            self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_l_imm3_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_addq_l_imm3_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_add(self.ftu, self.dbin);
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

    pub fn handler_st_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = 1;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
        2 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_dbt_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.t = 1;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            self.alub = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.t = (self.isr & SR_Z) as i32;
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.alu_add(self.alub, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; } else { __state = 3; } continue;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
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
        4 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_st_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 1;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_st_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 1;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_st_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 1;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_st_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 1;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_st_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = 1;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_st_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 1;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_st_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 1;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_subq_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_sub8(((self.ftu) as u8), ((self.da[ry]) as u8));
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

    pub fn handler_subq_b_imm3_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subq_b_imm3_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subq_b_imm3_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subq_b_imm3_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dcr = ((self.dt) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subq_b_imm3_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dt = M68000::ext32(self.ftu);
            self.au = self.at;
            self.alu_sub8(((self.ftu) as u8), ((self.dbin) as u8));
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
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_subq_b_imm3_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subq_b_imm3_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub8(((self.ftu) as u8), ((self.dbin) as u8));
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
        if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
        else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_subq_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_sub(self.ftu, ((self.da[ry]) as u16));
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

    pub fn handler_subq_w_imm3_as(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_sub(self.ftu, ((self.da[ry]) as u16));
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

    pub fn handler_subq_w_imm3_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_w_imm3_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_w_imm3_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_w_imm3_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_w_imm3_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dt = M68000::ext32(self.ftu);
            self.au = self.at;
            self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_w_imm3_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_w_imm3_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_sub(self.ftu, ((self.da[ry]) as u16));
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

    pub fn handler_subq_l_imm3_as(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.dt = M68000::ext32(self.ftu);
        self.alu_sub(self.ftu, ((self.da[ry]) as u16));
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

    pub fn handler_subq_l_imm3_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_l_imm3_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_l_imm3_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_l_imm3_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_l_imm3_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dt = M68000::ext32(self.ftu);
            self.au = self.at;
            self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_l_imm3_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_subq_l_imm3_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.dt = M68000::ext32(self.ftu);
        self.au = self.at;
        self.alu_sub(self.ftu, self.dbin);
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

    pub fn handler_sf_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = 0;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
        2 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_dbra_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.t = 0;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            self.alub = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.t = (self.isr & SR_Z) as i32;
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.alu_add(self.alub, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; } else { __state = 3; } continue;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
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
        4 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_sf_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 0;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sf_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 0;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sf_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 0;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sf_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 0;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sf_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = 0;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sf_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 0;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sf_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = 0;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_shi_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
        2 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_dbhi_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            self.alub = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.t = (self.isr & SR_Z) as i32;
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.alu_add(self.alub, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; } else { __state = 3; } continue;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
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
        4 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_shi_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_shi_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_shi_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_shi_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_shi_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_shi_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_shi_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sls_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
        2 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_dbls_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            self.alub = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.t = (self.isr & SR_Z) as i32;
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.alu_add(self.alub, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; } else { __state = 3; } continue;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
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
        4 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_sls_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sls_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sls_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sls_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sls_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sls_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sls_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if (self.sr & (SR_C|SR_Z)) != 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scc_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
        2 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_dbcc_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            self.alub = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.t = (self.isr & SR_Z) as i32;
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.alu_add(self.alub, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; } else { __state = 3; } continue;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
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
        4 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_scc_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scc_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scc_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scc_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scc_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scc_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scc_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_C == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scs_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = (self.sr & SR_C) as i32;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
        2 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_dbcs_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.t = (self.sr & SR_C) as i32;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            self.alub = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.t = (self.isr & SR_Z) as i32;
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.alu_add(self.alub, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; } else { __state = 3; } continue;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
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
        4 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_scs_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = (self.sr & SR_C) as i32;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scs_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = (self.sr & SR_C) as i32;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scs_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = (self.sr & SR_C) as i32;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scs_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = (self.sr & SR_C) as i32;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scs_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = (self.sr & SR_C) as i32;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scs_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = (self.sr & SR_C) as i32;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_scs_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = (self.sr & SR_C) as i32;
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sne_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
        2 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_dbne_ds_rel16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.aob = self.au;
            self.alub = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.t = (self.isr & SR_Z) as i32;
            self.pc = self.au;
            self.au = self.at.wrapping_add(2);
            self.alu_add(self.alub, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.irc = self.edb;
            if self.t != 0 { __state = 4; } else { __state = 3; } continue;
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
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
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
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
        4 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_sne_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sne_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sne_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sne_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sne_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            __state = 6; continue;
        }
        5 => {
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sne_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

    pub fn handler_sne_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
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
            self.t = if self.sr & SR_Z == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.alu_and8(0x0000u16, 0x0000u16);
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
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
            self.au = self.at;
            self.alu_or8(((self.aluo) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            if self.aob & 1 != 0 {
                self.icount += 4;
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
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }}
    }

}
