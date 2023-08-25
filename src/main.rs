use core::panic;

struct Registers {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    sp: u32,
    lr: u32,
    pc: u32,
    cpsr: u32,
}

const MASK_NEGATIVE: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
const MASK_ZERO: u32 = 0b0100_0000_0000_0000_0000_0000_0000_0000;
const MASK_CARRY: u32 = 0b0010_0000_0000_0000_0000_0000_0000_0000;
const MASK_OVERFLOW: u32 = 0b0001_0000_0000_0000_0000_0000_0000_0000;
const MASK_UNDERFLOW: u32 = 0b0000_1000_0000_0000_0000_0000_0000_0000;

impl Registers {
    fn negative_set(&self) -> bool {
        return self.cpsr & MASK_NEGATIVE > 0;
    }

    fn zero_set(&self) -> bool {
        return self.cpsr & MASK_ZERO > 0;
    }

    fn carry_set(&self) -> bool {
        return self.cpsr & MASK_CARRY > 0;
    }

    fn overflow_set(&self) -> bool {
        return self.cpsr & MASK_OVERFLOW > 0;
    }

    fn underflow_set(&self) -> bool {
        return self.cpsr & MASK_UNDERFLOW > 0;
    }
}

// CSPR - current program status register
// N - negative
// Z - zero
// C - carry
// V - overflow
// Q - underflow

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

const COND_EQ: u32 = 0x00_00_00_00;
const COND_NE: u32 = 0x01_00_00_00;
const COND_CS: u32 = 0x02_00_00_00;
const COND_CC: u32 = 0x03_00_00_00;
const COND_MI: u32 = 0x04_00_00_00;
const COND_PL: u32 = 0x05_00_00_00;
const COND_VS: u32 = 0x06_00_00_00;
const COND_VC: u32 = 0x07_00_00_00;
const COND_HI: u32 = 0x08_00_00_00;
const COND_LS: u32 = 0x09_00_00_00;
const COND_GE: u32 = 0x0A_00_00_00;
const COND_LT: u32 = 0x0B_00_00_00;
const COND_GT: u32 = 0x0C_00_00_00;
const COND_LE: u32 = 0x0D_00_00_00;
const COND_AL: u32 = 0x0E_00_00_00;

fn process_condition(cond: u32, registers: &Registers) -> bool {
    let execute: bool = match cond {
        COND_EQ => registers.zero_set(),
        COND_NE => !registers.zero_set(),
        COND_CS => registers.carry_set(),
        COND_CC => !registers.carry_set(),
        COND_MI => registers.negative_set(),
        COND_PL => !registers.negative_set(),
        COND_VS => registers.overflow_set(),
        COND_VC => !registers.overflow_set(),
        COND_HI => registers.carry_set() && !registers.zero_set(),
        COND_LS => !registers.carry_set() || registers.zero_set(),
        COND_GE => registers.negative_set() == registers.overflow_set(),
        COND_LT => registers.negative_set() != registers.overflow_set(),
        COND_GT => !registers.zero_set() && (registers.negative_set() == registers.overflow_set()),
        COND_LE => registers.zero_set() || (registers.negative_set() != registers.overflow_set()),
        COND_AL => true,
        other @ _ => panic!("Illegal condition {other}")
    };
    return execute;
}

fn process_opcode(opcode: u8) {
    match opcode {
        // AND
        0b0000 => println!("AND"),

        // EOR
        0b0001 => println!("EOR"),

        // SUB
        0b0010 => println!("SUB"),

        // RSB
        0b0011 => println!("RSB"),

        // ADD
        0b0100 => println!("ADD"),

        // ADC
        0b0101 => println!("ADC"),

        // SBC
        0b0110 => println!("SBC"),

        // RSC
        0b0111 => println!("RSC"),

        // TST
        0b1000 => println!("TST"),

        // TEQ
        0b1001 => println!("TEQ"),

        // CMP
        0b1010 => println!("TMP"),

        // CMN
        0b1011 => println!("CMN"),

        // ORR
        0b1100 => println!("ORR"),

        // MOV
        0b1101 => println!("MOV"),

        // BIC
        0b1110 => println!("BIC"),

        // MVN
        0b1111 => println!("MVN"),

        _      => println!("Illegal opcode"),
    }
}

// https://developer.arm.com/documentation/dui0068/b/ARM-Instruction-Reference/ARM-general-data-processing-instructions/ADD--SUB--RSB--ADC--SBC--and-RSC?lang=en