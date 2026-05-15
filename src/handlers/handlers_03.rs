use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_mut, unreachable_code, unused_assignments)]
impl M68000 {
    pub fn handler_movea_w_dpc_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.da[rx] = M68000::ext32(self.dbin);
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_movea_w_dpci_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.da[rx] = M68000::ext32(self.dbin);
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_movea_w_imm16_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(((self.dt) as u16));
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.at;
        self.alu_and(M68000::high16(self.dt), 0xffffu16);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ds_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = ((self.da[ry]) as u16);
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_as_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = ((self.da[ry]) as u16);
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ais_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_aips_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((0u16) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_pais_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_das_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dais_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.da[rx];
            self.dbout = self.dbin;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_adr16_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_adr32_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpc_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpci_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.da[rx];
            self.dbout = self.dbin;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.pc;
            self.ir = self.irc;
            self.au = self.pc.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_imm16_aid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = ((self.dt) as u16);
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(((self.dt) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ir = self.irc;
        self.au = self.pc.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ds_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = ((self.da[ry]) as u16);
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_as_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = ((self.da[ry]) as u16);
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ais_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_aips_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((0u16) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_pais_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_das_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dais_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.da[rx];
            self.ir = self.irc;
            self.dbout = self.dbin;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_adr16_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_adr32_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpc_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = self.dbin;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpci_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.da[rx];
            self.ir = self.irc;
            self.dbout = self.dbin;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.pc;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_imm16_aipd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.da[rx];
        self.ir = self.irc;
        self.dbout = ((self.dt) as u16);
        self.pc = self.au;
        self.au = self.da[rx].wrapping_add(2);
        self.alu_and(((self.dt) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.pc;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ds_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_as_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_aips_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((0u16) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_das_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dais_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_sub(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
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

    pub fn handler_move_w_adr16_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_adr32_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpc_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpci_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_sub(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
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

    pub fn handler_move_w_imm16_paid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.da[rx].wrapping_sub(2);
        self.alu_and(((self.dt) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ds_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[rx]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = ((self.da[ry]) as u16);
        self.au = self.pc.wrapping_add(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_as_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[rx]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = ((self.da[ry]) as u16);
        self.au = self.pc.wrapping_add(2);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ais_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_aips_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((0u16) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_pais_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_das_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dais_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_adr16_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_adr32_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpc_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpci_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo).wrapping_add(self.da[rx]);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_imm16_dad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[rx]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.dbout = ((self.dt) as u16);
        self.au = self.pc.wrapping_add(2);
        self.alu_and(((self.dt) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ds_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.dbout = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_as_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.dbout = ((self.da[ry]) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.da[ry]) as u16), 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_ais_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_aips_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_pais_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_das_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_dais_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 6; continue;
        }
        5 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 6; continue;
        }
        6 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_adr16_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_adr32_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((0u16) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_dpc_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_dpci_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alu_ext(self.aluo);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 6; continue;
        }
        5 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            __state = 6; continue;
        }
        6 => {
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_imm16_daid(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alu_ext(self.dbin);
            self.icount -= 2;
            self.aob = self.au;
            self.t = (self.irc & 0x0800) as i32;
            self.pc = self.au;
            self.au = self.da[rx].wrapping_add(M68000::ext32(self.aluo));
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
            self.au = M68000::ext32(((self.da[self.map_sp((self.irc >> 12) as usize)]) as u16)).wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.dbout = ((self.dt) as u16);
            self.au = self.pc.wrapping_add(2);
            self.alu_and(((self.dt) as u16), 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_ds_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.pc = self.au;
        { let v = ((self.da[ry]) as u16); M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::ext32(self.dbin);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_as_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.pc = self.au;
        { let v = ((self.da[ry]) as u16); M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::ext32(self.dbin);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ais_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_aips_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((0u16) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_pais_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_das_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dais_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_adr16_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_adr32_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpc_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.au = M68000::ext32(self.aluo);
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpci_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.au = M68000::ext32(self.aluo);
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.set_ftu_const();
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_imm16_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = ((self.dt) as u16); M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::ext32(self.dbin);
        self.alu_and(((self.dt) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ds_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = ((self.da[ry]) as u16); M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_as_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = ((self.da[ry]) as u16); M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_and(((self.da[ry]) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_ais_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.ir = self.irc;
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_aips_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((0u16) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.ir = self.irc;
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_pais_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.ir = self.irc;
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_das_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.ir = self.irc;
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dais_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.ir = self.irc;
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_adr16_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.ir = self.irc;
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_adr32_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((0u16) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.ir = self.irc;
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpc_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((0u16) as u8);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
        self.alu_and(self.dbin, 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.ir = self.irc;
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.ftu = self.dbin;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_w_dpci_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((0u16) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            { let v = self.aluo; M68000::set_16h(&mut self.at, v); }
            self.alu_and(self.dbin, 0xffffu16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.ir = self.irc;
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.movemr = self.dbin;
            self.pc = self.au;
            self.ftu = self.dbin;
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_move_w_imm16_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        { let v = self.dbin; M68000::set_16l_32(&mut self.dt, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = ((self.dt) as u16); M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_and(((self.dt) as u16), 0xffffu16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_b_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_subx8(((self.da[ry]) as u8), ((0x0000u16) as u8));
        self.sr_xnzvc_u();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_b_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx8(((self.dbin) as u8), ((0x0000u16) as u8));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_negx_b_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx8(((self.dbin) as u8), ((0x0000u16) as u8));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_negx_b_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx8(((self.dbin) as u8), ((0x0000u16) as u8));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_negx_b_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx8(((self.dbin) as u8), ((0x0000u16) as u8));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_negx_b_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_subx8(((self.dbin) as u8), ((0x0000u16) as u8));
            self.sr_xnzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_negx_b_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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
        self.alu_subx8(((self.dbin) as u8), ((0x0000u16) as u8));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_negx_b_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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
        self.alu_subx8(((self.dbin) as u8), ((0x0000u16) as u8));
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_negx_w_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_subx(((self.da[ry]) as u16), 0x0000u16);
        self.sr_xnzvc_u();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_w_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_w_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_w_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_w_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_w_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_subx(self.dbin, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
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

    pub fn handler_negx_w_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_w_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_l_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_subx(((self.da[ry]) as u16), 0x0000u16);
        self.sr_xnzvc_u();
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.alu_subc(M68000::high16(self.da[ry]), 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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
        return;
    }

    pub fn handler_negx_l_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.dbout = self.aluo;
        self.au = self.at.wrapping_sub(2);
        self.alu_subc(self.alub, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_l_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.au.wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.dbout = self.aluo;
        self.au = self.at.wrapping_sub(2);
        self.alu_subc(self.alub, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_l_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.au = self.da[ry].wrapping_sub(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.icount -= 2;
        self.aob = self.au;
        self.da[ry] = self.au;
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.dbout = self.aluo;
        self.au = self.at.wrapping_sub(2);
        self.alu_subc(self.alub, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_l_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.dbout = self.aluo;
        self.au = self.at.wrapping_sub(2);
        self.alu_subc(self.alub, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_l_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.alub = self.dbin;
            self.alue = self.dbin;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_subx(self.dbin, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.dbout = self.aluo;
            self.au = self.at.wrapping_sub(2);
            self.alu_subc(self.alub, 0x0000u16);
            self.sr_xnzvc_u();
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
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

    pub fn handler_negx_l_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.dbout = self.aluo;
        self.au = self.at.wrapping_sub(2);
        self.alu_subc(self.alub, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_negx_l_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.alub = self.dbin;
        self.alue = self.dbin;
        self.at = self.au;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_subx(self.dbin, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.dbout = self.aluo;
        self.au = self.at.wrapping_sub(2);
        self.alu_subc(self.alub, 0x0000u16);
        self.sr_xnzvc_u();
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_sr_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.ftu = self.sr;
        { let v = self.ftu; M68000::set_16l_32(&mut self.da[ry], v); }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.au = self.pc.wrapping_add(2);
        self.set_ftu_const();
        self.icount -= 2;
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_sr_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.ftu = self.sr;
        self.au = self.at;
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_sr_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.ftu = self.sr;
        self.au = self.at;
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_sr_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
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
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.ftu = self.sr;
        self.au = self.at;
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_sr_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.ftu = self.sr;
        self.au = self.at;
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_sr_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.ftu = self.sr;
            self.au = self.at;
            self.alu_and(self.ftu, 0xffffu16);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.dbout = self.aluo;
            self.au = self.pc.wrapping_add(2);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
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

    pub fn handler_move_sr_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.ftu = self.sr;
        self.au = self.at;
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_move_sr_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.at;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.dbin, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.ftu = self.sr;
        self.au = self.at;
        self.alu_and(self.ftu, 0xffffu16);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_chk_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), ((self.da[ry]) as u16));
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.da[ry];
            self.at = self.da[ry];
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 6; continue;
        }
        5 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 4; } else { __state = 7; } continue;
        }
        6 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        7 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 6; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            self.at = M68000::ext32(self.dbin);
            self.au = M68000::ext32(self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.pc = self.au;
            self.dcr = ((self.da[rx]) as u8);
            { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.at;
            self.au = self.pc.wrapping_add(2);
            self.alu_and(self.dbin, 0xffffu16);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let mut __state: u32 = 0;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.aob = self.au;
            self.at = self.au;
            self.au = self.pc.wrapping_add(4);
            self.alu_and(self.dbin, 0xffffu16);
            self.dcr = ((self.da[rx]) as u8);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), self.dbin);
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 5; } else { __state = 4; } continue;
        }
        4 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 6; continue;
        }
        5 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 4; } else { __state = 7; } continue;
        }
        6 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        7 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 6; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_chk_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.alub = ((self.da[rx]) as u16);
            self.au = self.au.wrapping_sub(2);
            self.alu_sub(((self.da[rx]) as u16), ((self.dt) as u16));
            self.sr_nzvc();
            self.icount -= 2;
            self.t = if self.isr & (SR_V|SR_N) == 0 { 1 } else { 0 };
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.alu_and(self.alub, 0xffffu16);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; } else { __state = 1; } continue;
        }
        1 => {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.da[16] = self.au;
            self.ftu = 0x0018u16;
            self.au = self.au.wrapping_add(2);
            self.alu_and(M68000::high16(self.pc), 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.au;
            self.dbout = self.aluo;
            self.at = M68000::ext32(self.ftu);
            self.au = self.au.wrapping_sub(4);
            self.alu_and(self.alub, 0xffffu16);
            self.base_ssw = SSW_DATA;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            bus.write_data(self.aob & !1u32, self.dbout);
            self.icount -= 4;
            self.aob = self.at;
            self.pc = self.at;
            self.au = self.at.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
            self.au = self.au.wrapping_add(2);
            self.base_ssw = SSW_DATA | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_data(self.aob & !1u32);
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
            self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin).wrapping_add(2);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        2 => {
            self.t = (self.isr & SR_N) as i32;
            self.icount -= 2;
            if self.t != 0 { __state = 1; } else { __state = 4; } continue;
        }
        3 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.set_ftu_const();
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        4 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            __state = 3; continue;
        }
        _ => return,
        }}
    }

    pub fn handler_lea_ais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.movemr = self.dbin;
        self.pc = self.au;
        self.at = self.da[ry];
        self.ftu = self.dbin;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.da[rx] = self.at;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_lea_das_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_lea_dais_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(4);
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_lea_adr16_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.da[rx] = M68000::ext32(self.dbin);
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_lea_adr32_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.da[rx], v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        { let v = self.dbin; M68000::set_16l_32(&mut self.da[rx], v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_lea_dpc_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.pc);
        self.da[rx] = self.au;
        self.au = self.pc.wrapping_add(4);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_lea_dpci_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        3 => {
            self.da[rx] = self.au;
            self.au = self.pc.wrapping_add(4);
            self.icount -= 2;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.au.wrapping_add(2);
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_clr_b_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and8(((self.da[ry]) as u16), 0x0000u16);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        M68000::set_8_32(&mut self.da[ry], ((self.aluo) as u8));
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_clr_b_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and8(self.dbin, 0xffffu16);
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        self.edb = bus.read_data(self.aob & !1u32);
        if self.aob & 1 == 0 { self.edb >>= 8; }
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and8(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_clr_b_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(if ry < 15 { 1 } else { 2 });
        self.alu_and8(((self.da[self.movems as usize]) as u16), self.dbin);
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
        self.alu_and8(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_clr_b_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.au = self.da[ry].wrapping_sub(if ry < 15 { 1 } else { 2 });
        self.alu_and8(((self.da[self.movems as usize]) as u16), self.dbin);
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
        self.alu_and8(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_clr_b_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and8(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_clr_b_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
            self.irc = self.edb;
            self.dbin = self.edb;
            self.step_movem();
            __state = 3; continue;
        }
        2 => {
            self.au = self.da[self.map_sp((self.irc >> 12) as usize)].wrapping_add(self.au);
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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
            self.dcr = ((self.da[self.movems as usize]) as u8);
            self.base_ssw = SSW_DATA | SSW_R;
            self.edb = bus.read_data(self.aob & !1u32);
            if self.aob & 1 == 0 { self.edb >>= 8; }
            self.icount -= 4;
            self.dbin = self.edb;
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = self.at;
            self.alu_and8(self.dbin, 0x0000u16);
            self.sr_nzvc();
            self.base_ssw = SSW_PROGRAM | SSW_R;
            if self.aob & 1 != 0 {
                self.inst_state = S_ADDRESS_ERROR;
                return;
            }
            self.edb = bus.read_program(self.aob & !1u32);
            self.icount -= 4;
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

    pub fn handler_clr_b_adr16(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = M68000::ext32(self.dbin);
        self.au = M68000::ext32(self.dbin);
        self.alu_and8(((self.da[self.movems as usize]) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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
        self.alu_and8(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_clr_b_adr32(&mut self, bus: &mut impl crate::types::Bus) {
        self.aob = self.au;
        { let v = self.dbin; M68000::set_16h(&mut self.at, v); }
        self.au = self.au.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        { let v = self.dbin; M68000::set_16l_32(&mut self.at, v); }
        self.au = M68000::merge_16_32(M68000::high16(self.at), self.dbin);
        self.alu_and8(((self.da[self.movems as usize]) as u16), self.dbin);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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
        self.alu_and8(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
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

    pub fn handler_clr_w_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.au.wrapping_add(2);
        self.alu_and(((self.da[ry]) as u16), 0x0000u16);
        self.sr_nzvc();
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        { let v = self.aluo; M68000::set_16l_32(&mut self.da[ry], v); }
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_clr_w_ais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.at = self.da[ry];
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_clr_w_aips(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.da[ry];
        self.alub = self.dbin;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.at = self.da[ry];
        self.au = self.da[ry].wrapping_add(2);
        self.alu_and(((self.da[self.movems as usize]) as u16), self.dbin);
        self.da[ry] = self.au;
        self.au = self.pc.wrapping_add(2);
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_clr_w_pais(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.alub = self.dbin;
        self.pc = self.au;
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.au = self.da[ry].wrapping_sub(2);
        self.alu_and(((self.da[self.movems as usize]) as u16), self.dbin);
        self.icount -= 2;
        self.aob = self.au;
        self.at = self.au;
        self.da[ry] = self.au;
        self.au = self.pc;
        self.alu_and(self.alub, 0xffffu16);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

    pub fn handler_clr_w_das(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.au = M68000::ext32(self.dbin).wrapping_add(self.da[ry]);
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.at = self.au;
        self.au = self.pc.wrapping_add(4);
        self.alu_and(self.dbin, 0xffffu16);
        self.dcr = ((self.da[self.movems as usize]) as u8);
        self.base_ssw = SSW_DATA | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_data(self.aob & !1u32);
        self.icount -= 4;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.au = self.at;
        self.alu_and(self.dbin, 0x0000u16);
        self.sr_nzvc();
        self.base_ssw = SSW_PROGRAM | SSW_R;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        self.edb = bus.read_program(self.aob & !1u32);
        self.icount -= 4;
        self.irc = self.edb;
        self.dbin = self.edb;
        self.aob = self.au;
        self.ird = self.ir;
        if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
        self.dbout = self.aluo;
        self.au = self.pc.wrapping_add(2);
        self.base_ssw = SSW_DATA;
        if self.aob & 1 != 0 {
            self.inst_state = S_ADDRESS_ERROR;
            return;
        }
        bus.write_data(self.aob & !1u32, self.dbout);
        self.icount -= 4;
        self.set_ftu_const();
        self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
        if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
        return;
    }

}
