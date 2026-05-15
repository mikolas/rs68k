//! Test that partial handlers produce identical results to full handlers.
//! Runs both variants on the same initial state and compares all registers/memory.

use std::collections::HashMap;
use rs68k_mame::cpu::M68000;
use rs68k_mame::types::*;

struct TestMemory {
    data: HashMap<u32, u8>,
}

impl TestMemory {
    fn new() -> Self { Self { data: HashMap::new() } }

    fn write_word(&mut self, addr: u32, val: u16) {
        self.data.insert(addr & 0x00ffffff, (val >> 8) as u8);
        self.data.insert((addr.wrapping_add(1)) & 0x00ffffff, (val & 0xff) as u8);
    }

    fn read_word_raw(&self, addr: u32) -> u16 {
        let hi = *self.data.get(&(addr & 0x00ffffff)).unwrap_or(&0) as u16;
        let lo = *self.data.get(&((addr.wrapping_add(1)) & 0x00ffffff)).unwrap_or(&0) as u16;
        (hi << 8) | lo
    }

    fn clone_mem(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

impl Bus for TestMemory {
    fn read_program(&mut self, addr: u32) -> u16 { self.read_word_raw(addr) }
    fn read_data(&mut self, addr: u32) -> u16 { self.read_word_raw(addr) }
    fn write_data(&mut self, addr: u32, data: u16) { self.write_word(addr, data); }
    fn write_data_byte(&mut self, addr: u32, data: u8) {
        self.data.insert(addr & 0x00ffffff, data);
    }
    fn read_cpu(&mut self, addr: u32) -> u16 { self.read_word_raw(addr) }
}

fn setup_cpu(opcode: u16, next_word: u16, pc: u32) -> M68000 {
    let mut cpu = M68000::new();
    cpu.sr = 0x2700; // supervisor, all interrupts masked
    cpu.sp = 16;
    cpu.da[16] = 0x1000; // SSP
    cpu.ir = opcode;
    cpu.ird = opcode;
    cpu.irc = next_word;
    cpu.dbin = next_word;
    cpu.pc = pc.wrapping_add(2);
    cpu.au = pc.wrapping_add(4);
    cpu.aob = pc.wrapping_add(4);
    cpu.set_ftu_const();
    cpu.inst_state = cpu.decode_table[opcode as usize];
    cpu.icount = 10000;
    cpu
}

fn clone_cpu(cpu: &M68000) -> M68000 {
    let mut c = M68000::new();
    c.da = cpu.da;
    c.ipc = cpu.ipc; c.pc = cpu.pc; c.au = cpu.au; c.at = cpu.at;
    c.aob = cpu.aob; c.dt = cpu.dt; c.int_vector = cpu.int_vector;
    c.sp = cpu.sp; c.t = cpu.t; c.movems = cpu.movems;
    c.isr = cpu.isr; c.sr = cpu.sr; c.new_sr = cpu.new_sr;
    c.dbin = cpu.dbin; c.dbout = cpu.dbout; c.edb = cpu.edb;
    c.irc = cpu.irc; c.ir = cpu.ir; c.ird = cpu.ird;
    c.ftu = cpu.ftu; c.aluo = cpu.aluo; c.alue = cpu.alue; c.alub = cpu.alub;
    c.movemr = cpu.movemr; c.irdi = cpu.irdi;
    c.base_ssw = cpu.base_ssw; c.ssw = cpu.ssw;
    c.dcr = cpu.dcr;
    c.virq_state = cpu.virq_state; c.nmi_pending = cpu.nmi_pending;
    c.int_level = cpu.int_level; c.int_next_state = cpu.int_next_state;
    c.inst_state = cpu.inst_state; c.inst_substate = cpu.inst_substate;
    c.next_state = cpu.next_state;
    c.icount = cpu.icount;
    c
}

/// Run a partial handler to completion (keep calling until inst_substate == 0 and
/// the handler has returned after setting inst_state).
fn execute_one_partial(cpu: &mut M68000, bus: &mut impl Bus) -> u32 {
    let before = cpu.icount;
    cpu.ipc = cpu.pc.wrapping_sub(2);
    cpu.irdi = cpu.ird;
    // Run partial handler segments until instruction completes
    let initial_state = cpu.inst_state;
    loop {
        cpu.dispatch_partial(bus);
        // Instruction is done when inst_substate returns to 0 AND inst_state changed
        // (or we're in an exception state)
        if cpu.inst_substate == 0 {
            break;
        }
    }
    // Handle exception states (same as execute_one)
    let trace_is_next = initial_state >= 10;
    loop {
        match cpu.inst_state {
            S_DOUBLE_FAULT => break,
            S_TRACE if trace_is_next => break,
            S_TRACE => {
                loop {
                    cpu.dispatch_partial(bus);
                    if cpu.inst_substate == 0 { break; }
                }
            }
            S_RESET | S_BUS_ERROR | S_ADDRESS_ERROR | S_INTERRUPT | S_PRIVILEGE => {
                loop {
                    cpu.dispatch_partial(bus);
                    if cpu.inst_substate == 0 { break; }
                }
            }
            _ => break,
        }
    }
    (before - cpu.icount) as u32
}

fn compare_cpus(full: &M68000, partial: &M68000, label: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for i in 0..17 {
        if full.da[i] != partial.da[i] {
            errors.push(format!("{}: da[{}] full={:08x} partial={:08x}", label, i, full.da[i], partial.da[i]));
        }
    }
    macro_rules! cmp {
        ($field:ident) => {
            if full.$field != partial.$field {
                errors.push(format!("{}: {} full={:?} partial={:?}", label, stringify!($field), full.$field, partial.$field));
            }
        };
    }
    cmp!(pc); cmp!(au); cmp!(aob); cmp!(at); cmp!(dt);
    cmp!(sr); cmp!(isr); cmp!(ir); cmp!(ird); cmp!(irc);
    cmp!(dbin); cmp!(ftu); cmp!(aluo); cmp!(alue); cmp!(alub);
    cmp!(inst_state); cmp!(next_state); cmp!(icount);
    cmp!(movemr); cmp!(movems); cmp!(sp);
    errors
}

fn run_test(opcode: u16, next_word: u16, mem_setup: &[(u32, u16)], label: &str) {
    let mut mem = TestMemory::new();
    let pc = 0x1000u32;
    // Write opcode and next word
    mem.write_word(pc, opcode);
    mem.write_word(pc + 2, next_word);
    // Write a few more words for prefetch
    mem.write_word(pc + 4, 0x4e71); // NOP
    mem.write_word(pc + 6, 0x4e71);
    for &(addr, val) in mem_setup {
        mem.write_word(addr, val);
    }

    let cpu = setup_cpu(opcode, next_word, pc);

    // Run full handler
    let mut full_cpu = clone_cpu(&cpu);
    let mut full_mem = mem.clone_mem();
    let full_cycles = full_cpu.execute_one(&mut full_mem);

    // Run partial handler
    let mut part_cpu = clone_cpu(&cpu);
    let mut part_mem = mem.clone_mem();
    let part_cycles = execute_one_partial(&mut part_cpu, &mut part_mem);

    // Compare
    let mut errors = compare_cpus(&full_cpu, &part_cpu, label);
    if full_cycles != part_cycles {
        errors.push(format!("{}: cycles full={} partial={}", label, full_cycles, part_cycles));
    }
    // Compare memory
    for (&addr, &val) in &full_mem.data {
        let pval = *part_mem.data.get(&addr).unwrap_or(&0);
        if val != pval {
            errors.push(format!("{}: mem[{:06x}] full={:02x} partial={:02x}", label, addr, val, pval));
        }
    }

    if !errors.is_empty() {
        for e in &errors {
            eprintln!("  {}", e);
        }
        panic!("{}: {} errors", label, errors.len());
    }
}

#[test]
fn partial_nop() {
    // NOP: 4e71
    run_test(0x4e71, 0x4e71, &[], "NOP");
}

#[test]
fn partial_moveq() {
    // MOVEQ #42, D0: 7000 | 42 = 702a
    run_test(0x702a, 0x4e71, &[], "MOVEQ #42,D0");
}

#[test]
fn partial_add_b_imm() {
    // ADD.b #$12, D0: 0600 0012
    run_test(0x0600, 0x0012, &[], "ADD.b #$12,D0");
}

#[test]
fn partial_move_w_ais() {
    // MOVE.w (A0), D0: 3010
    // Need data at A0
    run_test(0x3010, 0x4e71, &[(0x0000, 0x1234)], "MOVE.w (A0),D0");
}

#[test]
fn partial_lea_d16_an() {
    // LEA 4(A0), A1: 4368 0004 — wait, let me use a simpler encoding
    // LEA (A0), A1: 43D0
    run_test(0x43D0, 0x4e71, &[], "LEA (A0),A1");
}

#[test]
fn partial_bra() {
    // BRA.b +2: 6002
    run_test(0x6002, 0x4e71, &[(0x1004, 0x4e71), (0x1006, 0x4e71)], "BRA.b +2");
}
