use crate::instruction::{Instruction, MemoryAddr, AccessKind};
use crate::emu::chunk::Part;
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

    pub fn process(&mut self, part: &Part) {
        match part.instruction.memory_access() {
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
                        // TODO: this will have to compute to rdi instead
                        part.instruction.prefixes(), part.instruction.compute_to_rdi(),

                        // mov rcx, {address of write}
                        vec![0x48, 0xb9], ((memory::_write_raw64 as *const ()) as usize).to_ne_bytes().to_vec(),

                        // mov rsi, {address of memory address}
                        vec![0x48, 0xbe], ((mem as *const MemoryAddr) as usize).to_ne_bytes().to_vec(),

                        // call rcx
                        vec![0xff, 0xd1],
                    ].concat());
                },
                AccessKind::Both(mem1, mem2) => {
                },
            },
            None => {
                self.out.extend(&part.bytes);
            },
        }
    }
}


