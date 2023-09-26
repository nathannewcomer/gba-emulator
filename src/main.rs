mod cpu;

use core::panic;

fn main() {
    println!("Hello, world!");
    process_opcode(0b0000);
}

const OPERAND_2_MASK: u32   = 0b0000_0000_0000_0000_0000_1111_1111_1111;
const DESTINATION_MASK: u32 = 0b0000_0000_0000_0000_1111_0000_0000_0000;
const OPERAND_1_MASK: u32   = 0b0000_0000_0000_1111_0000_0000_0000_0000;
const SET_MASK: u32         = 0b0000_0000_0001_0000_0000_0000_0000_0000;
const OPCODE_MASK: u32      = 0b0000_0001_1110_0000_0000_0000_0000_0000;
// TODO
const CONDITION_MASK: u32   = 0b1111_0000_0000_0000_0000_0000_0000_0000;

// Data processing
fn parse_instruction(instruction: u32) {
    // cond
    let condition = instruction & CONDITION_MASK;

    // 0 0 1?

    // opcode
    let opcode = instruction & OPCODE_MASK;

    // S
    // Set instruction codes


    // Rn
    // first operand register

    // Rd
    // destination register

    // Operand 2
}

fn process_opcode(opcode: u8) {
    match opcode {
        0b0010 => println!("ADD"),

        _      => panic!("Illegal opcode \"{opcode}\""),
    }
}

// https://developer.arm.com/documentation/dui0068/b/ARM-Instruction-Reference/ARM-general-data-processing-instructions/ADD--SUB--RSB--ADC--SBC--and-RSC?lang=en