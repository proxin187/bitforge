mod syscall;
mod memory;
mod chunk;

use crate::{Kind, parse};

use syscall::Syscall;
use memory::Memory;

use object::{Object, File};
use log::{info, warn};

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
    rip: u64,
    r: [u64; 7],
}

#[derive(Debug)]
pub struct Executor {
    syscall: Syscall,
    memory: Memory,
    ctx: Context,
    ip: usize,
}

impl Executor {
    pub fn new(path: &str) -> Result<Executor, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        let file = File::parse(&*data)?;

        Ok(Executor {
            syscall: Syscall::new(),
            memory: Memory::from(&file),
            ctx: Context::default(),
            ip: file.entry() as usize,
        })
    }

    pub fn exec(&mut self) {
        loop {
            let bytes = self.memory.read(self.ip..self.ip + 16);
            let instruction = parse(&bytes);

            match instruction.kind {
                Kind::MOVModRMImmediate { reg, imm32 } => {
                },
                Kind::SYSCALL => {
                    syscall::perform(&self.ctx);
                },
            }
        }

        /*
        let mut chunk = InstructionChunk::new(&self.memory, &mut self.ip);

        chunk.chunk.extend(unsafe { slice::from_raw_parts(_restore as *const u8, 8) });

        // it should be enough to simply add instructions that load the context at the end
        // as the rest of the chunk is scanned and we are therefore shure it will never access or modify our context
        //
        // this also includes push and pop instructions as these not only access registers but also
        // memory
        //
        // rules:
        //  - rsp (stack pointer) will always stay the same through out the entire chunk, meaning
        //    the rsp will be the same after execution as it was before execution.

        // TODO: does it work to access func after store and load?

        // TODO: maybe we can hardcode the return address here?
        let func: unsafe extern "C" fn() = unsafe { mem::transmute_copy(&chunk.chunk) };

        info!("chunk: {:x?}", chunk);
        info!("func: {:?}", func);

        unsafe {
            // store the registers of the emulator before running the chunk
            asm!(
                "push rax",
                "push rbx",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",
                "push rbp",
                "push [rsp - 0x38]",
            );

            // load in context for the program we are emulating

            // TODO: this fails because registers like eg. rsp (stack pointer) and rbp (frame pointer)
            asm!(
                "mov rbx, {rbx}",
                "mov rbp, {rbp}",
                "mov rsp, {rsp}",
                in("rax") self.ctx.rax,
                in("rcx") self.ctx.rcx,
                in("rdx") self.ctx.rdx,
                in("rsi") self.ctx.rsi,
                in("rdi") self.ctx.rdi,
                rbx = in(reg) self.ctx.rbx,
                rbp = in(reg) self.ctx.rbp,
                rsp = in(reg) self.ctx.rsp,
            );

            info!("func: {:?}", func);

            func();
        }
        */
    }
}

/*
#[naked]
unsafe extern "C" fn _restore() {
    asm!(
        "pop rsp",
        "pop rbp",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        options(noreturn),
    );
}
*/


