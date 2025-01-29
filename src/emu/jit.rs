use super::Context;

use std::arch::asm;
use std::mem;
use std::ptr;

use log::info;

const PAGE_SIZE: usize = 4096;


#[derive(Debug)]
pub struct Jit {
    ptr: *mut u8,
}

impl Jit {
    pub fn new() -> Jit {
        unsafe {
            let ptr = libc::mmap(
                0 as *mut libc::c_void,
                PAGE_SIZE,
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
            0,
            );

            info!("initialized: {:?}", ptr);

            Jit {
                ptr: ptr as *mut u8,
            }
        }
    }

    #[no_mangle]
    pub fn exec(&mut self, bytes: &[u8], ctx: &Context) -> Context {
        let mut output = Context::default();

        unsafe {
            // TODO: we will have to embed the mov of context in the bytes lol
            let _entry: [u8; 0] = [
            ];

            ptr::copy(bytes.as_ptr(), self.ptr, bytes.len());

            ptr::copy(_restore as *const u8, self.ptr.add(bytes.len()), 1);

            // TODO: we will have to embed the mov of context in the bytes lol
            asm!(
                "mov rbx, {rbx}",
                in("rax") ctx.rax,
                in("rcx") ctx.rcx,
                in("rdx") ctx.rdx,
                in("rsi") ctx.rsi,
                in("rdi") ctx.rdi,
                rbx = in(reg) ctx.rbx,
            );

            mem::transmute::<*mut u8, unsafe extern "C" fn()>(self.ptr)();

            asm!(
                "mov {rbx}, rbx",
                rbx = out(reg) output.rbx,
            );

            output
        }
    }
}

#[naked]
unsafe extern "C" fn _restore() {
    asm!(
        "ret",
        options(noreturn),
    );
}


