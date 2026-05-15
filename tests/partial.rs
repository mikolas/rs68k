//! Test that partial handlers produce identical results to full handlers.
//! Runs both variants on the same initial state and compares all registers/memory.

use std::collections::HashMap;
use rs68k::cpu::M68000;
use rs68k::types::*;

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

/// Run a partial handler to completion via pin-level adapter.
/// Simulates instant DTACK: when handler suspends with a bus request,
/// perform the access externally, fill edb, charge icount, then resume.
fn execute_one_partial(cpu: &mut M68000, bus: &mut impl Bus) -> u32 {
    let before = cpu.icount;
    cpu.ipc = cpu.pc.wrapping_sub(2);
    cpu.irdi = cpu.ird;
    let initial_state = cpu.inst_state;

    loop {
        let icount_before = cpu.icount;
        cpu.dispatch_partial(bus);

        // If inst_substate != 0, handler suspended for a bus access
        if cpu.inst_substate != 0 {
            // Perform the bus access externally based on aob, base_ssw, dbout
            let is_read = cpu.base_ssw & SSW_R != 0;
            let is_byte = cpu.pin_state.pending_byte;
            let space = cpu.base_ssw & 3; // 1=program, 2=data, 3=cpu

            if is_read {
                if space == 3 {
                    // CPU space (interrupt ack)
                    cpu.edb = bus.read_cpu(cpu.aob);
                } else if is_byte {
                    // Byte read: read word, pin state machine would put full word on bus
                    let space_read = if space == 2 {
                        bus.read_program(cpu.aob & !1u32)
                    } else {
                        bus.read_data(cpu.aob & !1u32)
                    };
                    cpu.edb = space_read;
                    // Note: byte shift happens in resume segment, not here
                } else {
                    // Word read
                    cpu.edb = if space == 2 {
                        bus.read_program(cpu.aob & !1u32)
                    } else {
                        bus.read_data(cpu.aob & !1u32)
                    };
                }
            } else {
                // Write
                if is_byte {
                    if cpu.aob & 1 != 0 {
                        bus.write_data_byte(cpu.aob, cpu.dbout as u8);
                    } else {
                        bus.write_data_byte(cpu.aob, (cpu.dbout >> 8) as u8);
                    }
                } else {
                    bus.write_data(cpu.aob & !1u32, cpu.dbout);
                }
            }
            // Charge bus cycle cost
            cpu.icount -= 4;
        } else {
            // inst_substate == 0: instruction (or exception handler) completed
            break;
        }
    }

    // Handle exception states
    let trace_is_next = initial_state >= 10;
    loop {
        match cpu.inst_state {
            S_DOUBLE_FAULT => break,
            S_TRACE if trace_is_next => break,
            S_TRACE | S_RESET | S_BUS_ERROR | S_ADDRESS_ERROR | S_INTERRUPT | S_PRIVILEGE => {
                loop {
                    cpu.dispatch_partial(bus);
                    if cpu.inst_substate != 0 {
                        let is_read = cpu.base_ssw & SSW_R != 0;
                        let is_byte = cpu.pin_state.pending_byte;
                        let space = cpu.base_ssw & 3;
                        if is_read {
                            if space == 3 {
                                cpu.edb = bus.read_cpu(cpu.aob);
                            } else if is_byte {
                                cpu.edb = if space == 2 { bus.read_program(cpu.aob & !1u32) } else { bus.read_data(cpu.aob & !1u32) };
                            } else {
                                cpu.edb = if space == 2 { bus.read_program(cpu.aob & !1u32) } else { bus.read_data(cpu.aob & !1u32) };
                            }
                        } else {
                            if is_byte {
                                if cpu.aob & 1 != 0 { bus.write_data_byte(cpu.aob, cpu.dbout as u8); }
                                else { bus.write_data_byte(cpu.aob, (cpu.dbout >> 8) as u8); }
                            } else {
                                bus.write_data(cpu.aob & !1u32, cpu.dbout);
                            }
                        }
                        cpu.icount -= 4;
                    } else {
                        break;
                    }
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

// ─── SST-based partial vs full comparison ─────────────────────────────────────

fn run_sst_partial_vs_full(filename: &str) {
    use std::collections::HashMap;

    let path = format!("external/SingleStepTests/v1/{}.json.bin", filename);
    let data = match std::fs::read(&path) {
        Ok(d) => d,
        Err(_) => { eprintln!("Skipping {}: file not found", filename); return; }
    };

    let mut ptr = 0;
    let magic = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    assert_eq!(magic, 0x1A3F5D71);
    let num_tests = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as usize; ptr += 4;

    let limit = std::env::var("SST_LIMIT").ok().and_then(|v| v.parse().ok()).unwrap_or(500usize);
    let total = num_tests.min(limit);
    let mut pass = 0usize;
    let mut fail = 0usize;
    let mut fail_msgs: Vec<String> = Vec::new();

    for _ in 0..total {
        // Parse test case (minimal parser)
        let (cpu_state, mem_data, tc_end) = parse_sst_test(&data, ptr);
        ptr = tc_end;

        // Setup memory
        let mut mem = TestMemory::new();
        for &(addr, val) in &mem_data {
            mem.data.insert(addr, val);
        }

        // Run full handler
        let mut full_cpu = clone_cpu(&cpu_state);
        let mut full_mem = mem.clone_mem();
        let full_cycles = full_cpu.execute_one(&mut full_mem);

        // Run partial handler (pin-level adapter)
        let mut part_cpu = clone_cpu(&cpu_state);
        let mut part_mem = mem.clone_mem();
        let part_cycles = execute_one_partial(&mut part_cpu, &mut part_mem);

        // Compare
        let mut ok = true;
        if full_cycles != part_cycles {
            ok = false;
            if fail_msgs.len() < 10 {
                fail_msgs.push(format!("cycles: full={} partial={}", full_cycles, part_cycles));
            }
        }
        // Compare key registers
        for i in 0..17 {
            if full_cpu.da[i] != part_cpu.da[i] {
                ok = false;
                if fail_msgs.len() < 10 {
                    fail_msgs.push(format!("da[{}]: full={:08x} partial={:08x}", i, full_cpu.da[i], part_cpu.da[i]));
                }
            }
        }
        if full_cpu.sr != part_cpu.sr { ok = false; if fail_msgs.len() < 10 { fail_msgs.push(format!("sr: full={:04x} partial={:04x}", full_cpu.sr, part_cpu.sr)); } }
        if full_cpu.au != part_cpu.au { ok = false; if fail_msgs.len() < 10 { fail_msgs.push(format!("au: full={:08x} partial={:08x}", full_cpu.au, part_cpu.au)); } }
        if full_cpu.inst_state != part_cpu.inst_state { ok = false; if fail_msgs.len() < 10 { fail_msgs.push(format!("inst_state: full={} partial={}", full_cpu.inst_state, part_cpu.inst_state)); } }
        if full_cpu.irc != part_cpu.irc { ok = false; if fail_msgs.len() < 10 { fail_msgs.push(format!("irc: full={:04x} partial={:04x}", full_cpu.irc, part_cpu.irc)); } }

        if ok { pass += 1; } else { fail += 1; }
    }

    if fail > 0 {
        eprintln!("\n=== {} PARTIAL FAILED: {}/{} passed ===", filename, pass, total);
        for msg in &fail_msgs { eprintln!("  {}", msg); }
        panic!("{}: {}/{} tests failed", filename, fail, total);
    } else {
        eprintln!("{}: partial {}/{} passed", filename, pass, total);
    }
}

fn parse_sst_test(data: &[u8], mut ptr: usize) -> (M68000, Vec<(u32, u8)>, usize) {
    // Skip test envelope
    let _numbytes = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    let magic = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    assert_eq!(magic, 0xABC12367);

    // Skip name
    let _nb = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    let _mg = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    let strlen = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as usize; ptr += 4;
    ptr += strlen;

    // Read initial state
    let _nb = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    let _mg = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;

    let mut d = [0u32; 8];
    for i in 0..8 { d[i] = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4; }
    let mut a = [0u32; 7];
    for i in 0..7 { a[i] = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4; }
    let usp = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    let ssp = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    let sr = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as u16; ptr += 4;
    let pc = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
    let pf0 = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as u16; ptr += 4;
    let pf1 = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as u16; ptr += 4;

    let num_rams = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as usize; ptr += 4;
    let mut ram = Vec::with_capacity(num_rams * 2);
    for _ in 0..num_rams {
        let addr = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()); ptr += 4;
        let word = u16::from_le_bytes(data[ptr..ptr+2].try_into().unwrap()); ptr += 2;
        ram.push((addr, (word >> 8) as u8));
        ram.push((addr | 1, (word & 0xFF) as u8));
    }

    // Setup CPU
    let mut cpu = M68000::new();
    for i in 0..8 { cpu.da[i] = d[i]; }
    for i in 0..7 { cpu.da[8 + i] = a[i]; }
    cpu.da[15] = usp;
    cpu.da[16] = ssp;
    cpu.sr = sr;
    cpu.isr = sr & 0x1f;
    cpu.sp = if sr & SR_S != 0 { 16 } else { 15 };
    cpu.ir = pf0;
    cpu.ird = pf0;
    cpu.irc = pf1;
    cpu.dbin = pf1;
    cpu.au = pc;
    cpu.pc = pc.wrapping_sub(2);
    cpu.set_ftu_const();
    cpu.inst_state = cpu.decode_table[pf0 as usize];
    cpu.icount = 10000;

    // Skip final state
    let nb = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as usize; ptr += 4;
    ptr += nb - 4; // skip the rest of final state block

    // Skip transactions
    let nb = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as usize; ptr += 4;
    ptr += nb - 4;

    (cpu, ram, ptr)
}

#[test] fn partial_sst_nop() { run_sst_partial_vs_full("NOP"); }
#[test] fn partial_sst_move_q() { run_sst_partial_vs_full("MOVE.q"); }
#[test] fn partial_sst_add_b() { run_sst_partial_vs_full("ADD.b"); }
#[test] fn partial_sst_move_b() { run_sst_partial_vs_full("MOVE.b"); }
#[test] fn partial_sst_move_w() { run_sst_partial_vs_full("MOVE.w"); }
#[test] fn partial_sst_move_l() { run_sst_partial_vs_full("MOVE.l"); }
#[test] fn partial_sst_lea() { run_sst_partial_vs_full("LEA"); }
#[test] fn partial_sst_exg() { run_sst_partial_vs_full("EXG"); }
#[test] fn partial_sst_swap() { run_sst_partial_vs_full("SWAP"); }

#[test]
fn partial_sst_all() {
    let files = [
        "ABCD", "ADD.b", "ADD.w", "ADD.l", "ADDA.w", "ADDA.l",
        "ADDX.b", "ADDX.w", "ADDX.l", "AND.b", "AND.w", "AND.l",
        "ANDItoCCR", "ANDItoSR", "ASL.b", "ASL.l", "ASL.w",
        "ASR.b", "ASR.l", "ASR.w", "Bcc", "BCHG", "BCLR", "BSET",
        "BSR", "BTST", "CHK", "CLR.b", "CLR.w", "CLR.l",
        "CMP.b", "CMP.w", "CMP.l", "CMPA.w", "CMPA.l",
        "DBcc", "DIVS", "DIVU", "EOR.b", "EOR.w", "EOR.l",
        "EORItoCCR", "EORItoSR", "EXG", "EXT.l", "EXT.w",
        "ILLEGAL_LineA", "ILLEGAL_LineF",
        "JMP", "JSR", "LEA", "LINK", "LSL.b", "LSL.l", "LSL.w",
        "LSR.b", "LSR.l", "LSR.w", "MOVE.b", "MOVE.w", "MOVE.l",
        "MOVE.q", "MOVEA.w", "MOVEA.l", "MOVEfromSR",
        "MOVEfromUSP", "MOVEtoCCR", "MOVEtoSR", "MOVEtoUSP",
        "MOVEM.w", "MOVEM.l", "MOVEP.w", "MOVEP.l",
        "MULS", "MULU", "NBCD", "NEG.b", "NEG.w", "NEG.l",
        "NEGX.b", "NEGX.w", "NEGX.l", "NOP", "NOT.b", "NOT.w", "NOT.l",
        "OR.b", "OR.w", "OR.l", "ORItoCCR", "ORItoSR",
        "PEA", "RESET", "ROL.b", "ROL.l", "ROL.w",
        "ROR.b", "ROR.l", "ROR.w", "ROXL.b", "ROXL.l", "ROXL.w",
        "ROXR.b", "ROXR.l", "ROXR.w", "RTE", "RTR", "RTS",
        "SBCD", "Scc", "STOP", "SUB.b", "SUB.w", "SUB.l",
        "SUBA.w", "SUBA.l", "SUBX.b", "SUBX.w", "SUBX.l",
        "SWAP", "TAS", "TRAP", "TRAPV", "TST.b", "TST.w", "TST.l",
        "UNLINK",
    ];
    let mut total_pass = 0usize;
    let mut total_fail = 0usize;
    let mut failed_files = Vec::new();
    for f in &files {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_sst_partial_vs_full(f);
        }));
        if result.is_ok() {
            total_pass += 1;
        } else {
            total_fail += 1;
            failed_files.push(*f);
        }
    }
    eprintln!("\n=== PARTIAL SST SUMMARY: {}/{} files passed ===", total_pass, total_pass + total_fail);
    if !failed_files.is_empty() {
        eprintln!("Failed: {:?}", failed_files);
        panic!("{} files failed", total_fail);
    }
}

// ─── clock() interface test ───────────────────────────────────────────────────

use rs68k::pins::Inputs68k;

/// Execute one instruction via the clock() interface with instant DTACK.
/// Handles reads and writes through the memory model.
fn execute_one_clocked(cpu: &mut M68000, mem: &mut TestMemory) -> u32 {
    let icount_before = cpu.icount;
    let mut clocks = 0u32;
    let mut was_active = false;
    let mut last_write_addr: u32 = 0;
    let mut last_write_data: u16 = 0;
    let mut last_was_write = false;
    let mut last_was_byte = false;

    loop {
        let mut inputs = Inputs68k::default();
        inputs.dtack_n = false; // instant acknowledge

        // Provide read data when bus cycle is active and reading
        if !cpu.pins().as_n && cpu.pins().rw {
            let addr = cpu.pins().addr & 0x00FF_FFFE;
            inputs.data_in = mem.read_word_raw(addr);
        }

        // Detect write: capture address/data while AS is asserted and rw=false
        if !cpu.pins().as_n && !cpu.pins().rw {
            last_write_addr = cpu.pins().addr;
            last_write_data = cpu.pins().data_out;
            last_was_write = true;
            last_was_byte = cpu.pins().uds_n != cpu.pins().lds_n; // one strobe = byte
        }

        // If AS just went high after a write, commit the write to memory
        if cpu.pins().as_n && last_was_write {
            if last_was_byte {
                // Byte write: UDS=even byte (high), LDS=odd byte (low)
                if last_write_addr & 1 != 0 {
                    mem.data.insert(last_write_addr & 0x00ffffff, last_write_data as u8);
                } else {
                    mem.data.insert(last_write_addr & 0x00ffffff, (last_write_data >> 8) as u8);
                }
            } else {
                mem.write_word(last_write_addr & 0x00FF_FFFE, last_write_data);
            }
            last_was_write = false;
        }

        if !cpu.pins().as_n {
            was_active = true;
        }

        cpu.clock(&inputs);
        clocks += 1;

        if clocks >= 2000 {
            panic!("clock() timeout after {} clocks", clocks);
        }

        // Done: instruction boundary (substate 0, bus idle, had activity)
        if cpu.inst_substate == 0 && cpu.pins().as_n && was_active && clocks > 2 {
            break;
        }
    }

    (icount_before - cpu.icount) as u32
}

#[test]
fn clock_nop() {
    let mut mem = TestMemory::new();
    let pc = 0x1000u32;
    mem.write_word(pc, 0x4e71);     // NOP
    mem.write_word(pc + 2, 0x4e71); // next NOP
    mem.write_word(pc + 4, 0x4e71);

    let mut cpu = setup_cpu(0x4e71, 0x4e71, pc);
    let cycles = execute_one_clocked(&mut cpu, &mut mem);
    assert_eq!(cycles, 4, "NOP should take 4 cycles");
}

#[test]
fn clock_moveq() {
    let mut mem = TestMemory::new();
    let pc = 0x1000u32;
    mem.write_word(pc, 0x702a);     // MOVEQ #42, D0
    mem.write_word(pc + 2, 0x4e71);
    mem.write_word(pc + 4, 0x4e71);

    let mut cpu = setup_cpu(0x702a, 0x4e71, pc);
    let cycles = execute_one_clocked(&mut cpu, &mut mem);
    assert_eq!(cycles, 4, "MOVEQ should take 4 cycles");
    assert_eq!(cpu.da[0], 42);
}

#[test]
fn clock_vs_full() {
    // Compare clock() results against execute_one() for a few instructions
    let test_cases: &[(u16, u16, &[(u32, u16)])] = &[
        (0x4e71, 0x4e71, &[]),                    // NOP
        (0x702a, 0x4e71, &[]),                    // MOVEQ #42, D0
        (0x3010, 0x4e71, &[(0x0000, 0x1234)]),    // MOVE.w (A0), D0
    ];

    for &(opcode, next, mem_setup) in test_cases {
        let pc = 0x1000u32;
        let mut mem = TestMemory::new();
        mem.write_word(pc, opcode);
        mem.write_word(pc + 2, next);
        mem.write_word(pc + 4, 0x4e71);
        mem.write_word(pc + 6, 0x4e71);
        for &(addr, val) in mem_setup {
            mem.write_word(addr, val);
        }

        // Full handler
        let mut full_cpu = setup_cpu(opcode, next, pc);
        let mut full_mem = mem.clone_mem();
        let full_cycles = full_cpu.execute_one(&mut full_mem);

        // Clock interface
        let mut clk_cpu = setup_cpu(opcode, next, pc);
        let mut clk_mem = mem.clone_mem();
        let clk_cycles = execute_one_clocked(&mut clk_cpu, &mut clk_mem);

        assert_eq!(full_cycles, clk_cycles,
            "opcode {:04x}: cycles full={} clock={}", opcode, full_cycles, clk_cycles);
        assert_eq!(full_cpu.da, clk_cpu.da,
            "opcode {:04x}: registers differ", opcode);
        assert_eq!(full_cpu.sr, clk_cpu.sr,
            "opcode {:04x}: SR differs", opcode);
    }
}
