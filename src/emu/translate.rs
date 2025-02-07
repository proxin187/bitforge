use crate::instruction::{Instruction, AccessKind};
use crate::emu::memory;


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
                AccessKind::Read(mem) => {
                },
                AccessKind::Write(mem) => {
                    // TODO: here we will have to push all the registers to the stack in addition
                    // to popping them after the write is done
                    //
                    // we will actually need to do this without the stack, therefore we might use
                    // something like a static variable to store it

                    self.out.extend([
                        instruction.prefixes(), instruction.compute_to_rax(),

                        // mov rcx, {address of write}
                        vec![0x48, 0xb9], ((memory::write_raw as *const ()) as usize).to_ne_bytes().to_vec(),

                        // call rcx
                        vec![0xff, 0xd1],
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


