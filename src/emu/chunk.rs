use super::memory;

use crate::instruction::{decode, Instruction, Code};

use log::info;


#[derive(Debug)]
pub struct Part {
    pub instruction: Instruction,
    pub bytes: Vec<u8>,
}

impl Part {
    pub fn new(instruction: Instruction, bytes: Vec<u8>) -> Part {
        Part {
            instruction,
            bytes,
        }
    }
}

#[derive(Debug)]
pub struct InstructionChunk {
    pub terminator: Instruction,
    pub chunk: Vec<Part>,
}

impl InstructionChunk {
    pub fn new(ip: &mut usize) -> Result<InstructionChunk, Box<dyn std::error::Error>> {
        let mut chunk: Vec<Part> = Vec::new();

        loop {
            let instruction = decode::decode(&memory::read(*ip..*ip + 16)?)?;

            info!("instruction: {:?}, ip: {:#x?}", instruction, *ip);

            match instruction.code {
                Code::Syscall | Code::JneRel8 | Code::CmpRM64Imm8 => {
                    return Ok(InstructionChunk {
                        terminator: instruction,
                        chunk,
                    });
                },
                _ => {
                    let bytes = memory::read(*ip..*ip + instruction.size as usize)?;

                    *ip += instruction.size as usize;

                    chunk.push(Part::new(instruction, bytes));
                },
            }
        }
    }
}


