//! SingleStepTests harness (MAME-derived tests).
//! Tests are in binary .json.bin format, decoded inline.
//! Run with: cargo test --test sst [test_name]
//! Env: SST_LIMIT=N to limit tests per file

use std::collections::HashMap;
use std::env;
use rs68k_mame::cpu::M68000;
use rs68k_mame::types::*;

// ─── Binary format parsing ────────────────────────────────────────────────────

struct TestCase {
    name: String,
    initial: ProcessorState,
    final_state: ProcessorState,
    length: u32,
}

struct ProcessorState {
    d: [u32; 8],
    a: [u32; 7],
    usp: u32, ssp: u32,
    sr: u16,
    pc: u32,  // This is m_au (next prefetch address)
    prefetch: [u16; 2],
    ram: Vec<(u32, u8)>,
}

fn read_u32(data: &[u8], ptr: &mut usize) -> u32 {
    let v = u32::from_le_bytes(data[*ptr..*ptr+4].try_into().unwrap());
    *ptr += 4;
    v
}

fn read_u16(data: &[u8], ptr: &mut usize) -> u16 {
    let v = u16::from_le_bytes(data[*ptr..*ptr+2].try_into().unwrap());
    *ptr += 2;
    v
}

fn read_u8(data: &[u8], ptr: &mut usize) -> u8 {
    let v = data[*ptr];
    *ptr += 1;
    v
}

fn read_name(data: &[u8], ptr: &mut usize) -> String {
    let _numbytes = read_u32(data, ptr);
    let magic = read_u32(data, ptr);
    assert_eq!(magic, 0x89ABCDEF);
    let strlen = read_u32(data, ptr) as usize;
    let s = String::from_utf8_lossy(&data[*ptr..*ptr+strlen]).to_string();
    *ptr += strlen;
    s
}

fn read_state(data: &[u8], ptr: &mut usize) -> ProcessorState {
    let _numbytes = read_u32(data, ptr);
    let magic = read_u32(data, ptr);
    assert_eq!(magic, 0x01234567);

    let mut d = [0u32; 8];
    for i in 0..8 { d[i] = read_u32(data, ptr); }
    let mut a = [0u32; 7];
    for i in 0..7 { a[i] = read_u32(data, ptr); }
    let usp = read_u32(data, ptr);
    let ssp = read_u32(data, ptr);
    let sr = read_u32(data, ptr) as u16;
    let pc = read_u32(data, ptr);

    let pf0 = read_u32(data, ptr) as u16;
    let pf1 = read_u32(data, ptr) as u16;

    let num_rams = read_u32(data, ptr) as usize;
    let mut ram = Vec::with_capacity(num_rams * 2);
    for _ in 0..num_rams {
        let addr = read_u32(data, ptr);
        let word = read_u16(data, ptr);
        // Store as bytes (big-endian word)
        ram.push((addr, (word >> 8) as u8));
        ram.push((addr | 1, (word & 0xFF) as u8));
    }

    ProcessorState { d, a, usp, ssp, sr, pc, prefetch: [pf0, pf1], ram }
}

fn read_transactions(data: &[u8], ptr: &mut usize) -> u32 {
    let _numbytes = read_u32(data, ptr);
    let magic = read_u32(data, ptr);
    assert_eq!(magic, 0x456789AB);
    let num_cycles = read_u32(data, ptr);
    let num_transactions = read_u32(data, ptr) as usize;
    for _ in 0..num_transactions {
        let tw = read_u8(data, ptr);
        let _cycles = read_u32(data, ptr);
        if tw != 0 {
            *ptr += 20; // fc, addr, data, UDS, LDS (5 x u32)
        }
    }
    num_cycles
}

fn read_test(data: &[u8], ptr: &mut usize) -> TestCase {
    let _numbytes = read_u32(data, ptr);
    let magic = read_u32(data, ptr);
    assert_eq!(magic, 0xABC12367);
    let name = read_name(data, ptr);
    let initial = read_state(data, ptr);
    let final_state = read_state(data, ptr);
    let length = read_transactions(data, ptr);
    TestCase { name, initial, final_state, length }
}

// ─── Memory model ─────────────────────────────────────────────────────────────

struct TestMemory {
    data: HashMap<u32, u8>,
}

