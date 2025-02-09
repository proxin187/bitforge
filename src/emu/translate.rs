use crate::instruction::{Instruction, MemoryAddr, AccessKind};
use crate::emu::chunk::Part;
use crate::emu::memory;

use log::info;


pub struct Translate {
    pub out: Vec<u8>,
}

impl Translate {
    pub fn new() -> Translate {
        Translate {
            out: Vec::new(),
        }
    }

    #[no_mangle]
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

                    info!("dec: {}, bytes: {:?}", ((memory::_write_raw64 as *const ()) as usize), ((memory::_write_raw64 as *const ()) as usize).to_ne_bytes().to_vec());

                    unsafe {
                        self.out.extend([
                            // part.instruction.compute_to_rdi(),

                            // mov rsi, {address of memory address}
                            // vec![0x48, 0xbe], ((mem as *const MemoryAddr) as usize).to_ne_bytes().to_vec(),

                            // mov rax, {address of write}
                            // vec![0x48, 0xb8], ((memory::_write_raw64 as *const ()) as usize).to_vec(),
                            vec![0x48, 0xb8], std::mem::transmute::<unsafe extern "sysv64" fn(), u64>(memory::_write_raw64 as unsafe extern "sysv64" fn()).to_ne_bytes().to_vec(),

                            // TODO: this doesnt work because the stack is invalid lol
                            // to fix this we will have to load in a valid stack pointer (rsp) and stack
                            // frame (rbp) before calling

                            // call rax
                            vec![0xff, 0xd0],
                        ].concat());
                    }
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


