pub mod decode;
pub mod error;

use crate::emu::{self, Context};


pub enum AccessKind<'a> {
    Read(&'a MemoryAddr),
    Write(&'a MemoryAddr),
    Both(&'a MemoryAddr, &'a MemoryAddr),
}

#[derive(Debug)]
pub struct Instruction {
    pub code: Code,
    pub ops: Vec<Operand>,
    pub size: usize,
    pub rex: Option<u8>,
}

impl Instruction {
    pub fn prefixes(&self) -> Vec<u8> {
        self.rex.map(|rex| vec![rex]).unwrap_or_default()
    }

    pub fn compute_to_rax(&self) -> Vec<u8> {
        match self.code {
            Code::MovRM64Imm32 => [vec![0xc7, 0xc0], self.ops[1].get_imm32().expect("internal error").to_ne_bytes().to_vec()].concat(),
            _ => unreachable!(),
        }
    }

    pub fn memory_access(&self) -> Option<AccessKind> {
        match self.code {
            Code::MovRM64Imm32 => self.ops[0].get_memory().map(|memory| AccessKind::Write(memory)),
            Code::Syscall => None,
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Imm32(u32),
    Memory(MemoryAddr),
    Register(Register),
}

impl Operand {
    pub fn get_memory<'a>(&'a self) -> Option<&'a MemoryAddr> {
        match self {
            Operand::Memory(memory) => Some(memory),
            _ => None,
        }
    }

    pub fn get_imm32(&self) -> Option<u32> {
        match self {
            Operand::Imm32(imm32) => Some(*imm32),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct MemoryAddr {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub scale: Option<u8>,
    pub displacement: Option<Displacement>,
}

impl MemoryAddr {
    pub fn new(base: Option<Register>, index: Option<Register>, scale: Option<u8>, displacement: Option<Displacement>) -> MemoryAddr {
        MemoryAddr {
            base,
            index,
            scale,
            displacement,
        }
    }

    fn index(&self, context: &Context) -> u64 {
        self.index.map(|index| context.get_register(&index)).unwrap_or_default() * self.scale.map(|scale| (scale as u64 * 2).max(1)).unwrap_or(1)
    }

    fn base(&self, context: &Context) -> u64 {
        self.base.map(|base| context.get_register(&base)).unwrap_or_default()
    }

    pub fn virtual_address(&self) -> u64 {
        let context = emu::context();

        self.displacement.map(|disp| Into::<u64>::into(disp)).unwrap_or_default() + self.base(&context) + self.index(&context)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Displacement {
    Disp8((u8, u8)),
    Disp32((u32, u8)),
}

impl Into<u64> for Displacement {
    fn into(self) -> u64 {
        match self {
            Displacement::Disp8((disp, multiplier)) => (disp as u64).pow(multiplier as u32),
            Displacement::Disp32((disp, multiplier)) => (disp as u64).pow(multiplier as u32),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
}

impl From<u8> for Register {
    fn from(value: u8) -> Register {
        match value {
            0 => Register::Rax,
            1 => Register::Rcx,
            2 => Register::Rdx,
            3 => Register::Rbx,
            4 => Register::Rsp,
            5 => Register::Rbp,
            6 => Register::Rsi,
            7 => Register::Rdi,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Code {
    MovRM64Imm32,
    Syscall,
}


