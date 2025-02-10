use crate::emu::{self, Context, CONTEXT, Translate};

use std::ptr::{self, addr_of};
use std::arch::asm;
use std::mem;

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
    pub fn exec(&mut self, translate: Translate) {
        let ctx = emu::context();
        let rsp: u64;
        let rbp: u64;
        let restore: Vec<u8>;

        unsafe {
            let entry: Vec<u8> = vec![
                vec![0x48, 0xb8], ctx.rax.to_ne_bytes().to_vec(),
                vec![0x48, 0xb9], ctx.rcx.to_ne_bytes().to_vec(),
                vec![0x48, 0xba], ctx.rdx.to_ne_bytes().to_vec(),
                vec![0x48, 0xbe], ctx.rsi.to_ne_bytes().to_vec(),
                vec![0x48, 0xbf], ctx.rdi.to_ne_bytes().to_vec(),
                vec![0x48, 0xbb], ctx.rbx.to_ne_bytes().to_vec(),
                vec![0x48, 0xbc], ctx.rsp.to_ne_bytes().to_vec(),
                vec![0x48, 0xbd], ctx.rbp.to_ne_bytes().to_vec(),
            ].concat();

            asm!(
                "mov {rsp}, rsp",
                "mov {rbp}, rbp",
                rsp = out(reg) rsp,
                rbp = out(reg) rbp,
            );

            let bytes = translate.to_bytes(rsp - 16, rbp);

            restore = vec![
                // mov r9, {address of context}
                vec![0x49, 0xb9], (addr_of!(CONTEXT) as u64).to_ne_bytes().to_vec(),

                vec![0x49, 0x89, 0x01],         // mov [r9], rax
                vec![0x49, 0x89, 0x59, 0x08],   // mov [r9 + 8], rbx
                vec![0x49, 0x89, 0x49, 0x10],   // mov [r9 + 16], rcx
                vec![0x49, 0x89, 0x51, 0x18],   // mov [r9 + 24], rdx
                vec![0x49, 0x89, 0x69, 0x20],   // mov [r9 + 32], rbp
                vec![0x49, 0x89, 0x61, 0x28],   // mov [r9 + 40], rsp
                vec![0x49, 0x89, 0x71, 0x30],   // mov [r9 + 48], rsi
                vec![0x49, 0x89, 0x79, 0x38],   // mov [r9 + 56], rdi

                vec![0x48, 0xbc], (rsp - 8).to_ne_bytes().to_vec(),
                vec![0x48, 0xbd], rbp.to_ne_bytes().to_vec(),
                vec!(0xc3),
            ].concat();

            ptr::copy(entry.as_ptr(), self.ptr, entry.len());

            ptr::copy(bytes.as_ptr(), self.ptr.add(entry.len()), bytes.len());

            ptr::copy(restore.as_ptr(), self.ptr.add(entry.len() + bytes.len()), restore.len());

            mem::transmute::<*mut u8, unsafe extern "C" fn()>(self.ptr)();
        }
    }
}


