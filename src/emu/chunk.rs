use super::memory;

use crate::{Instruction, Kind, parse};

use log::info;


#[derive(Debug)]
pub struct InstructionChunk {
    pub terminator: Instruction,
    pub bytes: Vec<u8>,
}

impl InstructionChunk {
    pub fn new(ip: &mut usize) -> Result<InstructionChunk, Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();

        loop {
            let instruction = parse(&memory::read(*ip..*ip + 16)?);

            info!("instruction: {:?}", instruction);

            match instruction.kind {
                Kind::SYSCALL {} => {
                    return Ok(InstructionChunk {
                        terminator: instruction,
                        bytes,
                    });
                },
                _ => {
                    bytes.extend(memory::read(*ip..*ip + instruction.size as usize)?);

                    *ip += instruction.size as usize;
                },
            }
        }
    }
}


