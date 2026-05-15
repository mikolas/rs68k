use rs68k::cpu::M68000;
use rs68k::types::*;
use std::time::Instant;

struct BenchBus {
    rom: Vec<u16>,
}

impl BenchBus {
    fn new() -> Self {
        // Fill with NOPs and a loop: BRA.s -2 (0x60FE)
        let mut rom = vec![0x4E71u16; 0x80000]; // 512K words of NOP
        // Put a tight loop at address 0x1000: MOVEQ #0,D0 / ADDQ.l #1,D0 / BRA.s -4
        let base = 0x800; // word offset
        rom[base] = 0x7000;     // MOVEQ #0, D0
        rom[base+1] = 0x5280;   // ADDQ.l #1, D0
        rom[base+2] = 0x60FC;   // BRA.s -4 (back to ADDQ)
        Self { rom }
    }
}

impl Bus for BenchBus {
    fn read_program(&mut self, addr: u32) -> u16 {
        let idx = ((addr >> 1) & 0x7FFFF) as usize;
        self.rom[idx]
    }
    fn read_data(&mut self, addr: u32) -> u16 {
        let idx = ((addr >> 1) & 0x7FFFF) as usize;
        self.rom[idx]
    }
    fn write_data(&mut self, _addr: u32, _data: u16) {}
    fn write_data_byte(&mut self, _addr: u32, _data: u8) {}
    fn read_cpu(&mut self, _addr: u32) -> u16 { 0 }
}

fn main() {
    let mut cpu = M68000::new();
    let mut bus = BenchBus::new();

    // Setup: supervisor mode, PC at the tight loop
    cpu.sr = 0x2700; // supervisor, IPL=7
    cpu.sp = 16;
    cpu.da[16] = 0x100000; // SSP
    cpu.pc = 0x1002; // past first instruction
    cpu.au = 0x1004;
    cpu.ir = 0x5280;  // ADDQ.l #1, D0
    cpu.ird = 0x5280;
    cpu.irc = 0x60FC;  // BRA.s -4
    cpu.dbin = 0x60FC;
    cpu.set_ftu_const();
    cpu.inst_state = cpu.decode_table[cpu.ird as usize];
    cpu.icount = i32::MAX;

    // Warmup
    for _ in 0..10000 {
        cpu.execute_one(&mut bus);
    }

    // Benchmark
    let iterations = 10_000_000u64;
    let start = Instant::now();
    let mut total_cycles: u64 = 0;
    for _ in 0..iterations {
        total_cycles += cpu.execute_one(&mut bus) as u64;
    }
    let elapsed = start.elapsed();

    let mips = iterations as f64 / elapsed.as_secs_f64() / 1_000_000.0;
    let mhz = total_cycles as f64 / elapsed.as_secs_f64() / 1_000_000.0;

    println!("Executed {} instructions in {:.3}s", iterations, elapsed.as_secs_f64());
    println!("Total cycles: {}", total_cycles);
    println!("Performance: {:.1} MIPS (million instructions/sec)", mips);
    println!("Effective clock: {:.1} MHz", mhz);
    println!("Amiga 68000 = 7.09 MHz → headroom: {:.0}x", mhz / 7.09);
}
