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
        let mut rsp: u64;

        unsafe {
            // TODO: this can be procedurally generated instead of hardcoding it
            let entry: Vec<u8> = vec![
                vec![0x48, 0xb8], ctx.rax.to_ne_bytes().to_vec(),
                vec![0x48, 0xb9], ctx.rcx.to_ne_bytes().to_vec(),
                vec![0x48, 0xba], ctx.rdx.to_ne_bytes().to_vec(),
                vec![0x48, 0xbe], ctx.rsi.to_ne_bytes().to_vec(),
                vec![0x48, 0xbf], ctx.rdi.to_ne_bytes().to_vec(),
                vec![0x48, 0xbb], ctx.rbx.to_ne_bytes().to_vec(),
                vec![0x48, 0xbc], ctx.rsp.to_ne_bytes().to_vec(),
            ].concat();

            asm!(
                "mov {rsp}, rsp",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                rsp = out(reg) rsp,
            );

            // TODO: we need to figure out whether this increments the stack pointer, we could
            // ensure it doesnt increment by predefining it as an empty Vec
            let restore: Vec<u8> = vec![
                vec![0x48, 0xbc], rsp.to_ne_bytes().to_vec(),
                vec!(0xc3),
            ].concat();

            ptr::copy(entry.as_ptr(), self.ptr, entry.len());

            ptr::copy(bytes.as_ptr(), self.ptr.add(entry.len()), bytes.len());

            ptr::copy(restore.as_ptr(), self.ptr.add(entry.len() + bytes.len()), restore.len());

            mem::transmute::<*mut u8, unsafe extern "C" fn()>(self.ptr)();

            // TODO: this might need to be embeded in restore as that would be more consistent
            // this could be done by embeding the memory addresses of output in the mov
            // instructions
            asm!(
                "mov {rbx}, rbx",
                out("rax") output.rax,
                out("rcx") output.rcx,
                out("rdx") output.rdx,
                out("rsi") output.rsi,
                out("rdi") output.rdi,
                rbx = out(reg) output.rbx,
            );

            output
        }
    }
}


