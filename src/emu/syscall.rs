use crate::emu;

use log::info;


pub fn perform() -> bool {
    let ctx = emu::context();

    match ctx.rax {
        60 => {
            info!("exited with status code: {}", ctx.rdi);

            true
        },
        _ => unimplemented!(),
    }
}


