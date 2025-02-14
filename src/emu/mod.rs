pub mod memory;
mod translate;
mod syscall;
mod chunk;
mod jit;

use crate::instruction::{Instruction, Code, Register};

use chunk::InstructionChunk;
use translate::Translate;
use jit::Jit;

use object::{Object, File};
use log::info;

use std::fs;

pub static mut CONTEXT: Context = Context::new();

pub fn context() -> Context {
    unsafe { CONTEXT }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Context {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rbp: u64,
    rsp: u64,
    rsi: u64,
    rdi: u64,
}

impl Context {
    pub const fn new() -> Context {
        Context {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rbp: 0,
            rsp: 0x1001a8,
            rsi: 0,
            rdi: 0,
        }
    }

    pub fn get_register(&self, register: &Register) -> u64 {
        match register {
            Register::Rax => self.rax,
            Register::Rbx => self.rbx,
            Register::Rcx => self.rcx,
            Register::Rdx => self.rdx,
            Register::Rbp => self.rbp,
            Register::Rsp => self.rsp,
            Register::Rsi => self.rsi,
            Register::Rdi => self.rdi,
        }
    }
}

#[derive(Debug)]
pub struct Eflags {
    CF: bool,
    OF: bool,
    SF: bool,
    ZF: bool,
    AF: bool,
    PF: bool,
}

impl Eflags {
    pub fn new() -> Eflags {
        Eflags {
            CF: false,
            OF: false,
            SF: false,
            ZF: false,
            AF: false,
            PF: false,
        }
    }
}

#[derive(Debug)]
pub struct Executor {
    eflags: Eflags,
    jit: Jit,
    ip: usize,
    should_close: bool,
}

impl Executor {
    pub fn new(path: &str) -> Result<Executor, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        let file = File::parse(&*data)?;

        memory::load(&file)?;

        Ok(Executor {
            eflags: Eflags::new(),
            jit: Jit::new(),
            ip: file.entry() as usize,
            should_close: false,
        })
    }

    fn emulate(&mut self, instruction: Instruction) {
        match instruction.code {
            Code::CmpRM64Imm8 => {
                let register = instruction.ops[0].get_register().expect("invalid argument");
                let imm8 = instruction.ops[1].get_imm8().expect("invalid argument");

                info!("register: {:?}, imm8: {}", unsafe { CONTEXT.get_register(&register) }, imm8);

                self.eflags.ZF = unsafe { CONTEXT.get_register(&register) } == imm8 as u64;
            },
            Code::JneRel8 => {
                if !self.eflags.ZF {
                    self.ip = (self.ip as isize + instruction.ops[0].get_imm8().expect("invalid arguments") as isize) as usize;
                }
            },
            Code::Syscall => {
                if syscall::perform() {
                    self.should_close = true;
                }
            },
            _ => unreachable!(),
        }

        self.ip += instruction.size;
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while !self.should_close {
            let instructions = InstructionChunk::new(&mut self.ip)?;
            let mut translate = Translate::new();

            for part in instructions.chunk.iter() {
                translate.process(&part);
            }

            if !translate.is_empty() {
                self.jit.exec(translate);
            }

            self.emulate(instructions.terminator);
        }

        Ok(())
    }
}


