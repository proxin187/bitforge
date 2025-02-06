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

    pub fn process(&mut self, instruction: Instruction, bytes: &[u8]) {
        match instruction.memory_access() {
            Some(access) => match access {
                AccessKind::Read(memory) => {
                },
                AccessKind::Write(memory) => {
                    // TODO: here we will have to push all the registers to the stack in addition
                    // to popping them after the write is done
                    self.out.extend([
                        instruction.prefixes(), instruction.compute_to_rax(),

                        // TODO: this will have to be a far indirect call to the write function
                        vec![0xff, ],
                    ].concat());
                },
                AccessKind::Both(mem1, mem2) => {
                },
            },
            None => {
                self.out.extend(bytes);
            },
        }
    }
}


