use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_mut, unreachable_code, unused_assignments)]
impl M68000 {
    pub fn partial_cmp_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(((self.dt) as u16));
            self.alu_sub(((self.dt) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_sub(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_as_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_sub(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_cmp_l_imm32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(((self.dt) as u16));
            self.alu_sub(((self.dt) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.dt), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_ds_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_sub(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_as_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_sub(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_ais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_aips_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((0u16) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_pais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
            self.au = self.da[ry].wrapping_sub(2);
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_das_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_dais_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_adr16_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_adr32_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_dpc_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_dpci_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 10; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((0u16) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_w_imm16_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(((self.dt) as u16));
            self.alu_sub(((self.dt) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.at), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_eor8(((self.da[rx]) as u8), ((self.da[ry]) as u8));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpm_b_aips_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.pc = self.au;
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.da[rx];
            self.ir = self.irc;
            self.au = self.da[rx].wrapping_add(if rx < 15 { 1 } else { 2 });
            self.alu_and8(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_sub8(((self.aluo) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_b_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_eor_b_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_eor_b_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_eor_b_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_eor_b_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = true;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = true;
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

    pub fn partial_eor_b_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_eor_b_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_eor8(((self.da[rx]) as u8), ((self.dbin) as u8));
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_eor_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_eor(((self.da[rx]) as u16), ((self.da[ry]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpm_w_aips_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.pc = self.au;
            self.au = self.da[ry].wrapping_add(2);
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            self.au = self.da[rx].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_sub(self.aluo, self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_w_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_w_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_w_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.au = self.da[ry].wrapping_sub(2);
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_w_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_w_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_eor_w_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_w_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.alu_eor(((self.da[rx]) as u16), ((self.da[ry]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor(M68000::high16(self.da[rx]), M68000::high16(self.da[ry]));
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

    pub fn partial_cmpm_l_aips_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.pc = self.au;
            self.au = self.da[ry].wrapping_add(2);
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.aob = self.at;
            self.alub = self.dbin;
            self.da[ry] = self.au;
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.aob = self.da[rx];
            self.ir = self.irc;
            self.au = self.da[rx].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alue = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.aob = self.pc;
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_sub(self.aluo, self.dbin);
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.alu_subc(self.alub, self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_l_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_l_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_l_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_l_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_l_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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

    pub fn partial_eor_l_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_eor(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_eor_l_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_eor(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_eor(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_ds_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_sub(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_as_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_sub(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_ais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_aips_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_pais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_das_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_dais_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_adr16_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_adr32_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_dpc_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_dpci_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_sub(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_cmpa_l_imm32_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(((self.dt) as u16));
            self.alu_sub(((self.dt) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.alu_subc(M68000::high16(self.dt), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_and8(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and8(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = true;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_b_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = true;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_and8(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_imm8_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(((self.dt) as u16));
            self.alu_and8(((self.dt) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_and(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(((self.dt) as u16));
            self.alu_and(((self.dt) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_and(((self.da[ry]) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_and_l_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_and_l_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_and_l_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_and_l_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_and_l_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_and_l_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_and_l_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_and_l_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(self.dbin);
            self.alu_and(self.dbin, ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.alu_and(self.alub, M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
            self.au = self.pc.wrapping_add(2);
            self.set_ftu_const();
            self.icount -= 2;
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
                self.pin_state.pending_byte = false;
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
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_imm32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.at = M68000::ext32(((self.dt) as u16));
            self.alu_and(((self.dt) as u16), ((self.da[rx]) as u16));
            self.sr_nzvc();
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(M68000::high16(self.dt), M68000::high16(self.da[rx]));
            self.sr_nzvc_u();
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            { let v = self.aluo; M68000::set_16h(&mut self.da[rx], v); }
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

    pub fn partial_mulu_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.alub = ((self.da[rx]) as u16);
            self.alue = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_mulu_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_mulu_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_mulu_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
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
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_mulu_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_mulu_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_mulu_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_mulu_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.irc = self.edb;
                self.dbin = self.edb;
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_mulu_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_mulu_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_mulu_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = ((self.dt) as u16);
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32mu(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
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

    pub fn partial_abcd_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_abcd8(((self.da[ry]) as u8), ((self.da[rx]) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
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

    pub fn partial_abcd_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.da[ry] = self.au;
            self.alu_and8(self.dbin, 0xffffu16);
            self.au = self.da[rx].wrapping_sub(if rx < 15 { 1 } else { 2 });
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.alub = self.dbin;
            self.at = self.au;
            self.da[rx] = self.au;
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.pc;
            self.alu_abcd8(((self.alub) as u8), ((self.dbin) as u8));
            self.sr_xnzvc_u();
            self.au = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and8(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.alub = self.dbin;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.at = self.au;
            self.da[ry] = self.au;
            self.au = self.pc;
            self.alu_and8(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = true;
                self.inst_substate = 4;
                return;
            }
            _ => { return; }
            }}
            continue;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_and_b_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 6;
            return;
        }
        6 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
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

    pub fn partial_exg_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[rx];
            self.au = self.da[ry];
            self.da[rx] = self.au;
            self.da[ry] = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_exg_ad_as(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[rx];
            self.au = self.da[ry];
            self.da[rx] = self.au;
            self.da[ry] = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 12; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_w_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_exg_dd_as(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[rx];
            self.au = self.da[ry];
            self.da[rx] = self.au;
            self.da[ry] = self.at;
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_and(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_and(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_and(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_and(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
                if self.t != 0 { self.inst_substate = 16; break; } else { __state = 1; continue; }
            }
            1 => {
                self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_and(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dbout = self.aluo;
            self.au = self.au.wrapping_sub(2);
            self.alu_and(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_and_l_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alu_and(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alu_and(M68000::high16(self.da[rx]), self.alue);
            self.sr_nzvc_u();
            self.base_ssw = SSW_DATA;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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

    pub fn partial_muls_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.alub = ((self.da[rx]) as u16);
            self.alue = ((self.da[ry]) as u16);
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_muls_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_muls_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.alub = self.dbin;
            self.dcr = ((self.da[rx]) as u8);
            self.at = self.da[ry];
            self.au = self.da[ry].wrapping_add(2);
            self.da[ry] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_muls_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
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
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_muls_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_muls_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
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
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 12; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_DATA | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_muls_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_muls_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.irc = self.edb;
                self.dbin = self.edb;
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_muls_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_muls_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
                self.pin_state.pending_byte = false;
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
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = self.dbin;
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                if self.t != 0 { __state = 2; } else { __state = 1; } continue;
            }
            1 => {
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
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
                self.dbin = self.edb;
                self.step_movem();
                self.inst_substate = 12; break;
            }
            1 => {
                self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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
                self.dcr = ((self.da[rx]) as u8);
                self.base_ssw = SSW_PROGRAM | SSW_R;
                self.pin_state.pending_byte = false;
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

    pub fn partial_muls_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.alub = ((self.da[rx]) as u16);
            self.alue = ((self.dt) as u16);
            self.at = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(((self.da[rx]) as u16), 0x0000u16);
            self.sr_nzvc();
            self.t = (self.alue & 1) as i32;
            { let v = self.aluo; M68000::set_16l_32(&mut self.da[rx], v); }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
                self.t = if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 };
                { let v = ((self.at) as u16); M68000::set_16l_32(&mut self.pc, v); }
                self.au = self.au.wrapping_sub(1);
                self.alu_roxr32ms(self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                match self.t { 0 => { __state = 3; }, 1 => { __state = 4; }, 2 => { __state = 2; }, _ => { __state = 1; } }
                continue;
            }
            2 => {
                self.alu_sub(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            3 => {
                self.ird = self.ir;
                if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
                { let v = M68000::high16(self.at); M68000::set_16h(&mut self.pc, v); }
                self.da[rx] = M68000::merge_16_32(self.aluo, self.alue);
                self.au = self.at.wrapping_add(2);
                self.alu_and(self.aluo, 0xffffu16);
                self.sr_nzvc_u();
                self.set_ftu_const();
                self.icount -= 2;
                self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
                if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
                self.inst_substate = 0;
                return;
            }
            4 => {
                self.alu_add(self.alub, self.aluo);
                self.sr_nzvc_u();
                self.icount -= 2;
                __state = 1; continue;
            }
            _ => { return; }
            }}
            continue;
        }
        _ => { self.inst_substate = 0; return; }
        }}
    }

    pub fn partial_add_b_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(((self.da[ry]) as u16));
            self.alu_add8(((self.da[ry]) as u8), ((self.da[rx]) as u8));
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_add_b_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and8(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_add8(((self.dbin) as u8), ((self.da[rx]) as u8));
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_add_b_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_add8(((self.dbin) as u8), ((self.da[rx]) as u8));
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_add_b_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 2;
            return;
        }
        2 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_add8(((self.dbin) as u8), ((self.da[rx]) as u8));
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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

    pub fn partial_add_b_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        loop { match self.inst_substate {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
            self.pin_state.pending_byte = true;
            self.inst_substate = 4;
            return;
        }
        4 => {
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.at = M68000::ext32(self.dbin);
            self.alu_add8(((self.dbin) as u8), ((self.da[rx]) as u8));
            self.sr_xnzvc();
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_8_32(&mut self.da[rx], ((self.aluo) as u8));
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            self.pin_state.pending_byte = false;
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
