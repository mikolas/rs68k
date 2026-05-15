use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_assignments, unused_mut, unreachable_code)]
impl M68000 {
    pub fn handler_add_b_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add8(self.dbin, self.da[rx]);
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

    pub fn handler_add_b_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.dbin, self.da[rx]);
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

    pub fn handler_add_b_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.dbin, self.da[rx]);
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

    pub fn handler_add_b_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.dbin, self.da[rx]);
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

    pub fn handler_add_b_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add8(self.dbin, self.da[rx]);
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

    pub fn handler_add_b_imm8_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.dt, self.da[rx]);
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

    pub fn handler_add_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_add(self.da[ry], self.da[rx]);
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

    pub fn handler_add_w_as_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_add(self.da[ry], self.da[rx]);
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

    pub fn handler_add_w_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.dbin, self.da[rx]);
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

    pub fn handler_add_w_imm16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dt, self.da[rx]);
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

    pub fn handler_add_l_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_add(self.da[ry], self.da[rx]);
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
        self.alu_addc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_as_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_add(self.da[ry], self.da[rx]);
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
        self.alu_addc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_ais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_aips_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_pais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_das_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_dais_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.dbin, self.da[rx]);
            self.sr_xnzvc();
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_adr16_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_adr32_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_dpc_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        self.sr_xnzvc();
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_dpci_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.dbin, self.da[rx]);
            self.sr_xnzvc();
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_addc(self.alub, M68000::high16(self.da[rx]));
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

    pub fn handler_add_l_imm32_dd(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dt, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.dt), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_ds_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_add(self.da[ry], self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_as_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_add(self.da[ry], self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_ais_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_aips_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_pais_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_das_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_dais_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.dbin, self.da[rx]);
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
            self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_adr16_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_adr32_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_dpc_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_dpci_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.dbin, self.da[rx]);
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
            self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_w_imm16_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dt, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.at), M68000::high16(self.da[rx]));
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

    pub fn handler_addx_b_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_addx8(self.da[ry], self.da[rx]);
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

    pub fn handler_addx_b_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_addx8(self.alub, self.dbin);
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

    pub fn handler_add_b_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.da[rx], self.dbin);
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

    pub fn handler_add_b_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.da[rx], self.dbin);
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

    pub fn handler_add_b_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.da[rx], self.dbin);
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

    pub fn handler_add_b_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.da[rx], self.dbin);
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

    pub fn handler_add_b_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add8(self.da[rx], self.dbin);
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

    pub fn handler_add_b_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.da[rx], self.dbin);
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

    pub fn handler_add_b_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add8(self.da[rx], self.dbin);
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

    pub fn handler_addx_w_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_addx(self.da[ry], self.da[rx]);
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

    pub fn handler_addx_w_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_addx(self.alub, self.dbin);
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

    pub fn handler_add_w_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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

    pub fn handler_add_w_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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

    pub fn handler_add_w_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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

    pub fn handler_add_w_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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

    pub fn handler_add_w_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.da[rx], self.dbin);
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

    pub fn handler_add_w_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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

    pub fn handler_add_w_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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

    pub fn handler_addx_l_ds_dd(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_addx(self.da[ry], self.da[rx]);
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
        self.alu_addx(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
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

    pub fn handler_addx_l_pais_paid(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_addx(self.aluo, self.dbin);
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
        self.alu_addx(self.alub, self.dbin);
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

    pub fn handler_add_l_dd_ais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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
        self.alu_addc(M68000::high16(self.da[rx]), self.alue);
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

    pub fn handler_add_l_dd_aips(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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
        self.alu_addc(M68000::high16(self.da[rx]), self.alue);
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

    pub fn handler_add_l_dd_pais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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
        self.alu_addc(M68000::high16(self.da[rx]), self.alue);
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

    pub fn handler_add_l_dd_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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
        self.alu_addc(M68000::high16(self.da[rx]), self.alue);
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

    pub fn handler_add_l_dd_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.da[rx], self.dbin);
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
            self.alu_addc(M68000::high16(self.da[rx]), self.alue);
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

    pub fn handler_add_l_dd_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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
        self.alu_addc(M68000::high16(self.da[rx]), self.alue);
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

    pub fn handler_add_l_dd_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.da[rx], self.dbin);
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
        self.alu_addc(M68000::high16(self.da[rx]), self.alue);
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

    pub fn handler_adda_l_ds_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = (self.irdi & 7) as usize;
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_add(self.da[ry], self.da[rx]);
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
        self.alu_addc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_l_as_ad(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = self.map_sp((((self.irdi >> 9) & 7) | 8) as usize);
        let ry = self.map_sp(((self.irdi & 7) | 8) as usize);
        self.aob = self.au;
        self.ir = self.irc;
        self.pc = self.au;
        self.at = M68000::ext32(self.da[ry]);
        self.alu_add(self.da[ry], self.da[rx]);
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
        self.alu_addc(M68000::high16(self.da[ry]), M68000::high16(self.da[rx]));
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

    pub fn handler_adda_l_ais_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_aips_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_pais_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_das_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_dais_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.dbin, self.da[rx]);
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_addc(self.alub, M68000::high16(self.da[rx]));
            self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_adr16_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_adr32_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_dpc_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dbin, self.da[rx]);
        M68000::set_16l_32(&mut self.da[rx], self.aluo);
        self.alu_addc(self.alub, M68000::high16(self.da[rx]));
        self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_dpci_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.alu_add(self.dbin, self.da[rx]);
            M68000::set_16l_32(&mut self.da[rx], self.aluo);
            self.alu_addc(self.alub, M68000::high16(self.da[rx]));
            self.base_ssw = SSW_PROGRAM | SSW_R;
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

    pub fn handler_adda_l_imm32_ad(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.alu_add(self.dt, self.da[rx]);
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
        self.alu_addc(M68000::high16(self.dt), M68000::high16(self.da[rx]));
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

    pub fn handler_asr_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asr8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_lsr_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsr8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_roxr_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and8x(M68000::high16(self.da[ry]), 0xff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8x(self.da[ry], 0xff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxr8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_ror_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_ror8(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_asr_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asr8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_lsr_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsr8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_roxr_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and8x(M68000::high16(self.da[ry]), 0xff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8x(self.da[ry], 0xff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxr8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_ror_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_ror8(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_asr_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asr(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_lsr_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsr(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_roxr_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_andx(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_andx(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxr(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_ror_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_ror(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_asr_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asr(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_lsr_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsr(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_roxr_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_andx(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_andx(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxr(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_ror_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_ror(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_asr_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asr32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_lsr_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsr32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_roxr_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_andx(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_andx(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxr32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_andx(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_ror_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_ror32(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_asr_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asr32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_lsr_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsr32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_roxr_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_andx(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_andx(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxr32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_andx(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_ror_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_ror32(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_asr_ais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asr(self.aluo);
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

    pub fn handler_asr_aips(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asr(self.aluo);
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

    pub fn handler_asr_pais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asr(self.aluo);
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

    pub fn handler_asr_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asr(self.aluo);
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

    pub fn handler_asr_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.au = self.at;
            self.alu_asr(self.aluo);
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

    pub fn handler_asr_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asr(self.aluo);
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

    pub fn handler_asr_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asr(self.aluo);
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

    pub fn handler_asl_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asl8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_lsl_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsl8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_roxl_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and8x(M68000::high16(self.da[ry]), 0xff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8x(self.da[ry], 0xff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxl8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_rol_b_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_rol8(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_asl_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asl8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_lsl_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsl8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_roxl_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and8x(M68000::high16(self.da[ry]), 0xff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8x(self.da[ry], 0xff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxl8(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_rol_b_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and8(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and8(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_rol8(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
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

    pub fn handler_asl_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asl(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_lsl_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsl(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_roxl_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_andx(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_andx(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxl(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_rol_w_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_rol(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_asl_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asl(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_lsl_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsl(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_roxl_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_andx(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_andx(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxl(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_rol_w_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_rol(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            M68000::set_16l_32(&mut self.da[ry], self.aluo);
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

    pub fn handler_asl_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asl32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_lsl_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsl32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.alu_and(self.aluo, self.alue);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_roxl_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_andx(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_andx(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxl32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_andx(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_rol_l_imm3_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.ftu);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_rol32(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_asl_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_asl32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_lsl_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_lsl32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.alu_and(self.aluo, self.alue);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_roxl_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_andx(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_andx(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_roxl32(self.aluo);
            self.sr_xnzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_andx(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_rol_l_dd_ds(&mut self, bus: &mut impl crate::types::Bus) {
        let rx = ((self.irdi >> 9) & 7) as usize;
        let ry = (self.irdi & 7) as usize;
        let mut __state = 0u32;
        loop { match __state {
        0 => {
            self.aob = self.au;
            self.ir = self.irc;
            self.pc = self.au;
            self.au = M68000::ext32(self.da[rx]);
            self.alu_and(M68000::high16(self.da[ry]), 0xffff);
            self.t = !(self.au & 0x3f);
            self.alue = self.aluo;
            self.au = self.au - 1;
            self.alu_and(self.da[ry], 0xffff);
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
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        1 => {
            self.t = !(self.au & 0x3f);
            self.au = self.au - 1;
            self.alu_rol32(self.aluo);
            self.sr_nzvc();
            self.icount -= 2;
            if self.t != 0 { __state = 2; continue; } else { __state = 1; continue; }
        }
        2 => {
            self.alub = self.alue;
            self.da[ry] = M68000::merge_16_32(self.alue, self.aluo);
            self.icount -= 2;
            self.ird = self.ir;
            if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
            self.au = self.pc + 2;
            self.alu_and(self.alub, 0xffff);
            self.sr_nz_u();
            self.set_ftu_const();
            self.icount -= 2;
            self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] };
            if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
            return;
        }
        _ => return,
        }} // end match+loop
    }

    pub fn handler_asl_ais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asl(self.aluo);
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

    pub fn handler_asl_aips(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asl(self.aluo);
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

    pub fn handler_asl_pais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asl(self.aluo);
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

    pub fn handler_asl_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asl(self.aluo);
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

    pub fn handler_asl_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.au = self.at;
            self.alu_asl(self.aluo);
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

    pub fn handler_asl_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asl(self.aluo);
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

    pub fn handler_asl_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_asl(self.aluo);
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

    pub fn handler_lsr_ais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsr(self.aluo);
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

    pub fn handler_lsr_aips(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsr(self.aluo);
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

    pub fn handler_lsr_pais(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsr(self.aluo);
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

    pub fn handler_lsr_das(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsr(self.aluo);
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

    pub fn handler_lsr_dais(&mut self, bus: &mut impl crate::types::Bus) {
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
            self.au = self.au + 2;
            self.alu_and(self.dbin, 0xffff);
            self.sr_nzvc();
            self.au = self.at;
            self.alu_lsr(self.aluo);
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

    pub fn handler_lsr_adr16(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsr(self.aluo);
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

    pub fn handler_lsr_adr32(&mut self, bus: &mut impl crate::types::Bus) {
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
        self.au = self.au + 2;
        self.alu_and(self.dbin, 0xffff);
        self.sr_nzvc();
        self.au = self.at;
        self.alu_lsr(self.aluo);
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

}
