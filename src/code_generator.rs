use scanner;
extern crate num_traits;
use num_traits::FromPrimitive;

pub struct CodeGenerator {
    bytes: Vec<u8>,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator { bytes: vec![] }
    }

    fn to_decimal(&self, value: &str) -> u8 {
        let slice_len = value.len() - 1;
        let hex_str = &value[1..slice_len];
        u8::from_str_radix(hex_str, 16).unwrap()
    }

    pub fn start(&mut self, tokens: &Vec<(scanner::Token, String)>) -> &Vec<u8> {
        // TODO: improve code generation
        // zion-vm cannot handle more than 255 bytes        
        self.bytes = Vec::with_capacity(255);

        // iterate over tokens, and generate code
        for &(ref id, ref value) in tokens.iter() {            
            //println!("{:?} {}",id, value);
            match *id {                
                scanner::Token::InstructionId => match value.as_str() {
                    "mov" => self.bytes.push(Id::Mov as u8),
                    "stp" => self.bytes.push(Id::Stp as u8),
                    _ => {}
                },
                scanner::Token::Reg => match value.as_str() {
                    "a" => self.bytes.push(Registers::A as u8),
                    "b" => self.bytes.push(Registers::B as u8),
                    "c" => self.bytes.push(Registers::C as u8),
                    "d" => self.bytes.push(Registers::D as u8),
                    "e" => self.bytes.push(Registers::E as u8),
                    "f" => self.bytes.push(Registers::F as u8),
                    _ => {}
                },
                scanner::Token::Imm => {
                    let num = self.to_decimal(value);
                    self.bytes.push(num);
                }
                scanner::Token::Mem => {
                    let num = self.to_decimal(value);
                    self.bytes.push(num);
                }
                _ => {
                    panic!("unknown token {:?}", *id);
                }
            }
        }

        &self.bytes
    }
}


// TODO: unfortunately the crate we use to cast enum fields to u8...
//  requires to add numbers explicitly...
//  try to find another crate without this limitation
#[repr(u8)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum Registers {
    //mov A, B(RR)
    //mov A, 0xFF(R, IMM)
    //mov A, @0xFF(R, MEM)
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum Id {
    //mov A, B(RR)
    //mov A, 0xFF(R, IMM)
    //mov A, @0xFF(R, MEM)
    Stp = 0,
    Add = 1,
    Sub = 2,
    Ror = 3,
    Rol = 4,
    Inc = 5,
    Dec = 6,
    Jmp = 7,
    Jz = 8,
    Jnz = 9,
    Jc = 0xa,
    Jnc = 0xb,
    Cmp = 0xc,
    Tm1 = 0xd,
    Tm2 = 0xe,
    And = 0xf,
    Or = 0x10,
    Not = 0x11,
    Xor = 0x12,
    Mov = 0x13,
    Nop = 0x14,
    Sec = 0x15,
    Clc = 0x16,
    In = 0x17,
    Out = 0x18,
    Swap = 0x19,
}

///instruction type
#[repr(u8)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum Type {
    R = 0,     //inc A
    IMM = 1,   //jmp 0xFF
    MEM = 2,   //jmp @0xFF
    R_R = 3,   //mov A, B
    R_IMM = 4, //mov A, 0xFF
    R_MEM = 5, //mov A, @0xFF
    MEM_R = 6, //mov @0xFF, A
}
