use super::Instruction;


#[derive(Debug)]
pub struct ModRM {
    mod_: u8,
    reg: u8,
    rm: u8,
}

impl ModRM {
    pub fn new(byte: u8) -> ModRM {
        ModRM {
            mod_: byte >> 6,
            reg: (byte >> 3) & 7,
            rm: byte & 7,
        }
    }
}

pub fn parse(bytes: [u8; 16]) -> Instruction {
    match bytes {
        [0x48, 0xc7, reg, ..] | [0xc7, reg, ..] => match (reg >> 3) & 7 {
            0 => {
                // TODO: here we will have to parse modrm and determine whether we its memory to
                // register or anything else
                todo!();
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}


