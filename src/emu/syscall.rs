use super::Context;

use log::info;


pub fn perform(ctx: Context) -> Option<Context> {
    match ctx.rax {
        60 => {
            info!("exited with status code: {}", ctx.rdi);

            None
        },
        _ => unimplemented!(),
    }
}


