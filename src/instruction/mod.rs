mod decode;


#[derive(Debug)]
pub struct Instruction {
    code: Code,
    ops: Vec<Operand>,
    size: usize,
}

#[derive(Debug)]
pub enum Operand {
    Imm32(u32),
    Memory(Memory),
    Register(Register),
}

#[derive(Debug)]
pub struct Memory {
    base: Option<Register>,
    index: Option<Register>,
    scale: u8,
    displacement: u32,
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

#[derive(Debug)]
pub enum Code {
    MovRM64Imm32,
    Syscall,
}


