use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_mut, unreachable_code, unused_assignments)]
impl M68000 {
    pub fn partial_btst_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_btst_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.dt) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
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
            self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_btst_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.inst_substate = 4;
            return;
        }
        4 => {
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

    pub fn partial_btst_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_btst_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
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
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
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
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
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

    pub fn partial_btst_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_btst_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
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

    pub fn partial_btst_imm8_dpc(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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
            self.alu_and8(self.dbin, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_btst_imm8_dpci(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
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
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
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
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
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

    pub fn partial_bchg_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.alu_eor(((self.da[ry]) as u16), (1u16 << (self.dcr & 15)));
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
                self.inst_substate = 0;
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
                self.inst_substate = 0;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_bchg_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bchg_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.dt) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bchg_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.inst_substate = 4;
            return;
        }
        4 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bchg_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bchg_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
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

    pub fn partial_bchg_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bchg_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 12;
            return;
        }
        12 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_bclr_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = self.dt;
            self.au = self.da[ry];
            self.alu_or(((self.dt) as u16), ((self.da[ry]) as u16));
            self.t = (self.dcr & 0x10) as i32;
            self.alub = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_or(((self.da[ry]) as u16), (1u16 << (self.dcr & 15)));
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
                self.inst_substate = 0;
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
                self.inst_substate = 0;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_bclr_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alub = self.dbin;
            self.ftu = self.sr;
            self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bclr_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bclr_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bclr_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alub = self.dbin;
            self.ftu = self.sr;
            self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bclr_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
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

    pub fn partial_bclr_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.alub = self.dbin;
            self.ftu = self.sr;
            self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_eor8(((self.aluo) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bclr_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 12;
            return;
        }
        12 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_bset_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = self.dt;
            self.au = self.da[ry];
            self.alu_or(((self.dt) as u16), ((self.da[ry]) as u16));
            self.t = (self.dcr & 0x10) as i32;
            self.alub = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_or(((self.da[ry]) as u16), (1u16 << (self.dcr & 15)));
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
                self.inst_substate = 0;
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
                self.inst_substate = 0;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_bset_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bset_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bset_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bset_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bset_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
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

    pub fn partial_bset_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_or8(((self.dbin) as u8), (((1u16 << (self.dcr & 7))) as u8));
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
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

    pub fn partial_bset_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_or8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.alu_and8(self.alub, (1u16 << (self.dcr & 7)));
            self.sr_z();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 12;
            return;
        }
        12 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_eori_b_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_eor8(((self.dt) as u8), ((self.da[ry]) as u8));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_eori_b_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_eori_b_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.dt) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
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
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_eori_b_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_eori_b_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_eori_b_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
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

    pub fn partial_eori_b_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_eori_b_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.dt) as u8), ((self.dbin) as u8));
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
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 12;
            return;
        }
        12 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_eori_imm8_ccr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.ftu = self.sr;
            self.au = self.pc.wrapping_sub(2);
            self.icount -= 2;
            self.alu_eor8(((self.ftu) as u8), ((self.dt) as u8));
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

    pub fn partial_eori_w_imm16_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_eor(((self.dt) as u16), ((self.da[ry]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_eori_w_imm16_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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

    pub fn partial_eori_w_imm16_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.dt) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_eor(((self.dt) as u16), self.dbin);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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

    pub fn partial_eori_w_imm16_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.au = self.da[ry].wrapping_sub(2);
            self.alu_eor(((self.dt) as u16), self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and(self.alub, 0xffffu16);
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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

    pub fn partial_eori_w_imm16_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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

    pub fn partial_eori_w_imm16_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor(((self.dt) as u16), self.dbin);
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and(self.dbin, 0xffffu16);
                self.dcr = ((self.dt) as u8);
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

    pub fn partial_eori_w_imm16_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_eor(((self.dt) as u16), self.dbin);
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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

    pub fn partial_eori_w_imm16_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_eor(((self.dt) as u16), self.dbin);
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
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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

    pub fn partial_eori_i16u_sr(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }
            self.aob = self.au;
            self.pc = self.au;
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
            self.ftu = self.sr;
            self.au = self.pc.wrapping_sub(2);
            self.icount -= 2;
            self.alu_eor(self.ftu, ((self.dt) as u16));
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

    pub fn partial_eori_l_imm32_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.alu_eor(((self.dt) as u16), ((self.da[ry]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
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
            self.alu_eor(M68000::high16(self.dt), M68000::high16(self.da[ry]));
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
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_eori_l_imm32_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.dt), self.alue);
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

    pub fn partial_eori_l_imm32_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
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
            self.au = self.au.wrapping_add(2);
            self.da[ry] = self.au;
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.dt), self.alue);
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

    pub fn partial_eori_l_imm32_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.au = self.da[ry].wrapping_sub(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.icount -= 2;
            self.aob = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
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
            self.alu_eor(((self.dt) as u16), self.dbin);
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
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.dt), self.alue);
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

    pub fn partial_eori_l_imm32_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
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
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
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
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 16;
            return;
        }
        16 => {
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

    pub fn partial_eori_l_imm32_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
                if self.t != 0 { self.inst_substate = 20; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 22; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 18;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        18 => {
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
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
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 16;
            return;
        }
        16 => {
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
        20 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 22; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 18;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        22 => {
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

    pub fn partial_eori_l_imm32_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_eor(((self.dt) as u16), self.dbin);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
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
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
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
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 16;
            return;
        }
        16 => {
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

    pub fn partial_eori_l_imm32_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.alu_eor(((self.dt) as u16), self.dbin);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.inst_substate = 16;
            return;
        }
        16 => {
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
            self.inst_substate = 18;
            return;
        }
        18 => {
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

    pub fn partial_cmpi_b_imm8_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sub8(((self.dt) as u8), ((self.da[ry]) as u8));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_cmpi_b_imm8_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_cmpi_b_imm8_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_cmpi_b_imm8_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_cmpi_b_imm8_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_cmpi_b_imm8_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
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
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        6 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
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
                self.dcr = ((self.dt) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.edb = bus.read_data(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
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

    pub fn partial_cmpi_b_imm8_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
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
            self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
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

    pub fn partial_cmpi_b_imm8_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
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
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 8;
            return;
        }
        8 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub8(((self.dt) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
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

    pub fn partial_cmpi_w_imm16_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_sub(((self.dt) as u16), ((self.da[ry]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_cmpi_w_imm16_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_sub(((self.dt) as u16), self.dbin);
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

    pub fn partial_cmpi_w_imm16_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.alu_sub(((self.dt) as u16), self.dbin);
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

    pub fn partial_cmpi_w_imm16_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.alu_sub(((self.dt) as u16), self.dbin);
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

    pub fn partial_cmpi_w_imm16_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.dt) as u8);
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
            self.alu_sub(((self.dt) as u16), self.dbin);
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

    pub fn partial_cmpi_w_imm16_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
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
                self.dcr = ((self.dt) as u8);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub(((self.dt) as u16), self.dbin);
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
                self.dcr = ((self.dt) as u8);
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

    pub fn partial_cmpi_w_imm16_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
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
            self.alu_sub(((self.dt) as u16), self.dbin);
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

    pub fn partial_cmpi_w_imm16_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
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
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
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
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
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

    pub fn partial_cmpi_l_imm32_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.alu_sub(((self.dt) as u16), ((self.da[ry]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.dt), M68000::high16(self.da[ry]));
            self.sr_nzvc_u();
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
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

    pub fn partial_cmpi_l_imm32_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
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
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_subc(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
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

    pub fn partial_cmpi_l_imm32_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
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
            self.au = self.au.wrapping_add(2);
            self.da[ry] = self.au;
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
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_subc(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
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

    pub fn partial_cmpi_l_imm32_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.au = self.da[ry].wrapping_sub(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.icount -= 2;
            self.aob = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
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
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_subc(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
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

    pub fn partial_cmpi_l_imm32_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
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
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_subc(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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

    pub fn partial_cmpi_l_imm32_dais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_subc(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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
            self.irc = self.edb;
            self.dbin = self.edb;
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

    pub fn partial_cmpi_l_imm32_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
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
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_subc(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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

    pub fn partial_cmpi_l_imm32_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
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
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.dt) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
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
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
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
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_sub(((self.dt) as u16), self.dbin);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_subc(M68000::high16(self.dt), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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

    pub fn partial_move_b_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
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

    pub fn partial_move_b_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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

    pub fn partial_move_b_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[rx]) as u8);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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

    pub fn partial_move_b_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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

    pub fn partial_move_b_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            self.dcr = ((self.da[rx]) as u8);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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

    pub fn partial_move_b_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
                self.dcr = ((self.da[rx]) as u8);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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
                self.dcr = ((self.da[rx]) as u8);
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

    pub fn partial_move_b_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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

    pub fn partial_move_b_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            self.dcr = ((self.da[rx]) as u8);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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

    pub fn partial_move_b_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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

    pub fn partial_move_b_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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
            M68000::set_8_32(&mut self.da[rx], ((self.dbin) as u8));
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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

    pub fn partial_move_b_imm8_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = self.dt;
            self.au = self.au.wrapping_add(2);
            self.alu_and8(((self.dt) as u16), 0xffffu16);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_ds_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, ((self.da[ry]) as u16));
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
            self.alu_and8(((self.da[ry]) as u16), 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
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

    pub fn partial_move_b_ais_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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

    pub fn partial_move_b_aips_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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

    pub fn partial_move_b_pais_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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

    pub fn partial_move_b_das_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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

    pub fn partial_move_b_dais_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
                self.dcr = ((0u16) as u8);
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

    pub fn partial_move_b_adr16_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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

    pub fn partial_move_b_adr32_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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

    pub fn partial_move_b_dpc_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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

    pub fn partial_move_b_dpci_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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
            self.aob = self.da[rx];
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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

    pub fn partial_move_b_imm8_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, ((self.dt) as u16));
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
            self.alu_and8(((self.dt) as u16), 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
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

    pub fn partial_move_b_ds_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, ((self.da[ry]) as u16));
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
            self.alu_and8(((self.da[ry]) as u16), 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_ais_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            self.dbin = self.edb;
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_aips_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_pais_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_das_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_dais_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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
                self.dcr = ((0u16) as u8);
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

    pub fn partial_move_b_adr16_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_adr32_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_dpc_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_dpci_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, self.dbin);
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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

    pub fn partial_move_b_imm8_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, ((self.dt) as u16));
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
            self.alu_and8(((self.dt) as u16), 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
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

    pub fn partial_move_b_ds_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(((self.da[ry]) as u16), 0xffffu16);
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
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.inst_substate = 0;
            return;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_ais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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

    pub fn partial_move_b_aips_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((0u16) as u8);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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

    pub fn partial_move_b_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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

    pub fn partial_move_b_das_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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

    pub fn partial_move_b_dais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                self.dcr = ((0u16) as u8);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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
                self.dcr = ((0u16) as u8);
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

    pub fn partial_move_b_adr16_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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

    pub fn partial_move_b_adr32_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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

    pub fn partial_move_b_dpc_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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

    pub fn partial_move_b_dpci_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
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
            self.da[rx] = self.au;
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
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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

    pub fn partial_move_b_imm8_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.alu_and8(((self.dt) as u16), 0xffffu16);
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
            self.da[rx] = self.au;
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

    pub fn partial_move_b_ds_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[rx]);
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
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, ((self.da[ry]) as u16));
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(((self.da[ry]) as u16), 0xffffu16);
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

    pub fn partial_move_b_ais_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
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
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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

    pub fn partial_move_b_aips_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((0u16) as u8);
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
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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

    pub fn partial_move_b_pais_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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

    pub fn partial_move_b_das_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
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
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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

    pub fn partial_move_b_dais_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
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
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
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

    pub fn partial_move_b_adr16_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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

    pub fn partial_move_b_adr32_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
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
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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

    pub fn partial_move_b_dpc_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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

    pub fn partial_move_b_dpci_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and8(self.dbin, 0xffffu16);
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.step_movem();
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
                self.dbin = self.edb;
                self.step_movem();
                __state = 1; continue;
            }
            1 => {
                self.aob = self.au;
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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

    pub fn partial_move_b_imm8_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.aob = self.au;
            self.pc = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[rx]);
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
            self.ir = self.irc;
            M68000::set_8xl(&mut self.dbout, ((self.dt) as u16));
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(((self.dt) as u16), 0xffffu16);
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

    pub fn partial_move_b_ds_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
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
                self.ir = self.irc;
                M68000::set_8xl(&mut self.dbout, ((self.da[ry]) as u16));
                self.au = self.pc.wrapping_add(2);
                self.alu_and8(((self.da[ry]) as u16), 0xffffu16);
                self.sr_nzvc();
                self.base_ssw = SSW_DATA;
                if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
                else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
                self.icount -= 4;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
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
                self.ir = self.irc;
                M68000::set_8xl(&mut self.dbout, ((self.da[ry]) as u16));
                self.au = self.pc.wrapping_add(2);
                self.alu_and8(((self.da[ry]) as u16), 0xffffu16);
                self.sr_nzvc();
                self.base_ssw = SSW_DATA;
                if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
                else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
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

    pub fn partial_move_b_ais_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 2;
            return;
        }
        2 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        6 => {
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
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_aips_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((0u16) as u8);
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        6 => {
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
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_pais_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        6 => {
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
        12 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 14; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_das_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 16; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            _ => { return; }
            }}
            continue;
        }
        8 => {
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 16; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_dais_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 20; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 22; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            _ => { return; }
            }}
            continue;
        }
        8 => {
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
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
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
        20 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 22; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
        22 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_adr16_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 16; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            _ => { return; }
            }}
            continue;
        }
        8 => {
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 16; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_adr32_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.dcr = ((0u16) as u8);
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                self.inst_substate = 18; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        10 => {
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
            self.irc = self.edb;
            self.dbin = self.edb;
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
                self.inst_substate = 18; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_dpc_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.edb = bus.read_program(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 14; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 16; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            _ => { return; }
            }}
            continue;
        }
        8 => {
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
        14 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 16; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_dpci_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.dbin = self.edb;
                self.alu_ext(self.aluo);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 20; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.inst_substate = 22; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
            _ => { return; }
            }}
            continue;
        }
        8 => {
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
                self.at = self.au;
                self.au = self.pc.wrapping_add(4);
                self.alu_and8(self.dbin, 0xffffu16);
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.edb = bus.read_program(self.aob & !1u32);
                if self.aob & 1 == 0 { self.edb >>= 8; }
                self.icount -= 4;
                self.inst_substate = 4;
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
                self.irc = self.edb;
                self.inst_substate = 22; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.alu_and8(self.dbin, 0xffffu16);
                self.sr_nzvc();
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
        22 => {
            let mut __state: u32 = 1;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                __state = 1; continue;
            }
            1 => {
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
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_move_b_imm8_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            let mut __state: u32 = 0;
            loop { match __state {
            0 => {
                self.irc = self.edb;
                self.dbin = self.edb;
                self.alu_ext(self.dbin);
                self.icount -= 2;
                self.aob = self.au;
                self.t = (self.irc & 0x0800) as i32;
                self.pc = self.au;
                self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
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
                self.ir = self.irc;
                M68000::set_8xl(&mut self.dbout, ((self.dt) as u16));
                self.au = self.pc.wrapping_add(2);
                self.alu_and8(((self.dt) as u16), 0xffffu16);
                self.sr_nzvc();
                self.base_ssw = SSW_DATA;
                if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
                else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
                self.icount -= 4;
                self.inst_substate = 6;
                return;
            }
            _ => { return; }
            }}
            continue;
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
                self.ir = self.irc;
                M68000::set_8xl(&mut self.dbout, ((self.dt) as u16));
                self.au = self.pc.wrapping_add(2);
                self.alu_and8(((self.dt) as u16), 0xffffu16);
                self.sr_nzvc();
                self.base_ssw = SSW_DATA;
                if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
                else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
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

    pub fn partial_move_b_ds_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = ((self.da[ry]) as u16); M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::ext32(self.dbin);
            self.alu_and8(((self.da[ry]) as u16), 0xffffu16);
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
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8xl(&mut self.dbout, self.aluo);
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 { bus.write_data_byte(self.aob, self.dbout as u8); }
            else { bus.write_data_byte(self.aob, (self.dbout >> 8) as u8); }
            self.icount -= 4;
            self.inst_substate = 4;
            return;
        }
        4 => {
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

}
