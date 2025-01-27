mod syscall;
mod memory;
mod chunk;

use crate::{Kind, parse};

use chunk::InstructionChunk;
use memory::Memory;

use object::{Object, File};
use log::{info, warn};

use std::arch::asm;
use std::slice;
use std::mem;
use std::fs;


#[derive(Debug, Default)]
pub struct Context {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rbp: u64,
    rsp: u64,
    rsi: u64,
    rdi: u64,
    r: [u64; 7],
}

#[derive(Debug)]
pub struct Executor {
    memory: Memory,
    ctx: Context,
    ip: usize,
}

impl Executor {
    pub fn new(path: &str) -> Result<Executor, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        let file = File::parse(&*data)?;

        Ok(Executor {
            memory: Memory::from(&file),
            ctx: Context::default(),
            ip: file.entry() as usize,
        })
    }

    fn exec(&mut self, mut chunk: InstructionChunk) {
        chunk.bytes.extend(unsafe { slice::from_raw_parts(_restore as *const u8, 8) });

        // it should be enough to simply add instructions that load the context at the end
        // as the rest of the chunk is scanned and we are therefore shure it will never access or modify our context
        //
        // this also includes push and pop instructions as these not only access registers but also
        // memory
        //
        // rules:
        //  - rsp (stack pointer) will always stay the same through out the entire chunk, meaning
        //    the rsp will be the same after execution as it was before execution.
        //  - we could therefore ensure that we dont need to update the stack pointer as it should
        //    be the same through out the entire execution

        let func: unsafe extern "C" fn() = unsafe { mem::transmute_copy(&chunk.bytes) };

        info!("chunk: {:x?}", chunk);
        info!("func1: {:?}", func);

        unsafe {
            info!("we only get here?");

            // store the registers of the emulator before running the chunk
            //
            // TODO: this segfaults
            asm!(
                "push rax",
                "push rbx",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",

                // it looks like pushing rbp causes a segfault, we will have to investiage this
                // "push rbp",
            );

            // load in context for the program we are emulating
            info!("the push is not a problem");

            // TODO: this fails because registers like eg. rsp (stack pointer) and rbp (frame pointer)
            //
            // TODO: now this segfaults, it is most likely because of rbp here too
            asm!(
                "mov rbx, {rbx}",
                "mov rbp, {rbp}",
                in("rax") self.ctx.rax,
                in("rcx") self.ctx.rcx,
                in("rdx") self.ctx.rdx,
                in("rsi") self.ctx.rsi,
                in("rdi") self.ctx.rdi,
                rbx = in(reg) self.ctx.rbx,
                rbp = in(reg) self.ctx.rbp,
            );

            info!("func2: {:?}", func);

            asm!("int3");

            func();
        }
    }

    pub fn run(&mut self) {
        let chunk = InstructionChunk::new(&self.memory, &mut self.ip);

        self.exec(chunk);
    }
}

#[naked]
unsafe extern "C" fn _restore() {
    asm!(
        "pop rbp",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        "ret",
        options(noreturn),
    );
}


