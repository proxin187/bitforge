pub mod decode;
pub mod error;


pub enum AccessKind<'a> {
    Read(&'a Memory),
    Write(&'a Memory),
    Both(&'a Memory, &'a Memory),
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
    Memory(Memory),
    Register(Register),
}

impl Operand {
    pub fn get_memory<'a>(&'a self) -> Option<&'a Memory> {
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
pub struct Memory {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub scale: Option<u8>,
    pub displacement: Option<Displacement>,
}

impl Memory {
    pub fn new(base: Option<Register>, index: Option<Register>, scale: Option<u8>, displacement: Option<Displacement>) -> Memory {
        Memory {
            base,
            index,
            scale,
            displacement,
        }
    }
}

#[derive(Debug)]
pub enum Displacement {
    Disp8((u8, u8)),
    Disp32((u32, u8)),
}

#[derive(Debug)]
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


