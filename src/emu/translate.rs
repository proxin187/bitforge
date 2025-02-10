use crate::instruction::{MemoryAddr, AccessKind};
use crate::emu::chunk::Part;
use crate::emu::{CONTEXT, memory};

use log::info;

use std::ptr::addr_of;


#[derive(Debug, Clone)]
pub enum Block {
    Byte(u8),
    StackPointer,
    FramePointer,
}

pub struct Translate {
    blocks: Vec<Block>,
}

impl Translate {
    pub fn new() -> Translate {
        Translate {
            blocks: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn to_bytes(self, rsp: u64, rbp: u64) -> Vec<u8> {
        self.blocks.iter()
            .flat_map(|block| match block {
                Block::Byte(byte) => vec![*byte],
                Block::StackPointer => rsp.to_ne_bytes().to_vec(),
                Block::FramePointer => rbp.to_ne_bytes().to_vec(),
            })
            .collect::<Vec<u8>>()
    }

    #[no_mangle]
    pub fn process(&mut self, part: &Part) {
        match part.instruction.memory_access() {
            Some(access) => match access {
                AccessKind::Read(mem) => {
                },
                AccessKind::Write(mem) => {
                    info!("dec: {}, bytes: {:?}", ((memory::_write_raw64 as *const ()) as usize), ((memory::_write_raw64 as *const ()) as usize).to_ne_bytes().to_vec());

                    unsafe {
                        let write_addr = ((memory::_write_raw64 as *const ()) as usize).to_ne_bytes()
                            .map(|byte| Block::Byte(byte))
                            .to_vec();

                        let ctx_addr = (addr_of!(CONTEXT) as u64).to_ne_bytes()
                            .map(|byte| Block::Byte(byte))
                            .to_vec();

                        let mem_addr = ((mem as *const MemoryAddr) as usize).to_ne_bytes()
                            .map(|byte| Block::Byte(byte))
                            .to_vec();

                        self.blocks.extend([
                            // mov r9, {address of context}
                            vec![Block::Byte(0x49), Block::Byte(0xb9)], ctx_addr.clone(),

                            // mov [r9], rax
                            vec![Block::Byte(0x49), Block::Byte(0x89), Block::Byte(0x01)],

                            // mov [r9 + 8], rbx
                            vec![Block::Byte(0x49), Block::Byte(0x89), Block::Byte(0x59), Block::Byte(0x08)],

                            // mov [r9 + 16], rcx
                            vec![Block::Byte(0x49), Block::Byte(0x89), Block::Byte(0x49), Block::Byte(0x10)],

                            // mov [r9 + 24], rdx
                            vec![Block::Byte(0x49), Block::Byte(0x89), Block::Byte(0x51), Block::Byte(0x18)],

                            // mov [r9 + 32], rbp
                            vec![Block::Byte(0x49), Block::Byte(0x89), Block::Byte(0x69), Block::Byte(0x20)],

                            // mov [r9 + 40], rsp
                            vec![Block::Byte(0x49), Block::Byte(0x89), Block::Byte(0x61), Block::Byte(0x28)],

                            // mov [r9 + 48], rsi
                            vec![Block::Byte(0x49), Block::Byte(0x89), Block::Byte(0x71), Block::Byte(0x30)],

                            // mov [r9 + 56], rdi
                            vec![Block::Byte(0x49), Block::Byte(0x89), Block::Byte(0x79), Block::Byte(0x38)],

                            // mov rsp, {stack pointer}
                            vec![Block::Byte(0x48), Block::Byte(0xbc), Block::StackPointer],

                            // mov rbp, {frame pointer}
                            vec![Block::Byte(0x48), Block::Byte(0xbd), Block::FramePointer],

                            // mov rdi, {computed value}
                            part.instruction.compute_to_rdi().iter().map(|byte| Block::Byte(*byte)).collect(),

                            // mov rsi, {address of memory address}
                            vec![Block::Byte(0x48), Block::Byte(0xbe)], mem_addr,

                            // mov rax, {address of write}
                            vec![Block::Byte(0x48), Block::Byte(0xb8)], write_addr,

                            // call rax
                            vec![Block::Byte(0xff), Block::Byte(0xd0)],

                            // mov r9, {address of context}
                            vec![Block::Byte(0x49), Block::Byte(0xb9)], ctx_addr,

                            // mov rax, [r9]
                            vec![Block::Byte(0x49), Block::Byte(0x8b), Block::Byte(0x01)],

                            // mov rbx, [r9 + 8]
                            vec![Block::Byte(0x49), Block::Byte(0x8b), Block::Byte(0x59), Block::Byte(0x08)],

                            // mov rcx, [r9 + 16]
                            vec![Block::Byte(0x49), Block::Byte(0x8b), Block::Byte(0x49), Block::Byte(0x10)],

                            // mov rdx, [r9 + 24]
                            vec![Block::Byte(0x49), Block::Byte(0x8b), Block::Byte(0x51), Block::Byte(0x18)],

                            // mov rbp, [r9 + 32]
                            vec![Block::Byte(0x49), Block::Byte(0x8b), Block::Byte(0x69), Block::Byte(0x20)],

                            // mov rsp, [r9 + 40]
                            vec![Block::Byte(0x49), Block::Byte(0x8b), Block::Byte(0x61), Block::Byte(0x28)],

                            // mov rsi, [r9 + 48]
                            vec![Block::Byte(0x49), Block::Byte(0x8b), Block::Byte(0x71), Block::Byte(0x30)],

                            // mov rdi, [r9 + 56]
                            vec![Block::Byte(0x49), Block::Byte(0x8b), Block::Byte(0x79), Block::Byte(0x38)],
                        ].concat());
                    }
                },
                AccessKind::Both(mem1, mem2) => {
                },
            },
            None => {
                let blocks = part.bytes.iter()
                    .map(|byte| Block::Byte(*byte));

                self.blocks.extend(blocks);
            },
        }
    }
}


