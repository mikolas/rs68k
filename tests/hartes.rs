use std::collections::HashMap;
use std::env;
use std::io::Read;
use flate2::read::GzDecoder;
use serde::Deserialize;

use rs68k_mame::cpu::M68000;
use rs68k_mame::types::*;

#[derive(Deserialize)]
struct TestCase {
    name: String,
    initial: ProcessorState,
    #[serde(rename = "final")]
    final_state: ProcessorState,
    length: u32,
}

#[derive(Deserialize)]
struct ProcessorState {
    d0: u32, d1: u32, d2: u32, d3: u32, d4: u32, d5: u32, d6: u32, d7: u32,
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
    usp: u32, ssp: u32,
    sr: u16,
    pc: u32,
    prefetch: [u16; 2],
    ram: Vec<[u32; 2]>,
}

struct TestMemory {
    data: HashMap<u32, u8>,
}

impl TestMemory {
    fn from_ram(ram: &[[u32; 2]]) -> Self {
        let mut data = HashMap::new();
        for entry in ram {
            data.insert(entry[0], entry[1] as u8);
        }
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
    fn read_program(&mut self, addr: u32) -> u16 {
        self.read_word(addr)
    }
    fn read_data(&mut self, addr: u32) -> u16 {
        self.read_word(addr)
    }
    fn write_data(&mut self, addr: u32, data: u16) {
        self.write_word(addr, data);
    }
    fn write_data_byte(&mut self, addr: u32, data: u8) {
        self.write_byte(addr, data);
    }
    fn read_cpu(&mut self, addr: u32) -> u16 {
        self.read_word(addr)
    }
}

fn setup_cpu(ps: &ProcessorState) -> M68000 {
    let mut cpu = M68000::new();

    cpu.da[0] = ps.d0; cpu.da[1] = ps.d1; cpu.da[2] = ps.d2; cpu.da[3] = ps.d3;
    cpu.da[4] = ps.d4; cpu.da[5] = ps.d5; cpu.da[6] = ps.d6; cpu.da[7] = ps.d7;
    cpu.da[8] = ps.a0; cpu.da[9] = ps.a1; cpu.da[10] = ps.a2; cpu.da[11] = ps.a3;
    cpu.da[12] = ps.a4; cpu.da[13] = ps.a5; cpu.da[14] = ps.a6;
    cpu.da[15] = ps.usp;
    cpu.da[16] = ps.ssp;

    cpu.sr = ps.sr;
    cpu.isr = ps.sr & 0x1f;  // ISR holds CCR bits from previous instruction's ALU result
    cpu.sp = if ps.sr & SR_S != 0 { 16 } else { 15 };

    // Prefetch: [0] = IR = IRD, [1] = IRC
    cpu.ir = ps.prefetch[0];
    cpu.ird = ps.prefetch[0];
    cpu.irc = ps.prefetch[1];
    cpu.dbin = ps.prefetch[1];

    // PC: internal PC = formal_pc + 2 (past the opcode word)
    cpu.pc = ps.pc.wrapping_add(2);
    cpu.au = ps.pc.wrapping_add(4);

    // FTU from IRD
    cpu.set_ftu_const();

    // Decode to get inst_state
    cpu.inst_state = cpu.decode_table[cpu.ird as usize];

    // icount budget (large enough for any instruction)
    cpu.icount = 1000;

    cpu
}

fn compare_state(cpu: &M68000, mem: &TestMemory, expected: &ProcessorState, name: &str) -> Vec<String> {
    let mut errors = Vec::new();

    let ed = [expected.d0, expected.d1, expected.d2, expected.d3,
              expected.d4, expected.d5, expected.d6, expected.d7];
    for i in 0..8 {
        if cpu.da[i] != ed[i] {
            errors.push(format!("{}: D{} got {:08x} want {:08x}", name, i, cpu.da[i], ed[i]));
        }
    }

    let ea = [expected.a0, expected.a1, expected.a2, expected.a3,
              expected.a4, expected.a5, expected.a6];
    for i in 0..7 {
        if cpu.da[8 + i] != ea[i] {
            errors.push(format!("{}: A{} got {:08x} want {:08x}", name, i, cpu.da[8+i], ea[i]));
        }
    }

    if cpu.da[15] != expected.usp {
        errors.push(format!("{}: USP got {:08x} want {:08x}", name, cpu.da[15], expected.usp));
    }
    if cpu.da[16] != expected.ssp {
        errors.push(format!("{}: SSP got {:08x} want {:08x}", name, cpu.da[16], expected.ssp));
    }

    if cpu.sr != expected.sr {
        errors.push(format!("{}: SR got {:04x} want {:04x}", name, cpu.sr, expected.sr));
    }

    // PC: formal PC = cpu.pc - 2 (our internal pc points past current opcode fetch)
    // At end of instruction, pc points to next instruction + 2 (past its first word)
    // Formal PC in test = address of next instruction = cpu.pc - 2
    let formal_pc = cpu.pc.wrapping_sub(2);
    if formal_pc != expected.pc {
        errors.push(format!("{}: PC got {:08x} want {:08x}", name, formal_pc, expected.pc));
    }

    // Prefetch
    if cpu.ir != expected.prefetch[0] {
        errors.push(format!("{}: IR got {:04x} want {:04x}", name, cpu.ir, expected.prefetch[0]));
    }
    if cpu.irc != expected.prefetch[1] {
        errors.push(format!("{}: IRC got {:04x} want {:04x}", name, cpu.irc, expected.prefetch[1]));
    }

    // RAM
    for entry in &expected.ram {
        let addr = entry[0];
        let val = entry[1] as u8;
        let got = mem.read_byte(addr);
        if got != val {
            errors.push(format!("{}: RAM[{:06x}] got {:02x} want {:02x}", name, addr, got, val));
        }
    }

    errors
}

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
    let limit: usize = env::var("HARTES_LIMIT").ok().and_then(|v| v.parse().ok()).unwrap_or(usize::MAX);

