// CSPR - current program status register
// N - negative
// Z - zero
// C - carry
// V - overflow
// Q - underflow

const MASK_NEGATIVE: u32 =  0b1000_0000_0000_0000_0000_0000_0000_0000;
const MASK_ZERO: u32 =      0b0100_0000_0000_0000_0000_0000_0000_0000;
const MASK_CARRY: u32 =     0b0010_0000_0000_0000_0000_0000_0000_0000;
const MASK_OVERFLOW: u32 =  0b0001_0000_0000_0000_0000_0000_0000_0000;
const MASK_UNDERFLOW: u32 = 0b0000_1000_0000_0000_0000_0000_0000_0000;

// Condition flags
pub const COND_EQ: u32 = 0x00_00_00_00;
pub const COND_NE: u32 = 0x00_00_00_10;
pub const COND_CS: u32 = 0x00_00_00_20;
pub const COND_CC: u32 = 0x00_00_00_30;
pub const COND_MI: u32 = 0x00_00_00_40;
pub const COND_PL: u32 = 0x00_00_00_50;
pub const COND_VS: u32 = 0x00_00_00_60;
pub const COND_VC: u32 = 0x00_00_00_70;
pub const COND_HI: u32 = 0x00_00_00_80;
pub const COND_LS: u32 = 0x00_00_00_90;
pub const COND_GE: u32 = 0x00_00_00_A0;
pub const COND_LT: u32 = 0x00_00_00_B0;
pub const COND_GT: u32 = 0x00_00_00_C0;
pub const COND_LE: u32 = 0x00_00_00_D0;
pub const COND_AL: u32 = 0x00_00_00_E0;

#[derive(Default)]
pub struct Registers {
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

impl Registers {
    fn new() -> Self {
        Default::default()
    }

    // Check condition flags
    fn is_negative_set(&self) -> bool {
        return self.cpsr & MASK_NEGATIVE > 0;
    }

    fn is_zero_set(&self) -> bool {
        return self.cpsr & MASK_ZERO > 0;
    }

    fn is_carry_set(&self) -> bool {
        return self.cpsr & MASK_CARRY > 0;
    }

    fn is_overflow_set(&self) -> bool {
        return self.cpsr & MASK_OVERFLOW > 0;
    }

    fn is_underflow_set(&self) -> bool {
        return self.cpsr & MASK_UNDERFLOW > 0;
    }

    // Set condition flags
    pub fn set_negative(&mut self) {
        self.cpsr = self.cpsr | MASK_NEGATIVE;
    }

    pub fn set_zero(&mut self) {
        self.cpsr = self.cpsr | MASK_ZERO;
    }

    pub fn set_carry(&mut self) {
        self.cpsr = self.cpsr | MASK_CARRY;
    }

    pub fn set_overflow(&mut self) {
        self.cpsr = self.cpsr | MASK_OVERFLOW;
    }

    pub fn set_underflow(&mut self) {
        self.cpsr = self.cpsr | MASK_UNDERFLOW;
    }

    // clear condition flags
    fn clear_negative(&mut self) {
        self.cpsr = self.cpsr & !MASK_NEGATIVE;
    }

    fn clear_zero(&mut self) {
        self.cpsr = self.cpsr & !MASK_ZERO;
    }

    fn clear_carry(&mut self) {
        self.cpsr = self.cpsr & !MASK_CARRY;
    }

    fn clear_overflow(&mut self) {
        self.cpsr = self.cpsr & !MASK_OVERFLOW;
    }

    fn clear_underflow(&mut self) {
        self.cpsr = self.cpsr & !MASK_UNDERFLOW;
    }
}

pub fn process_condition(cond: u32, registers: &Registers) -> bool {
    let execute: bool = match cond {
        COND_EQ => registers.is_zero_set(),
        COND_NE => !registers.is_zero_set(),
        COND_CS => registers.is_carry_set(),
        COND_CC => !registers.is_carry_set(),
        COND_MI => registers.is_negative_set(),
        COND_PL => !registers.is_negative_set(),
        COND_VS => registers.is_overflow_set(),
        COND_VC => !registers.is_overflow_set(),
        COND_HI => registers.is_carry_set() && !registers.is_zero_set(),
        COND_LS => !registers.is_carry_set() || registers.is_zero_set(),
        COND_GE => registers.is_negative_set() == registers.is_overflow_set(),
        COND_LT => registers.is_negative_set() != registers.is_overflow_set(),
        COND_GT => !registers.is_zero_set() && (registers.is_negative_set() == registers.is_overflow_set()),
        COND_LE => registers.is_zero_set() || (registers.is_negative_set() != registers.is_overflow_set()),
        COND_AL => true,
        other @ _ => panic!("Illegal condition {other}")
    };
    return execute;
}

#[cfg(test)]
mod tests {
    use super::*;

