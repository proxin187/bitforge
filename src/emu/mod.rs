mod syscall;
mod memory;
mod chunk;
mod jit;

use crate::{Instruction, Kind};

use chunk::InstructionChunk;
use memory::Memory;
use jit::Jit;

use object::{Object, File};
use log::{info, warn};

use std::fs;


#[derive(Debug, Default, Clone, Copy)]
pub struct Context {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rbp: u64,
    rsp: u64,
    rsi: u64,
    rdi: u64,
    r: [u64; 7],
}

#[derive(Debug)]
pub struct Executor {
    memory: Memory,
    ctx: Context,
    jit: Jit,
    ip: usize,
    should_close: bool,
}

impl Executor {
    pub fn new(path: &str) -> Result<Executor, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        let file = File::parse(&*data)?;

        Ok(Executor {
            memory: Memory::from(&file),
            ctx: Context::default(),
            jit: Jit::new(),
            ip: file.entry() as usize,
            should_close: false,
        })
    }

    fn emulate(&mut self, instruction: Instruction) {
        match instruction.kind {
            Kind::MOVModRMImmediate { reg, imm32 } => {
            },
            Kind::SYSCALL => {
                match syscall::perform(self.ctx) {
                    Some(ctx) => {
                        self.ctx = ctx;
                    },
                    None => {
                        self.should_close = true;
                    },
                }
            },
        }
    }

    pub fn run(&mut self) {
       while !self.should_close {
            let chunk = InstructionChunk::new(&self.memory, &mut self.ip);

            if !chunk.bytes.is_empty() {
                self.ctx = self.jit.exec(&chunk.bytes, &self.ctx);
            }

            self.emulate(chunk.terminator);
        }
    }
}


