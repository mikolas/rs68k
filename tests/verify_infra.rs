use rs68k::cpu::M68000;
use rs68k::types::*;

#[test]
fn test_alu_add_overflow() {
    let mut cpu = M68000::new();
    cpu.alu_add(0x0001, 0x7FFF);
    assert_eq!(cpu.aluo, 0x8000);
    assert!(cpu.isr & SR_N != 0, "N should be set");
    assert!(cpu.isr & SR_V != 0, "V should be set");
    assert!(cpu.isr & SR_C == 0, "C should be clear");
}

#[test]
fn test_alu_add_carry() {
    let mut cpu = M68000::new();
    cpu.alu_add(0x0001, 0xFFFF);
    assert_eq!(cpu.aluo, 0x0000);
    assert!(cpu.isr & SR_Z != 0, "Z should be set");
    assert!(cpu.isr & SR_C != 0, "C should be set");
    assert!(cpu.isr & SR_X != 0, "X should be set");
}

#[test]
fn test_alu_sub_overflow() {
    let mut cpu = M68000::new();
    cpu.alu_sub(0x0001, 0x8000);
    assert_eq!(cpu.aluo, 0x7FFF);
    assert!(cpu.isr & SR_V != 0, "V should be set");
    assert!(cpu.isr & SR_N == 0, "N should be clear");
}

#[test]
fn test_alu_and_zero() {
    let mut cpu = M68000::new();
    cpu.sr = 0;
    cpu.alu_and(0xFF00, 0x00FF);
    assert_eq!(cpu.aluo, 0x0000);
    assert!(cpu.isr & SR_Z != 0, "Z should be set");
}

#[test]
fn test_decode_table() {
    let cpu = M68000::new();
    assert_eq!(cpu.decode_table.len(), 0x10000);
    assert_ne!(cpu.decode_table[0x4e71], S_ILLEGAL, "NOP should be valid");
    assert_ne!(cpu.decode_table[0x4e75], S_ILLEGAL, "RTS should be valid");
    assert_eq!(cpu.decode_table[0xFFFF], S_ILLEGAL, "0xFFFF should be illegal");
}

#[test]
fn test_set_ftu_const() {
    let mut cpu = M68000::new();
    
    // MOVEQ #5
    cpu.ird = 0x7005;
    cpu.set_ftu_const();
    assert_eq!(cpu.ftu, 5);
    
    // MOVEQ #-5
    cpu.ird = 0x70FB;
    cpu.set_ftu_const();
    assert_eq!(cpu.ftu, 0xFFFB);
    
    // ADDQ #1
    cpu.ird = 0x5200;
    cpu.set_ftu_const();
    assert_eq!(cpu.ftu, 1);
    
    // ADDQ #8 (encoded as 0)
    cpu.ird = 0x5000;
    cpu.set_ftu_const();
    assert_eq!(cpu.ftu, 8);
    
    // Bcc with 8-bit displacement 0x10
    cpu.ird = 0x6010;
    cpu.set_ftu_const();
    assert_eq!(cpu.ftu, 0x0010);
    
    // Bcc with negative 8-bit displacement 0xF0 = -16
    cpu.ird = 0x60F0;
    cpu.set_ftu_const();
    assert_eq!(cpu.ftu, 0xFFF0);
}

#[test]
fn test_alu_asr() {
    let mut cpu = M68000::new();
    cpu.alu_asr(0x8002);
    assert_eq!(cpu.aluo, 0xC001);
    assert!(cpu.isr & SR_N != 0);
    assert!(cpu.isr & SR_Z == 0);
    assert!(cpu.isr & SR_C == 0); // bit 0 was 0
}

#[test]
fn test_alu_lsl() {
    let mut cpu = M68000::new();
    cpu.alu_lsl(0x8001);
    assert_eq!(cpu.aluo, 0x0002);
    assert!(cpu.isr & SR_C != 0); // bit 15 was 1
    assert!(cpu.isr & SR_X != 0);
    assert!(cpu.isr & SR_Z == 0);
}

#[test]
fn test_update_interrupt() {
    let mut cpu = M68000::new();
    cpu.sr = 0x2000; // supervisor, IPL=0
    cpu.int_level = 3;
    cpu.update_interrupt();
    assert_ne!(cpu.int_next_state, 0, "should trigger interrupt");
    assert_eq!(cpu.int_next_state & 0xFF, S_INTERRUPT as u32);
    assert_eq!(cpu.int_next_state >> 24, 3);
    
    // Masked
    cpu.sr = 0x2300; // supervisor, IPL=3
    cpu.update_interrupt();
    assert_eq!(cpu.int_next_state, 0, "should be masked");
    
    // Level 4 > IPL 3
    cpu.int_level = 4;
    cpu.update_interrupt();
    assert_ne!(cpu.int_next_state, 0, "level 4 > mask 3");
}
