mod syscall;
mod memory;
mod chunk;
mod jit;

use crate::{Kind, parse};

use chunk::InstructionChunk;
use memory::Memory;
use jit::Jit;

use object::{Object, File};
use log::{info, warn};

use std::fs;


#[derive(Debug, Default)]
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
        })
    }

    pub fn run(&mut self) {
        let chunk = InstructionChunk::new(&self.memory, &mut self.ip);

        self.ctx = self.jit.exec(&chunk.bytes, &self.ctx);
    }
}