    let path = format!("external/ProcessorTests/680x0/68000/v1/{}.json.gz", filename);
    let file = std::fs::File::open(&path).unwrap_or_else(|e| panic!("Cannot open {}: {}", path, e));
    let mut decoder = GzDecoder::new(file);
    let mut json = String::new();
    decoder.read_to_string(&mut json).unwrap();
    let tests: Vec<TestCase> = serde_json::from_str(&json).unwrap();

    let total = tests.len().min(limit);
    let mut pass = 0usize;
    let mut fail = 0usize;
    let mut fail_msgs: Vec<String> = Vec::new();

    for tc in tests.iter().take(limit) {
        let errors = run_test(tc);
        if errors.is_empty() {
            pass += 1;
        } else {
            fail += 1;
            if fail_msgs.len() < 20 {
                for e in &errors { fail_msgs.push(e.clone()); }
            }
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

#[test] fn test_nop() { run_test_file("NOP"); }
#[test] fn test_move_q() { run_test_file("MOVE.q"); }
#[test] fn test_exg() { run_test_file("EXG"); }
#[test] fn test_add_b() { run_test_file("ADD.b"); }
#[test] fn test_add_w() { run_test_file("ADD.w"); }
#[test] fn test_add_l() { run_test_file("ADD.l"); }
#[test] fn test_sub_b() { run_test_file("SUB.b"); }
#[test] fn test_sub_w() { run_test_file("SUB.w"); }
#[test] fn test_sub_l() { run_test_file("SUB.l"); }
#[test] fn test_and_b() { run_test_file("AND.b"); }
#[test] fn test_and_w() { run_test_file("AND.w"); }
#[test] fn test_and_l() { run_test_file("AND.l"); }
#[test] fn test_or_b() { run_test_file("OR.b"); }
#[test] fn test_or_w() { run_test_file("OR.w"); }
#[test] fn test_or_l() { run_test_file("OR.l"); }
#[test] fn test_eor_b() { run_test_file("EOR.b"); }
#[test] fn test_eor_w() { run_test_file("EOR.w"); }
#[test] fn test_eor_l() { run_test_file("EOR.l"); }
#[test] fn test_cmp_b() { run_test_file("CMP.b"); }
#[test] fn test_cmp_w() { run_test_file("CMP.w"); }
#[test] fn test_cmp_l() { run_test_file("CMP.l"); }
#[test] fn test_bcc() { run_test_file("Bcc"); }
#[test] fn test_lea() { run_test_file("LEA"); }
#[test] fn test_move_b() { run_test_file("MOVE.b"); }
#[test] fn test_move_w() { run_test_file("MOVE.w"); }
#[test] fn test_move_l() { run_test_file("MOVE.l"); }
#[test] fn test_jmp() { run_test_file("JMP"); }
#[test] fn test_jsr() { run_test_file("JSR"); }
#[test] fn test_rts() { run_test_file("RTS"); }
#[test] fn test_bsr() { run_test_file("BSR"); }
#[test] fn test_tst_b() { run_test_file("TST.b"); }
#[test] fn test_tst_w() { run_test_file("TST.w"); }
#[test] fn test_tst_l() { run_test_file("TST.l"); }
#[test] fn test_movem_w() { run_test_file("MOVEM.w"); }
#[test] fn test_movem_l() { run_test_file("MOVEM.l"); }
#[test] fn test_mulu() { run_test_file("MULU"); }
#[test] fn test_muls() { run_test_file("MULS"); }
#[test] fn test_divu() { run_test_file("DIVU"); }
#[test] fn test_divs() { run_test_file("DIVS"); }
#[test] fn test_swap() { run_test_file("SWAP"); }
#[test] fn test_ext_w() { run_test_file("EXT.w"); }
#[test] fn test_ext_l() { run_test_file("EXT.l"); }
#[test] fn test_neg_b() { run_test_file("NEG.b"); }
#[test] fn test_neg_w() { run_test_file("NEG.w"); }
#[test] fn test_neg_l() { run_test_file("NEG.l"); }
#[test] fn test_not_b() { run_test_file("NOT.b"); }
#[test] fn test_not_w() { run_test_file("NOT.w"); }
#[test] fn test_not_l() { run_test_file("NOT.l"); }
#[test] fn test_clr_b() { run_test_file("CLR.b"); }
#[test] fn test_clr_w() { run_test_file("CLR.w"); }
#[test] fn test_clr_l() { run_test_file("CLR.l"); }
#[test] fn test_asl_b() { run_test_file("ASL.b"); }
#[test] fn test_asl_w() { run_test_file("ASL.w"); }
#[test] fn test_asl_l() { run_test_file("ASL.l"); }
#[test] fn test_asr_b() { run_test_file("ASR.b"); }
#[test] fn test_asr_w() { run_test_file("ASR.w"); }
#[test] fn test_asr_l() { run_test_file("ASR.l"); }
#[test] fn test_lsl_b() { run_test_file("LSL.b"); }
#[test] fn test_lsl_w() { run_test_file("LSL.w"); }
#[test] fn test_lsl_l() { run_test_file("LSL.l"); }
#[test] fn test_lsr_b() { run_test_file("LSR.b"); }
#[test] fn test_lsr_w() { run_test_file("LSR.w"); }
#[test] fn test_lsr_l() { run_test_file("LSR.l"); }
#[test] fn test_rol_b() { run_test_file("ROL.b"); }
#[test] fn test_rol_w() { run_test_file("ROL.w"); }
#[test] fn test_rol_l() { run_test_file("ROL.l"); }
#[test] fn test_ror_b() { run_test_file("ROR.b"); }
#[test] fn test_ror_w() { run_test_file("ROR.w"); }
#[test] fn test_ror_l() { run_test_file("ROR.l"); }
#[test] fn test_roxl_b() { run_test_file("ROXL.b"); }
#[test] fn test_roxl_w() { run_test_file("ROXL.w"); }
#[test] fn test_roxl_l() { run_test_file("ROXL.l"); }
#[test] fn test_roxr_b() { run_test_file("ROXR.b"); }
#[test] fn test_roxr_w() { run_test_file("ROXR.w"); }
#[test] fn test_roxr_l() { run_test_file("ROXR.l"); }
#[test] fn test_dbcc() { run_test_file("DBcc"); }
#[test] fn test_scc() { run_test_file("Scc"); }
#[test] fn test_addx_b() { run_test_file("ADDX.b"); }
#[test] fn test_addx_w() { run_test_file("ADDX.w"); }
#[test] fn test_addx_l() { run_test_file("ADDX.l"); }
#[test] fn test_subx_b() { run_test_file("SUBX.b"); }
#[test] fn test_subx_w() { run_test_file("SUBX.w"); }
#[test] fn test_subx_l() { run_test_file("SUBX.l"); }
#[test] fn test_abcd() { run_test_file("ABCD"); }
#[test] fn test_sbcd() { run_test_file("SBCD"); }
#[test] fn test_nbcd() { run_test_file("NBCD"); }
#[test] fn test_link() { run_test_file("LINK"); }
#[test] fn test_unlink() { run_test_file("UNLINK"); }
#[test] fn test_pea() { run_test_file("PEA"); }
#[test] fn test_trap() { run_test_file("TRAP"); }
#[test] fn test_trapv() { run_test_file("TRAPV"); }
#[test] fn test_rtr() { run_test_file("RTR"); }
#[test] fn test_rte() { run_test_file("RTE"); }
#[test] fn test_chk() { run_test_file("CHK"); }
#[test] fn test_tas() { run_test_file("TAS"); }
#[test] fn test_movea_w() { run_test_file("MOVEA.w"); }
#[test] fn test_movea_l() { run_test_file("MOVEA.l"); }
#[test] fn test_adda_w() { run_test_file("ADDA.w"); }
#[test] fn test_adda_l() { run_test_file("ADDA.l"); }
#[test] fn test_suba_w() { run_test_file("SUBA.w"); }
#[test] fn test_suba_l() { run_test_file("SUBA.l"); }
#[test] fn test_cmpa_w() { run_test_file("CMPA.w"); }
#[test] fn test_cmpa_l() { run_test_file("CMPA.l"); }
