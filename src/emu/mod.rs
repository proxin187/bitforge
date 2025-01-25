mod memory;
mod chunk;

use crate::{Instruction, Kind};

use memory::Memory;
use chunk::InstructionChunk;
use object::{Object, File};
use log::{info, warn};

use std::fs;


#[derive(Debug)]
pub struct Executor {
    memory: Memory,
    entry: usize,
}

impl Executor {
    pub fn new(path: &str) -> Result<Executor, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        let file = File::parse(&*data)?;

        Ok(Executor {
            memory: Memory::from(&file),
            entry: file.entry() as usize,
        })
    }

    pub fn exec(&mut self) {
        /*
        if let Some((segment, size)) = self.memory.get_segment(self.entry).and_then(|segment| segment.len(self.entry).map(|size| (segment, size))) {
            let read = segment.read(self.entry..self.entry + size);

            // let chunk = InstructionChunk::new(read);
        }
        */

        info!("executor: {:x?}", self);
    }
}


