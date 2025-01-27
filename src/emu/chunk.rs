use super::memory::Memory;

use crate::{Instruction, Kind, parse};

use log::info;


#[derive(Debug)]
pub struct InstructionChunk {
    pub terminator: Instruction,
    pub chunk: Vec<u8>,
}

impl InstructionChunk {
    pub fn new(memory: &Memory, ip: &mut usize) -> InstructionChunk {
        let mut chunk: Vec<u8> = Vec::new();

        loop {
            let instruction = parse(&memory.read(*ip..*ip + 16));

            info!("instruction: {:?}", instruction);

            match instruction.kind {
                Kind::SYSCALL {} => {
                    return InstructionChunk {
                        terminator: instruction,
                        chunk,
                    };
                },
                _ => {
                    chunk.extend(memory.read(*ip..*ip + instruction.size as usize));

                    *ip += instruction.size as usize;
                },
            }
        }
    }
}