    // EQ - equal
    // True when Z set
    #[test]
    fn condition_equal_true() {
        let condition = COND_EQ;

        let mut registers = Registers::default();
        registers.set_zero();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }
    
    #[test]
    fn condition_equal_false() {
        let condition = COND_EQ;

        let mut registers = Registers::default();
        registers.clear_zero();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // NE - not equal
    // True when Z clear
    #[test]
    fn condition_not_equal_true() {
        let condition = COND_NE;

        let mut registers = Registers::default();
        registers.clear_zero();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }

    #[test]
    fn condition_not_equal_false() {
        let condition = COND_NE;

        let mut registers = Registers::default();
        registers.set_zero();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // CS - unsigned char of same
    // True when C set
    #[test]
    fn condition_carry_set_true() {
        let condition = COND_CS;

        let mut registers = Registers::default();
        registers.set_carry();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }

    #[test]
    fn condition_carry_set_false() {
        let condition = COND_CS;

        let mut registers = Registers::default();
        registers.clear_carry();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // CC - unsigned lower
    // True when C clear
    #[test]
    fn condition_carry_clear_true() {
        let condition = COND_CC;

        let mut registers = Registers::default();
        registers.clear_carry();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }

    #[test]
    fn condition_carry_clear_false() {
        let condition = COND_CC;

        let mut registers = Registers::default();
        registers.set_carry();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // MI - negative
    // True when N set
    #[test]
    fn condition_negative_set_true() {
        let condition = COND_MI;

        let mut registers = Registers::default();
        registers.set_negative();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }

    #[test]
    fn condition_negative_set_false() {
        let condition = COND_MI;

        let mut registers = Registers::default();
        registers.clear_negative();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // PL - positive or zero
    // True when N clear
    #[test]
    fn condition_negative_clear_true() {
        let condition = COND_PL;

        let mut registers = Registers::default();
        registers.clear_negative();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }

    #[test]
    fn condition_negative_clear_false() {
        let condition = COND_PL;

        let mut registers = Registers::default();
        registers.set_negative();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // VS - overflow
    // True when V set
    #[test]
    fn condition_overflow_set_true() {
        let condition = COND_VS;

        let mut registers = Registers::default();
        registers.set_overflow();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }

    #[test]
    fn condition_overflow_set_false() {
        let condition = COND_VS;

        let mut registers = Registers::default();
        registers.clear_overflow();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // VC - no overflow
    // True when V clear
    #[test]
    fn condition_overflow_clear_true() {
        let condition = COND_VC;

        let mut registers = Registers::default();
        registers.clear_overflow();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }

    #[test]
    fn condition_overflow_clear_false() {
        let condition = COND_VC;

        let mut registers = Registers::default();
        registers.set_overflow();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // HI - unsigned char
    // True when C set and Z clear
    #[test]
    fn condition_hi_true() {
        let condition = COND_HI;

        let mut registers = Registers::default();
        registers.set_carry();
        registers.clear_zero();

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }

    #[test]
    fn condition_hi_false_1() {
        let condition = COND_HI;

        let mut registers = Registers::default();
        registers.set_carry();
        registers.set_zero();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    #[test]
    fn condition_hi_false_2() {
        let condition = COND_HI;

        let mut registers = Registers::default();
        registers.clear_carry();
        registers.set_zero();

        let result = process_condition(condition, &registers);

        assert_eq!(result, false);
    }

    // LS - unsigned lower or same

    // GE - greater or equal

    // LT - less than

    // GT - greater than

    // LE - less than or equal

    // AL - always
    #[test]
    fn condition_always() {
        let registers = Registers::default();
        registers.is_negative_set();
        let condition = COND_AL;

        let result = process_condition(condition, &registers);

        assert_eq!(result, true);
    }
}