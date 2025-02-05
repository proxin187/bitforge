use crate::instruction::{Instruction, AccessKind};


pub struct Translate {
    pub out: Vec<u8>,
}

impl Translate {
    pub fn new() -> Translate {
        Translate {
            out: Vec::new(),
        }
    }

    // TODO: this process is mostly going to be about checking the modrm byte and passing it to the
    // read or write function
    pub fn process(&mut self, instruction: Instruction, bytes: &[u8]) {
        match instruction.memory_access() {
            Some(access) => match access {
                AccessKind::Read => {
                },
                AccessKind::Write => {
                },
                AccessKind::Both => {
                },
            },
            None => {
                self.out.extend(bytes);
            },
        }
    }
}


