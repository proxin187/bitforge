pub mod decode;
pub mod error;


#[derive(Debug)]
pub struct Instruction {
    pub code: Code,
    pub ops: Vec<Operand>,
    pub size: usize,
    pub rex: Option<u8>,
}

impl Instruction {
    // TODO: we will have to make this turn a function that reads or writtes memory into a function
    // that does the same except with registers, for examples mov [rsp + 4], 2 would be turned into
    // mov rax, 4, and then some instructions to write it into emulated memory
    pub fn to_reg(&self) {
    }
}

#[derive(Debug)]
pub enum Operand {
    Imm32(u32),
    Memory(Memory),
    Register(Register),
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


