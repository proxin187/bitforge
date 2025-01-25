use crate::{Instruction, Kind, parse};

use log::info;


struct ModRM {
    mod_: u8,
    reg: u8,
    rm: u8,
}

impl ModRM {
    pub fn new(byte: u8) -> ModRM {
        ModRM {
            mod_: byte & 0xc0 >> 6,
            reg: byte & 0x38 >> 3,
            rm: byte & 0x7,
        }
    }
}

#[derive(Debug)]
pub struct InstructionChunk {
    terminator: Instruction,
    chunk: Vec<u8>,
}

impl InstructionChunk {
    // TODO: for performance reasons we will chunk up instructions up until a terminator (any
    // instruction we need to emulate), then execute the chunk as a whole,
    // this prevents checking the next instruction for each and every instruction

    pub fn new(read: &[u8]) -> InstructionChunk {
        let mut chunk: Vec<u8> = Vec::new();
        let mut ip = 0;

        loop {
            let instruction = parse(&read[ip..]);

            info!("instruction: {:?}", instruction);

            match instruction.kind {
                Kind::SYSCALL {} => {
                    return InstructionChunk {
                        terminator: instruction,
                        chunk,
                    };
                },
                _ => {
                    chunk.extend(&read[ip..ip + instruction.size as usize]);

                    ip += instruction.size as usize;
                },
            }
        }
    }
}


