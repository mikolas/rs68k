use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_mut, unreachable_code, unused_assignments)]
impl M68000 {
    pub fn partial_clr_w_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_w_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_and(((self.da[self.movems as usize]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_w_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_and(((self.da[self.movems as usize]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_l_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0x0000u16);
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.alu_and(M68000::high16(self.da[ry]), 0x0000u16);
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_l_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_and(self.alub, 0x0000u16);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_l_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_and(self.alub, 0x0000u16);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_l_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.au = self.da[ry].wrapping_sub(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.icount -= 2;
            self.aob = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_and(self.alub, 0x0000u16);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_l_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_and(self.alub, 0x0000u16);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_l_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 18; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 14;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.au = self.au.wrapping_add(2);
                self.alu_and(self.dbin, 0xffffu16);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_and(self.alub, 0x0000u16);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        16 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 18; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 14;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        18 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.au = self.au.wrapping_add(2);
                self.alu_and(self.dbin, 0xffffu16);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_l_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_and(((self.da[self.movems as usize]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_and(self.alub, 0x0000u16);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_clr_l_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_and(((self.da[self.movems as usize]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_and(self.alub, 0x0000u16);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 14;
            return;
        }
        14 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_b_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub8(((self.da[ry]) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_b_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_b_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_b_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_b_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_b_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_b_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_b_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_w_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub(((self.da[ry]) as u16), 0x0000u16);
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_w_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_w_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_w_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_w_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_w_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_w_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_w_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_l_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub(((self.da[ry]) as u16), 0x0000u16);
            self.sr_xnzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.alu_subc(M68000::high16(self.da[ry]), 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_l_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_subc(self.alub, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_l_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_subc(self.alub, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_l_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.au = self.da[ry].wrapping_sub(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.icount -= 2;
            self.aob = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_subc(self.alub, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_l_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_subc(self.alub, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_l_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 18; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 14;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.au = self.au.wrapping_add(2);
                self.alu_and(self.dbin, 0xffffu16);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_subc(self.alub, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        16 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 18; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 14;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        18 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.au = self.au.wrapping_add(2);
                self.alu_and(self.dbin, 0xffffu16);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_l_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_subc(self.alub, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_neg_l_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_sub(self.dbin, 0x0000u16);
            self.sr_xnzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_subc(self.alub, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 14;
            return;
        }
        14 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_ds_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.movemr = self.dbin;
            self.ftu = ((self.da[ry]) as u16);
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_ais_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_aips_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_pais_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_das_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_dais_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_adr16_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_adr32_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_dpc_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_dpci_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.pc.wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_imm8_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = ((self.dt) as u16);
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_b_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_not8(((self.da[ry]) as u8));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_b_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8x(((self.dbin) as u8), ((0xffu16) as u8));
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not8(((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_b_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8x(((self.alub) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not8(((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_b_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and8x(((self.alub) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not8(((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_b_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8x(((self.dbin) as u8), ((0xffu16) as u8));
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not8(((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_b_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8x(((self.dbin) as u8), ((0xffu16) as u8));
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not8(((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8x(((self.dbin) as u8), ((0xffu16) as u8));
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_b_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8x(((self.dbin) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not8(((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_b_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8x(((self.dbin) as u8), ((0xffu16) as u8));
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not8(((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_w_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_not(((self.da[ry]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_w_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_andx(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_w_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_andx(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_w_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_w_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_andx(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_w_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_andx(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_andx(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_w_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_w_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_l_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_not(((self.da[ry]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.alu_not(M68000::high16(self.da[ry]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[ry], v); }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_l_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_andx(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_not(self.alub);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_l_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_andx(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_not(self.alub);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_l_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.au = self.da[ry].wrapping_sub(4);
            self.alu_andx(self.dbin, 0xffffu16);
            self.icount -= 2;
            self.aob = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_not(self.alub);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_l_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_andx(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_not(self.alub);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_l_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 18; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 14;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.au = self.au.wrapping_add(2);
                self.alu_andx(self.dbin, 0xffffu16);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_not(self.alub);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        16 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 18; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 14;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        18 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.au = self.au.wrapping_add(2);
                self.alu_andx(self.dbin, 0xffffu16);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_l_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_andx(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_not(self.alub);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_not_l_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_andx(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_not(self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_not(self.alub);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 12;
            return;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 14;
            return;
        }
        14 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_ds_sr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.movemr = self.dbin;
            self.ftu = ((self.da[ry]) as u16);
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_ais_sr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_andx(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_aips_sr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_andx(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_pais_sr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_das_sr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_andx(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_dais_sr(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_andx(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_andx(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_adr16_sr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_adr32_sr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_dpc_sr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_andx(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_dpci_sr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.pc.wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_andx(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_andx(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_i16u_sr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.movemr = self.dbin;
            self.ftu = ((self.dt) as u16);
            self.au = self.au.wrapping_sub(2);
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_nbcd_b_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sbcd8(((self.da[ry]) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_nbcd_b_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sbcd8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_nbcd_b_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sbcd8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_nbcd_b_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sbcd8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_nbcd_b_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sbcd8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_nbcd_b_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sbcd8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_nbcd_b_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sbcd8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_nbcd_b_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sbcd8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 10;
            return;
        }
        10 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_swap_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.alub = ((self.da[ry]) as u16);
            self.pc = self.au;
            { let v = ((self.da[ry]) as u16); M68000::set_16h(&mut self.at, v); }
            self.alu_and(M68000::high16(self.da[ry]), 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[ry] = M68000::merge_16_32(M68000::high16(self.at), self.aluo);
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_pea_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = self.da[ry];
            self.au = self.da[self.sp as usize].wrapping_sub(4);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.at);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_pea_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.at = self.da[ry];
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.at);
            self.at = self.au;
            self.alu_and(((self.au) as u16), 0xffffu16);
            self.au = self.da[self.sp as usize].wrapping_sub(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.at);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_pea_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.ir = self.irc;
                self.at = self.au;
                self.alu_and(((self.au) as u16), 0xffffu16);
                self.au = self.pc.wrapping_add(4);
                self.icount -= 2;
                self.aob = self.au;
                self.pc = self.au;
                self.au = self.da[self.sp as usize].wrapping_sub(4);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.at);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.ir = self.irc;
                self.at = self.au;
                self.alu_and(((self.au) as u16), 0xffffu16);
                self.au = self.pc.wrapping_add(4);
                self.icount -= 2;
                self.aob = self.au;
                self.pc = self.au;
                self.au = self.da[self.sp as usize].wrapping_sub(4);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_pea_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.au = self.da[self.sp as usize].wrapping_sub(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.at);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = ((self.at) as u16);
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_pea_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = self.da[self.sp as usize].wrapping_sub(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.at);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = ((self.at) as u16);
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_pea_dpc(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.at = self.pc;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.at);
            self.at = self.au;
            self.alu_and(((self.au) as u16), 0xffffu16);
            self.au = self.da[self.sp as usize].wrapping_sub(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.at);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_pea_dpci(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.pc.wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.ir = self.irc;
                self.at = self.au;
                self.alu_and(((self.au) as u16), 0xffffu16);
                self.au = self.pc.wrapping_add(4);
                self.icount -= 2;
                self.aob = self.au;
                self.pc = self.au;
                self.au = self.da[self.sp as usize].wrapping_sub(4);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = M68000::high16(self.at);
            self.da[self.sp as usize] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.ir = self.irc;
                self.at = self.au;
                self.alu_and(((self.au) as u16), 0xffffu16);
                self.au = self.pc.wrapping_add(4);
                self.icount -= 2;
                self.aob = self.au;
                self.pc = self.au;
                self.au = self.da[self.sp as usize].wrapping_sub(4);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_ext_w_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_ext(((self.da[ry]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_list_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.da[ry];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 8; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 10; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        8 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 10; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 8; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_listp_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_sub(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem_predec();
                if self.t != 0 { self.inst_substate = 8; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.at = self.au;
                self.au = self.au.wrapping_sub(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem_predec();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 10; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.da[ry] = self.at;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        8 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem_predec();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 10; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.da[ry] = self.at;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem_predec();
                if self.t != 0 { self.inst_substate = 8; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.at = self.au;
                self.au = self.au.wrapping_sub(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_list_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = M68000::ext32(self.dbin);
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.dbin));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_list_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.pc = self.au;
                { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
                self.au = self.au.wrapping_sub(2);
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 16; } else { self.inst_substate = 14; } break;
            }
            1 => {
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 16; } else { self.inst_substate = 14; } break;
            }
            1 => {
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        16 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_list_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = M68000::ext32(self.dbin);
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_list_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_ext_l_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[ry] = self.at;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_l_list_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.da[ry];
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.aob = self.au;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.dbout = ((self.da[self.movems as usize]) as u16);
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_l_listp_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_sub(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem_predec();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.at = self.au;
                self.au = self.au.wrapping_sub(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.aob = self.au;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.dbout = M68000::high16(self.da[self.movems as usize]);
            self.at = self.au;
            self.au = self.au.wrapping_sub(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem_predec();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.da[ry] = self.at;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem_predec();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.da[ry] = self.at;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem_predec();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = ((self.da[self.movems as usize]) as u16);
                self.at = self.au;
                self.au = self.au.wrapping_sub(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_l_list_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = M68000::ext32(self.dbin);
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.da[ry].wrapping_add(M68000::ext32(self.dbin));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.aob = self.au;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.dbout = ((self.da[self.movems as usize]) as u16);
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_l_list_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(4);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.pc = self.au;
                { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
                self.au = self.au.wrapping_sub(2);
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 18; } else { self.inst_substate = 16; } break;
            }
            1 => {
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 12;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 18; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.aob = self.au;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.dbout = ((self.da[self.movems as usize]) as u16);
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 16; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 18; } else { self.inst_substate = 16; } break;
            }
            1 => {
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 12;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        16 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 18; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        18 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 16; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_l_list_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = M68000::ext32(self.dbin);
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.aob = self.au;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.dbout = ((self.da[self.movems as usize]) as u16);
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_l_list_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.aob = self.au;
            self.t = if self.movemr == 0 { 1 } else { 0 };
            self.dbout = ((self.da[self.movems as usize]) as u16);
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 16; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 12;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 16; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 12;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        16 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.dbout = M68000::high16(self.da[self.movems as usize]);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA;
                bus.write_data(self.aob & !1u32, self.dbout);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_b_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = self.da[ry];
            self.au = self.au.wrapping_add(2);
            self.alu_and8(((self.da[ry]) as u16), 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_b_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_b_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(((self.da[self.movems as usize]) as u8), ((self.dbin) as u8));
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_b_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(((self.da[self.movems as usize]) as u8), ((self.dbin) as u8));
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_b_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_b_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 12; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 12; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_b_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(((self.da[self.movems as usize]) as u8), ((self.dbin) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_b_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(((self.da[self.movems as usize]) as u8), ((self.dbin) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_w_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = self.da[ry];
            self.au = self.au.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_w_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_w_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_or(((self.da[self.movems as usize]) as u16), self.dbin);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_w_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.au = self.da[ry].wrapping_sub(2);
            self.alu_or(((self.da[self.movems as usize]) as u16), self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_w_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_w_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 12; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 12; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_w_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or(((self.da[self.movems as usize]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_w_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or(((self.da[self.movems as usize]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_l_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = self.da[ry];
            self.au = self.au.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(M68000::high16(self.da[ry]), 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_l_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_l_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_l_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.au = self.da[ry].wrapping_sub(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.icount -= 2;
            self.aob = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_l_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_l_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.au = self.au.wrapping_add(2);
                self.alu_and(self.dbin, 0xffffu16);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.au = self.au.wrapping_add(2);
                self.alu_and(self.dbin, 0xffffu16);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_l_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or(((self.da[self.movems as usize]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tst_l_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or(((self.da[self.movems as usize]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nz_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tas_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.alub = ((self.da[ry]) as u16);
            self.pc = self.au;
            self.alu_or8(((self.ftu) as u8), ((self.da[ry]) as u8));
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.au.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tas_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(((self.ftu) as u8), ((self.dbin) as u8));
            self.icount -= 2;
            self.aob = self.au;
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc;
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tas_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_or8(((self.da[self.movems as usize]) as u8), ((self.dbin) as u8));
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(((self.ftu) as u8), ((self.dbin) as u8));
            self.icount -= 2;
            self.aob = self.au;
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc;
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tas_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_or8(((self.da[self.movems as usize]) as u8), ((self.dbin) as u8));
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
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(((self.ftu) as u8), ((self.dbin) as u8));
            self.icount -= 2;
            self.aob = self.au;
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc;
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tas_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(((self.ftu) as u8), ((self.dbin) as u8));
            self.icount -= 2;
            self.aob = self.au;
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc;
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tas_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            self.dbin = self.edb;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(((self.ftu) as u8), ((self.dbin) as u8));
            self.icount -= 2;
            self.aob = self.au;
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc;
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.da[self.movems as usize]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tas_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(((self.da[self.movems as usize]) as u8), ((self.dbin) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(((self.ftu) as u8), ((self.dbin) as u8));
            self.icount -= 2;
            self.aob = self.au;
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc;
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_tas_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[self.movems as usize]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(((self.da[self.movems as usize]) as u8), ((self.dbin) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            self.dbin = self.edb;
            self.pc = self.au;
            self.au = self.at;
            self.alu_or8(((self.ftu) as u8), ((self.dbin) as u8));
            self.icount -= 2;
            self.aob = self.au;
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc;
            self.alu_and8(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 10;
            return;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_ais_list(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_aips_list(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.da[ry] = self.at;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        10 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 12; break; }
            }
            1 => {
                self.aob = self.pc;
                self.da[ry] = self.at;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_das_list(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_dais_list(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.au = self.da[ry].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 16; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 12;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                __state = 1; continue;
            }
            1 => {
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
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 18; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 20; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 16; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 12;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        16 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                __state = 1; continue;
            }
            1 => {
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
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        18 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 20; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        20 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 18; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_adr16_list(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_adr32_list(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 8;
            return;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 16; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 12;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        12 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 16; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 12;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        16 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_movem_w_dpc_list(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
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
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        8 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        10 => {
            if self.aob & 1 != 0 {
                self.icount -= 4;
                self.inst_state = S_ADDRESS_ERROR;
                self.inst_substate = 0;
                return;
            }
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { __state = 1; continue; } else { self.inst_substate = 14; break; }
            }
            1 => {
                self.aob = self.pc;
                self.ir = self.irc;
                self.au = self.pc.wrapping_add(2);
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 10;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.step_movem();
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.aob = self.au;
                self.t = if self.movemr == 0 { 1 } else { 0 };
                self.at = self.au;
                self.da[self.movems as usize] = M68000::ext32(self.dbin);
                self.au = self.au.wrapping_add(2);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 8;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

}
