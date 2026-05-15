use rs68k_mame::cpu::M68000;
use rs68k_mame::types::*;
use std::time::Instant;

struct BenchBus {
    rom: Vec<u16>,
}

impl BenchBus {
    fn new() -> Self {
        let mut rom = vec![0x4E71u16; 0x80000];
        let base = 0x800;
        rom[base] = 0x7000;     // MOVEQ #0, D0
        rom[base+1] = 0x5280;   // ADDQ.l #1, D0
        rom[base+2] = 0x60FC;   // BRA.s -4 (back to ADDQ)
        Self { rom }
    }
}

impl Bus for BenchBus {
    fn read_program(&mut self, addr: u32) -> u16 {
        self.rom[((addr >> 1) & 0x7FFFF) as usize]
    }
    fn read_data(&mut self, addr: u32) -> u16 {
        self.rom[((addr >> 1) & 0x7FFFF) as usize]
    }
    fn write_data(&mut self, _addr: u32, _data: u16) {}
    fn write_data_byte(&mut self, _addr: u32, _data: u8) {}
    fn read_cpu(&mut self, _addr: u32) -> u16 { 0 }
}

/// Execute one instruction via pin-level partial handler with instant bus completion.
fn execute_one_partial(cpu: &mut M68000, bus: &mut BenchBus) -> u32 {
    let before = cpu.icount;
    cpu.ipc = cpu.pc.wrapping_sub(2);
    cpu.irdi = cpu.ird;

    loop {
        cpu.dispatch_partial(bus);
        if cpu.inst_substate != 0 {
            // Bus access: perform externally
            if cpu.base_ssw & SSW_R != 0 {
                let space = cpu.base_ssw & 3;
                if space == 3 {
                    cpu.edb = bus.read_cpu(cpu.aob);
                } else if space == 2 {
                    cpu.edb = bus.read_program(cpu.aob & !1u32);
                } else {
                    cpu.edb = bus.read_data(cpu.aob & !1u32);
                }
            } else {
                if cpu.pin_state.pending_byte {
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
    (before - cpu.icount) as u32
}

fn main() {
    let mut cpu = M68000::new();
    let mut bus = BenchBus::new();

    cpu.sr = 0x2700;
    cpu.sp = 16;
    cpu.da[16] = 0x100000;
    cpu.pc = 0x1002;
    cpu.au = 0x1004;
    cpu.ir = 0x5280;
    cpu.ird = 0x5280;
    cpu.irc = 0x60FC;
    cpu.dbin = 0x60FC;
    cpu.set_ftu_const();
    cpu.inst_state = cpu.decode_table[cpu.ird as usize];
    cpu.icount = i32::MAX;

    // Warmup
    for _ in 0..10000 {
        execute_one_partial(&mut cpu, &mut bus);
    }

    // Benchmark partial
    let iterations = 10_000_000u64;
    let start = Instant::now();
    let mut total_cycles: u64 = 0;
    for _ in 0..iterations {
        total_cycles += execute_one_partial(&mut cpu, &mut bus) as u64;
    }
    let elapsed_partial = start.elapsed();

    let mips_p = iterations as f64 / elapsed_partial.as_secs_f64() / 1_000_000.0;
    let mhz_p = total_cycles as f64 / elapsed_partial.as_secs_f64() / 1_000_000.0;

    // Reset and benchmark full for comparison
    cpu.pc = 0x1002;
    cpu.au = 0x1004;
    cpu.ir = 0x5280;
    cpu.ird = 0x5280;
    cpu.irc = 0x60FC;
    cpu.dbin = 0x60FC;
    cpu.set_ftu_const();
    cpu.inst_state = cpu.decode_table[cpu.ird as usize];
    cpu.icount = i32::MAX;

    for _ in 0..10000 {
        cpu.execute_one(&mut bus);
    }

    let start = Instant::now();
    total_cycles = 0;
    for _ in 0..iterations {
        total_cycles += cpu.execute_one(&mut bus) as u64;
    }
    let elapsed_full = start.elapsed();

    let mips_f = iterations as f64 / elapsed_full.as_secs_f64() / 1_000_000.0;
    let mhz_f = total_cycles as f64 / elapsed_full.as_secs_f64() / 1_000_000.0;

    println!("=== Full handlers ===");
    println!("  {:.1} MIPS, {:.1} MHz effective, {:.0}x Amiga headroom", mips_f, mhz_f, mhz_f / 7.09);
    println!("  Time: {:.3}s", elapsed_full.as_secs_f64());
    println!();
    println!("=== Partial handlers (pin-level) ===");
    println!("  {:.1} MIPS, {:.1} MHz effective, {:.0}x Amiga headroom", mips_p, mhz_p, mhz_p / 7.09);
    println!("  Time: {:.3}s", elapsed_partial.as_secs_f64());
    println!();
    println!("Partial/Full ratio: {:.1}%", (mips_p / mips_f) * 100.0);
}
