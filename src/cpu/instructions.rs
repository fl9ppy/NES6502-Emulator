use crate::bus::Bus;
use super::addressing::{AddressingMode, FetchResult};
use super::CPU;

/// Function pointer type for all instruction implementations.
/// Every instruction receives:
/// - &mut CPU
/// - &mut Bus
/// - addressing mode (which determines how to fetch operand)
pub type InstructionHandler =
    fn(&mut CPU, &mut dyn Bus, AddressingMode);

///
/// -------------------------------
/// INSTRUCTION IMPLEMENTATION STUBS
/// -------------------------------
///
/// These will be filled in later. For now they do nothing except
/// fetch the operand (if required) so the CPU does not desync.
///

pub fn instr_nop(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    // Do nothing — 1-byte instruction
}

pub fn instr_lda(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let fetch = cpu.fetch_operand(bus, mode);

    if let FetchResult::Value(v) = fetch {
        cpu.register_a = v;
        cpu.update_zn(v);
    }
}

pub fn instr_ldx(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let fetch = cpu.fetch_operand(bus, mode);

    if let FetchResult::Value(v) = fetch {
        cpu.register_x = v;
        cpu.update_zn(v);
    }
}

pub fn instr_ldy(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let fetch = cpu.fetch_operand(bus, mode);

    if let FetchResult::Value(v) = fetch {
        cpu.register_y = v;
        cpu.update_zn(v);
    }
}

pub fn instr_sta(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    if let FetchResult::Address(addr) = cpu.fetch_operand(bus, mode) {
        bus.write(addr, cpu.register_a);
    }
}

pub fn instr_stx(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    if let FetchResult::Address(addr) = cpu.fetch_operand(bus, mode) {
        bus.write(addr, cpu.register_x);
    }
}

pub fn instr_sty(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    if let FetchResult::Address(addr) = cpu.fetch_operand(bus, mode) {
        bus.write(addr, cpu.register_y);
    }
}

// Transfer registers
pub fn instr_tax(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.register_x = cpu.register_a;
    cpu.update_zn(cpu.register_x);
}

pub fn instr_tay(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.register_y = cpu.register_a;
    cpu.update_zn(cpu.register_y);
}

pub fn instr_txa(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.register_a = cpu.register_x;
    cpu.update_zn(cpu.register_a);
}

pub fn instr_tya(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.register_a = cpu.register_y;
    cpu.update_zn(cpu.register_a);
}

// INC/DEC/INX/DEX
pub fn instr_inx(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.register_x = cpu.register_x.wrapping_add(1);
    cpu.update_zn(cpu.register_x);
}

pub fn instr_iny(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.register_y = cpu.register_y.wrapping_add(1);
    cpu.update_zn(cpu.register_y);
}

pub fn instr_dex(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.register_x = cpu.register_x.wrapping_sub(1);
    cpu.update_zn(cpu.register_x);
}

pub fn instr_dey(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.register_y = cpu.register_y.wrapping_sub(1);
    cpu.update_zn(cpu.register_y);
}

// Branches (stubs — real implementation added later)
pub fn instr_beq(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let fetch = cpu.fetch_operand(bus, mode);
    if cpu.status_zero() {
        cpu.branch_relative(fetch.value_signed());
    }
}

pub fn instr_bne(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let fetch = cpu.fetch_operand(bus, mode);
    if !cpu.status_zero() {
        cpu.branch_relative(fetch.value_signed());
    }
}

pub fn instr_bcs(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let fetch = cpu.fetch_operand(bus, mode);
    if cpu.status_carry() {
        cpu.branch_relative(fetch.value_signed());
    }
}

pub fn instr_bcc(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let fetch = cpu.fetch_operand(bus, mode);
    if !cpu.status_carry() {
        cpu.branch_relative(fetch.value_signed());
    }
}

pub fn instr_jmp(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    if let FetchResult::Address(addr) = cpu.fetch_operand(bus, mode) {
        cpu.program_counter = addr;
    }
}

pub fn instr_jsr(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    if let FetchResult::Address(addr) = cpu.fetch_operand(bus, mode) {
        let return_addr = cpu.program_counter.wrapping_sub(1);
        cpu.push_word(bus, return_addr);
        cpu.program_counter = addr;
    }
}

pub fn instr_rts(cpu: &mut CPU, bus: &mut dyn Bus, _mode: AddressingMode) {
    let addr = cpu.pop_word(bus);
    cpu.program_counter = addr.wrapping_add(1);
}

pub fn instr_rti(cpu: &mut CPU, bus: &mut dyn Bus, _mode: AddressingMode) {
    cpu.status = cpu.pop_byte(bus);
    cpu.program_counter = cpu.pop_word(bus);
}

// Arithmetic (stubs)
pub fn instr_adc(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let f = cpu.fetch_operand(bus, mode);
    cpu.adc(f.value());
}

pub fn instr_sbc(cpu: &mut CPU, bus: &mut dyn Bus, mode: AddressingMode) {
    let f = cpu.fetch_operand(bus, mode);
    cpu.sbc(f.value());
}

// Placeholder for all unimplemented opcodes
pub fn instr_unimplemented(_cpu: &mut CPU, _bus: &mut dyn Bus, _mode: AddressingMode) {
    // We will fill these in gradually.
}
