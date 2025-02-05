mod translate;
mod syscall;
mod memory;
mod chunk;
mod jit;

use crate::instruction::{Instruction, Code};

use chunk::InstructionChunk;
use memory::Memory;
use jit::Jit;

use object::{Object, File};
use log::{info, warn};

use std::fs;


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
    pub fn new() -> Context {
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
}

#[derive(Debug)]
pub struct Executor {
    ctx: Context,
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
            ctx: Context::new(),
            jit: Jit::new(),
            ip: file.entry() as usize,
            should_close: false,
        })
    }

    fn emulate(&mut self, instruction: Instruction) {
        match instruction.code {
            Code::Syscall => {
                match syscall::perform(self.ctx) {
                    Some(ctx) => {
                        self.ctx = ctx;
                    },
                    None => {
                        self.should_close = true;
                    },
                }
            },
            _ => unreachable!(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while !self.should_close {
            let chunk = InstructionChunk::new(&mut self.ip)?;

            break;

            if !chunk.bytes.is_empty() {
                self.ctx = self.jit.exec(&chunk.bytes, &self.ctx);
            }

            self.emulate(chunk.terminator);
        }

        Ok(())
    }
}


