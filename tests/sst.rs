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
#[test] fn sst_sub_b() { run_test_file("SUB.b"); }
#[test] fn sst_sub_w() { run_test_file("SUB.w"); }
#[test] fn sst_sub_l() { run_test_file("SUB.l"); }
#[test] fn sst_and_b() { run_test_file("AND.b"); }
#[test] fn sst_and_w() { run_test_file("AND.w"); }
#[test] fn sst_and_l() { run_test_file("AND.l"); }
#[test] fn sst_or_b() { run_test_file("OR.b"); }
#[test] fn sst_or_w() { run_test_file("OR.w"); }
#[test] fn sst_or_l() { run_test_file("OR.l"); }
#[test] fn sst_eor_b() { run_test_file("EOR.b"); }
#[test] fn sst_eor_w() { run_test_file("EOR.w"); }
#[test] fn sst_eor_l() { run_test_file("EOR.l"); }
#[test] fn sst_cmp_b() { run_test_file("CMP.b"); }
#[test] fn sst_cmp_w() { run_test_file("CMP.w"); }
#[test] fn sst_cmp_l() { run_test_file("CMP.l"); }
#[test] fn sst_move_b() { run_test_file("MOVE.b"); }
#[test] fn sst_move_w() { run_test_file("MOVE.w"); }
#[test] fn sst_move_l() { run_test_file("MOVE.l"); }
#[test] fn sst_movea_w() { run_test_file("MOVEA.w"); }
#[test] fn sst_movea_l() { run_test_file("MOVEA.l"); }
#[test] fn sst_lea() { run_test_file("LEA"); }
#[test] fn sst_pea() { run_test_file("PEA"); }
#[test] fn sst_jmp() { run_test_file("JMP"); }
#[test] fn sst_jsr() { run_test_file("JSR"); }
#[test] fn sst_rts() { run_test_file("RTS"); }
#[test] fn sst_rte() { run_test_file("RTE"); }
#[test] fn sst_rtr() { run_test_file("RTR"); }
#[test] fn sst_bcc() { run_test_file("Bcc"); }
#[test] fn sst_bsr() { run_test_file("BSR"); }
#[test] fn sst_dbcc() { run_test_file("DBcc"); }
#[test] fn sst_scc() { run_test_file("Scc"); }
#[test] fn sst_exg() { run_test_file("EXG"); }
#[test] fn sst_ext_w() { run_test_file("EXT.w"); }
#[test] fn sst_ext_l() { run_test_file("EXT.l"); }
#[test] fn sst_swap() { run_test_file("SWAP"); }
#[test] fn sst_trap() { run_test_file("TRAP"); }
#[test] fn sst_trapv() { run_test_file("TRAPV"); }
#[test] fn sst_link() { run_test_file("LINK"); }
#[test] fn sst_unlink() { run_test_file("UNLINK"); }
#[test] fn sst_movem_w() { run_test_file("MOVEM.w"); }
#[test] fn sst_movem_l() { run_test_file("MOVEM.l"); }
#[test] fn sst_mulu() { run_test_file("MULU"); }
#[test] fn sst_muls() { run_test_file("MULS"); }
#[test] fn sst_divu() { run_test_file("DIVU"); }
#[test] fn sst_divs() { run_test_file("DIVS"); }
#[test] fn sst_abcd() { run_test_file("ABCD"); }
#[test] fn sst_sbcd() { run_test_file("SBCD"); }
#[test] fn sst_nbcd() { run_test_file("NBCD"); }
#[test] fn sst_neg_b() { run_test_file("NEG.b"); }
#[test] fn sst_neg_w() { run_test_file("NEG.w"); }
#[test] fn sst_neg_l() { run_test_file("NEG.l"); }
#[test] fn sst_negx_b() { run_test_file("NEGX.b"); }
#[test] fn sst_negx_w() { run_test_file("NEGX.w"); }
#[test] fn sst_negx_l() { run_test_file("NEGX.l"); }
#[test] fn sst_not_b() { run_test_file("NOT.b"); }
#[test] fn sst_not_w() { run_test_file("NOT.w"); }
#[test] fn sst_not_l() { run_test_file("NOT.l"); }
#[test] fn sst_clr_b() { run_test_file("CLR.b"); }
#[test] fn sst_clr_w() { run_test_file("CLR.w"); }
#[test] fn sst_clr_l() { run_test_file("CLR.l"); }
#[test] fn sst_tst_b() { run_test_file("TST.b"); }
#[test] fn sst_tst_w() { run_test_file("TST.w"); }
#[test] fn sst_tst_l() { run_test_file("TST.l"); }
#[test] fn sst_tas() { run_test_file("TAS"); }
#[test] fn sst_chk() { run_test_file("CHK"); }
#[test] fn sst_asl_b() { run_test_file("ASL.b"); }
#[test] fn sst_asl_w() { run_test_file("ASL.w"); }
#[test] fn sst_asl_l() { run_test_file("ASL.l"); }
#[test] fn sst_asr_b() { run_test_file("ASR.b"); }
#[test] fn sst_asr_w() { run_test_file("ASR.w"); }
#[test] fn sst_asr_l() { run_test_file("ASR.l"); }
#[test] fn sst_lsl_b() { run_test_file("LSL.b"); }
#[test] fn sst_lsl_w() { run_test_file("LSL.w"); }
#[test] fn sst_lsl_l() { run_test_file("LSL.l"); }
#[test] fn sst_lsr_b() { run_test_file("LSR.b"); }
#[test] fn sst_lsr_w() { run_test_file("LSR.w"); }
#[test] fn sst_lsr_l() { run_test_file("LSR.l"); }
#[test] fn sst_rol_b() { run_test_file("ROL.b"); }
#[test] fn sst_rol_w() { run_test_file("ROL.w"); }
#[test] fn sst_rol_l() { run_test_file("ROL.l"); }
#[test] fn sst_ror_b() { run_test_file("ROR.b"); }
#[test] fn sst_ror_w() { run_test_file("ROR.w"); }
#[test] fn sst_ror_l() { run_test_file("ROR.l"); }
#[test] fn sst_roxl_b() { run_test_file("ROXL.b"); }
#[test] fn sst_roxl_w() { run_test_file("ROXL.w"); }
#[test] fn sst_roxl_l() { run_test_file("ROXL.l"); }
#[test] fn sst_roxr_b() { run_test_file("ROXR.b"); }
#[test] fn sst_roxr_w() { run_test_file("ROXR.w"); }
#[test] fn sst_roxr_l() { run_test_file("ROXR.l"); }
#[test] fn sst_adda_w() { run_test_file("ADDA.w"); }
#[test] fn sst_adda_l() { run_test_file("ADDA.l"); }
#[test] fn sst_suba_w() { run_test_file("SUBA.w"); }
#[test] fn sst_suba_l() { run_test_file("SUBA.l"); }
#[test] fn sst_cmpa_w() { run_test_file("CMPA.w"); }
#[test] fn sst_cmpa_l() { run_test_file("CMPA.l"); }
#[test] fn sst_addx_b() { run_test_file("ADDX.b"); }
#[test] fn sst_addx_w() { run_test_file("ADDX.w"); }
#[test] fn sst_addx_l() { run_test_file("ADDX.l"); }
#[test] fn sst_subx_b() { run_test_file("SUBX.b"); }
#[test] fn sst_subx_w() { run_test_file("SUBX.w"); }
#[test] fn sst_subx_l() { run_test_file("SUBX.l"); }
#[test] fn sst_bchg() { run_test_file("BCHG"); }
#[test] fn sst_bclr() { run_test_file("BCLR"); }
#[test] fn sst_bset() { run_test_file("BSET"); }
#[test] fn sst_btst() { run_test_file("BTST"); }
#[test] fn sst_movep_w() { run_test_file("MOVEP.w"); }
#[test] fn sst_movep_l() { run_test_file("MOVEP.l"); }
#[test] fn sst_anditoccr() { run_test_file("ANDItoCCR"); }
#[test] fn sst_anditosr() { run_test_file("ANDItoSR"); }
#[test] fn sst_eoritoccr() { run_test_file("EORItoCCR"); }
#[test] fn sst_eoritosr() { run_test_file("EORItoSR"); }
#[test] fn sst_oritoccr() { run_test_file("ORItoCCR"); }
#[test] fn sst_oritosr() { run_test_file("ORItoSR"); }
#[test] fn sst_movefromsr() { run_test_file("MOVEfromSR"); }
#[test] fn sst_movetosr() { run_test_file("MOVEtoSR"); }
#[test] fn sst_movetoccr() { run_test_file("MOVEtoCCR"); }
#[test] fn sst_movefromusp() { run_test_file("MOVEfromUSP"); }
#[test] fn sst_movetousp() { run_test_file("MOVEtoUSP"); }
#[test] fn sst_illegal_linea() { run_test_file("ILLEGAL_LINEA"); }
#[test] fn sst_illegal_linef() { run_test_file("ILLEGAL_LINEF"); }
#[test] fn sst_reset() { run_test_file("RESET"); }
#[test] fn sst_stop() { run_test_file("STOP"); }
