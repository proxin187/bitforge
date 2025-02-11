use crate::instruction::{MemoryAddr, AccessKind};
use crate::emu::chunk::Part;
use crate::emu::{CONTEXT, memory};

use std::ptr::addr_of;


enum CtxOp {
    Load,
    Store,
}

impl CtxOp {
    pub fn value(&self) -> u8 {
        match self {
            CtxOp::Load => 0x8b,
            CtxOp::Store => 0x89,
        }
    }
}

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

    fn encode_addr(&self, addr: usize) -> Vec<Block> {
        addr.to_ne_bytes()
            .map(|byte| Block::Byte(byte))
            .to_vec()
    }

    fn ctx(&mut self, op: CtxOp, ctx_addr: Vec<Block>) {
        self.blocks.extend([
            vec![Block::Byte(0x49), Block::Byte(0xb9)], ctx_addr,
            vec![Block::Byte(0x49), Block::Byte(op.value()), Block::Byte(0x01)],
            vec![Block::Byte(0x49), Block::Byte(op.value()), Block::Byte(0x59), Block::Byte(0x08)],
            vec![Block::Byte(0x49), Block::Byte(op.value()), Block::Byte(0x49), Block::Byte(0x10)],
            vec![Block::Byte(0x49), Block::Byte(op.value()), Block::Byte(0x51), Block::Byte(0x18)],
            vec![Block::Byte(0x49), Block::Byte(op.value()), Block::Byte(0x69), Block::Byte(0x20)],
            vec![Block::Byte(0x49), Block::Byte(op.value()), Block::Byte(0x61), Block::Byte(0x28)],
            vec![Block::Byte(0x49), Block::Byte(op.value()), Block::Byte(0x71), Block::Byte(0x30)],
            vec![Block::Byte(0x49), Block::Byte(op.value()), Block::Byte(0x79), Block::Byte(0x38)],
        ].concat());
    }

    #[no_mangle]
    pub fn process(&mut self, part: &Part) {
        let ctx_addr = (unsafe { addr_of!(CONTEXT) as usize }).to_ne_bytes()
            .map(|byte| Block::Byte(byte))
            .to_vec();

        match part.instruction.memory_access() {
            Some(access) => {
                self.ctx(CtxOp::Store, ctx_addr.clone());

                self.blocks.extend([
                    // mov rsp, {stack pointer}
                    vec![Block::Byte(0x48), Block::Byte(0xbc), Block::StackPointer],

                    // mov rbp, {frame pointer}
                    vec![Block::Byte(0x48), Block::Byte(0xbd), Block::FramePointer],

                    // mov rdi, {computed value}
                    part.instruction.compute_to_rdi().iter().map(|byte| Block::Byte(*byte)).collect(),

                    // mov rsi, {address of memory address}
                    vec![Block::Byte(0x48), Block::Byte(0xbe)], self.encode_addr((access.addr as *const MemoryAddr) as usize),

                    // mov rax, {address of write}
                    vec![Block::Byte(0x48), Block::Byte(0xb8)], self.encode_addr(access.kind.handler() as usize),

                    // call rax
                    vec![Block::Byte(0xff), Block::Byte(0xd0)],
                ].concat());

                match access.kind {
                    AccessKind::Read => {
                    },
                    AccessKind::Write => {
                    },
                    AccessKind::Both => {
                    },
                }

                self.ctx(CtxOp::Load, ctx_addr.clone());
            },
            None => {
                let blocks = part.bytes.iter()
                    .map(|byte| Block::Byte(*byte));

                self.blocks.extend(blocks);
            },
        }
    }
}


