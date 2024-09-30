use crate::cpu::AddressingMode;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code: code,
            mnemonic: mnemonic,
            len: len,
            cycles: cycles,
            mode: mode,
        }
    }
}

pub const BRK: u8 = 0x00;
pub const TAX: u8 = 0xAA;
pub const INX: u8 = 0xE8;

pub const LDA_I: u8 = 0xA9;
pub const LDA_Z: u8 = 0xA5;
pub const LDA_Z_X: u8 = 0xA5;
pub const LDA_A: u8 = 0xAD;
pub const LDA_A_X: u8 = 0xBD;
pub const LDA_A_Y: u8 = 0xB9;
pub const LDA_I_X: u8 = 0xA1;
pub const LDA_I_Y: u8 = 0xB1;

pub const STA_Z: u8 = 0x85;
pub const STA_Z_X: u8 = 0x95;
pub const STA_A: u8 = 0x8D;
pub const STA_A_X: u8 = 0x9D;
pub const STA_A_Y: u8 = 0x99;
pub const STA_I_X: u8 = 0x81;
pub const STA_I_Y: u8 = 0x91;

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpCode> = vec![
        OpCode::new(BRK,     "BRK", 1, 7, AddressingMode::NoneAddressing),
        OpCode::new(TAX,     "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(INX,     "INX", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(LDA_I,   "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(LDA_Z,   "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(LDA_Z_X, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(LDA_A,   "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(LDA_A_X, "LDA", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(LDA_A_Y, "LDA", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(LDA_I_X, "LDA", 3, 6, AddressingMode::Indirect_X),
        OpCode::new(LDA_I_Y, "LDA", 3, 5, AddressingMode::Indirect_Y),

        OpCode::new(STA_Z,   "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(STA_Z_X, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(STA_A,   "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(STA_A_X, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(STA_A_Y, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(STA_I_X, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(STA_I_Y, "STA", 2, 6, AddressingMode::Indirect_Y),
    ];


    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OPS_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}