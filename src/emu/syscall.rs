use super::Context;


// TODO: add support for rules

#[derive(Debug)]
pub struct Syscall {
}

impl Syscall {
    pub fn new() -> Syscall {
        Syscall {
        }
    }

    pub fn perform(&self, ctx: &Context) {
        match ctx.rax {
            _ => {
                // TODO: emulate the behaviour of invalid syscall
            },
        }
    }
}


