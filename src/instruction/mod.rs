pub mod decode;
pub mod error;

use crate::emu::{self, Context, memory};

use std::mem;


pub enum AccessKind {
    Read,
    Write,
    Both,
}

impl AccessKind {
    pub fn handler(&self) -> *const () {
        match self {
            AccessKind::Read => memory::_read_raw64 as *const (),
            AccessKind::Write => memory::_write_raw64 as *const (),
            _ => unreachable!(),
        }
    }
}

pub struct Access<'a> {
    pub addr: &'a MemoryAddr,
    pub kind: AccessKind,
}

#[derive(Debug)]
pub struct Instruction {
    pub code: Code,
    pub ops: Vec<Operand>,
    pub size: usize,
    pub rex: Option<u8>,
}

impl Instruction {
    pub fn compute_to_rdi(&self) -> Vec<u8> {
        match self.code {
            Code::MovRM64Imm32 => {
                let imm32 = self.ops[1].get_imm32().expect("internal error") as u64;

                [vec![0x48, 0xbf], imm32.to_ne_bytes().to_vec()].concat()
            },
            Code::MovR64RM64 => {
                let register: u8 = unsafe { mem::transmute(self.ops[0].get_register().expect("internal error")) };

                [vec![0x48, 0xbf], (register as u64).to_ne_bytes().to_vec()].concat()
            },
            Code::AddRM64Imm8 => {
                todo!("here we will have to make a mechanism where we can have an instruction to perform before the write")
            },
            _ => Vec::new(),
        }
    }

    pub fn memory_access(&self) -> Option<Access> {
        match self.code {
            Code::MovRM64Imm32 => self.ops[0].get_memory().map(|memory| Access { addr: memory, kind: AccessKind::Write }),
            Code::MovR64RM64 => self.ops[1].get_memory().map(|memory| Access { addr: memory, kind: AccessKind::Read }),
            Code::AddRM64Imm8 => self.ops[0].get_memory().map(|memory| Access { addr: memory, kind: AccessKind::Write }),
            Code::Syscall => None,
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Imm32(u32),
    Imm8(u8),
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

    pub fn get_register(&self) -> Option<Register> {
        match self {
            Operand::Register(register) => Some(*register),
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
#[repr(u8)]
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
    MovR64RM64,
    AddRM64Imm8,
    Syscall,
}