impl TestMemory {
    fn from_ram(ram: &[(u32, u8)]) -> Self {
        let mut data = HashMap::new();
        for &(addr, val) in ram { data.insert(addr & 0x00ffffff, val); }
        Self { data }
    }
    fn read_byte(&self, addr: u32) -> u8 {
        *self.data.get(&(addr & 0x00ffffff)).unwrap_or(&0)
    }
    fn read_word(&self, addr: u32) -> u16 {
        let hi = self.read_byte(addr) as u16;
        let lo = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
    fn write_byte(&mut self, addr: u32, val: u8) {
        self.data.insert(addr & 0x00ffffff, val);
    }
    fn write_word(&mut self, addr: u32, val: u16) {
        self.write_byte(addr, (val >> 8) as u8);
        self.write_byte(addr.wrapping_add(1), val as u8);
    }
}

impl Bus for TestMemory {
    fn read_program(&mut self, addr: u32) -> u16 { self.read_word(addr) }
    fn read_data(&mut self, addr: u32) -> u16 { self.read_word(addr) }
    fn write_data(&mut self, addr: u32, data: u16) { self.write_word(addr, data); }
    fn write_data_byte(&mut self, addr: u32, data: u8) { self.write_byte(addr, data); }
    fn read_cpu(&mut self, addr: u32) -> u16 { self.read_word(addr) }
}

// ─── CPU setup ────────────────────────────────────────────────────────────────

fn setup_cpu(ps: &ProcessorState) -> M68000 {
    let mut cpu = M68000::new();

    for i in 0..8 { cpu.da[i] = ps.d[i]; }
    for i in 0..7 { cpu.da[8 + i] = ps.a[i]; }
    cpu.da[15] = ps.usp;
    cpu.da[16] = ps.ssp;

    cpu.sr = ps.sr;
    cpu.isr = ps.sr & 0x1f;
    cpu.sp = if ps.sr & SR_S != 0 { 16 } else { 15 };

    // prefetch[0] = IR = IRD, prefetch[1] = IRC
    cpu.ir = ps.prefetch[0];
    cpu.ird = ps.prefetch[0];
    cpu.irc = ps.prefetch[1];
    cpu.dbin = ps.prefetch[1];

    // pc field = m_au (next prefetch address)
    // Internal: au = pc_field, pc = au - 2, formal instruction addr = au - 4
    cpu.au = ps.pc;
    cpu.pc = ps.pc.wrapping_sub(2);

    cpu.set_ftu_const();
    cpu.inst_state = cpu.decode_table[cpu.ird as usize];
    cpu.icount = 10000;

    cpu
}

// ─── Comparison ───────────────────────────────────────────────────────────────

fn compare_state(cpu: &M68000, mem: &TestMemory, expected: &ProcessorState, name: &str) -> Vec<String> {
    let mut errors = Vec::new();

    for i in 0..8 {
        if cpu.da[i] != expected.d[i] {
            errors.push(format!("{}: D{} got {:08x} want {:08x}", name, i, cpu.da[i], expected.d[i]));
        }
    }
    for i in 0..7 {
        if cpu.da[8+i] != expected.a[i] {
            errors.push(format!("{}: A{} got {:08x} want {:08x}", name, i, cpu.da[8+i], expected.a[i]));
        }
    }
    if cpu.da[15] != expected.usp { errors.push(format!("{}: USP got {:08x} want {:08x}", name, cpu.da[15], expected.usp)); }
    if cpu.da[16] != expected.ssp { errors.push(format!("{}: SSP got {:08x} want {:08x}", name, cpu.da[16], expected.ssp)); }
    if cpu.sr != expected.sr { errors.push(format!("{}: SR got {:04x} want {:04x}", name, cpu.sr, expected.sr)); }

    // PC comparison: expected.pc = m_au
    if cpu.au != expected.pc {
        errors.push(format!("{}: AU/PC got {:08x} want {:08x}", name, cpu.au, expected.pc));
    }

    // Prefetch
    if cpu.ir != expected.prefetch[0] { errors.push(format!("{}: IR got {:04x} want {:04x}", name, cpu.ir, expected.prefetch[0])); }
    if cpu.irc != expected.prefetch[1] { errors.push(format!("{}: IRC got {:04x} want {:04x}", name, cpu.irc, expected.prefetch[1])); }

    // RAM
    for &(addr, val) in &expected.ram {
        let got = mem.read_byte(addr);
        if got != val { errors.push(format!("{}: RAM[{:06x}] got {:02x} want {:02x}", name, addr, got, val)); }
    }

    errors
}

// ─── Test runner ──────────────────────────────────────────────────────────────

fn run_test(tc: &TestCase) -> Vec<String> {
    let mut cpu = setup_cpu(&tc.initial);
    let mut mem = TestMemory::from_ram(&tc.initial.ram);
    let cycles = cpu.execute_one(&mut mem);
    let mut errors = compare_state(&cpu, &mem, &tc.final_state, &tc.name);
    if cycles != tc.length {
        errors.push(format!("{}: cycles got {} want {}", tc.name, cycles, tc.length));
    }
    errors
}

fn run_test_file(filename: &str) {
    let limit: usize = env::var("SST_LIMIT").ok().and_then(|v| v.parse().ok()).unwrap_or(usize::MAX);
    let path = format!("external/SingleStepTests/v1/{}.json.bin", filename);
    let data = std::fs::read(&path).unwrap_or_else(|e| panic!("Cannot open {}: {}", path, e));

    let mut ptr = 0;
    let magic = read_u32(&data, &mut ptr);
    assert_eq!(magic, 0x1A3F5D71);
    let num_tests = read_u32(&data, &mut ptr) as usize;

    let total = num_tests.min(limit);
    let mut pass = 0usize;
    let mut fail = 0usize;
    let mut fail_msgs: Vec<String> = Vec::new();

    for i in 0..total {
        let tc = read_test(&data, &mut ptr);
        let errors = run_test(&tc);
        if errors.is_empty() {
            pass += 1;
        } else {
            fail += 1;
            if fail_msgs.len() < 20 {
                for e in &errors { fail_msgs.push(e.clone()); }
            }
        }
        // Skip remaining tests in file if we hit limit
        if i + 1 >= total && total < num_tests {
            break;
        }
    }

    if fail > 0 {
        eprintln!("\n=== {} FAILED: {}/{} passed ===", filename, pass, total);
        for msg in &fail_msgs { eprintln!("  {}", msg); }
        if fail > 20 { eprintln!("  ... and {} more failures", fail - 20); }
        panic!("{}: {}/{} tests failed", filename, fail, total);
    } else {
        eprintln!("{}: {}/{} passed", filename, pass, total);
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[test] fn sst_nop() { run_test_file("NOP"); }
#[test] fn sst_move_q() { run_test_file("MOVE.q"); }
#[test] fn sst_add_b() { run_test_file("ADD.b"); }
#[test] fn sst_add_w() { run_test_file("ADD.w"); }
#[test] fn sst_add_l() { run_test_file("ADD.l"); }
