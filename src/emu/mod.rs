mod translate;
mod syscall;
mod memory;
mod chunk;
mod jit;

use crate::instruction::{Instruction, Code, Register};

use chunk::InstructionChunk;
use jit::Jit;

use object::{Object, File};
use log::{info, warn};

use std::sync::{Mutex, LazyLock};
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
pub struct Executor {
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
            jit: Jit::new(),
            ip: file.entry() as usize,
            should_close: false,
        })
    }

    fn emulate(&mut self, instruction: Instruction) -> Result<(), Box<dyn std::error::Error>> {
        match instruction.code {
            Code::Syscall => {
                if syscall::perform() {
                    self.should_close = true;
                }

                Ok(())
            },
            _ => unreachable!(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while !self.should_close {
            let chunk = InstructionChunk::new(&mut self.ip)?;

            break;

            if !chunk.bytes.is_empty() {
                self.jit.exec(&chunk.bytes);
            }

            self.emulate(chunk.terminator);
        }

        unsafe {
            memory::_write_raw64(0 as *const _, 0);
            memory::_read_raw64(0 as *const _);
        }

        Ok(())
    }
}


