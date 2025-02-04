use super::error::Error;
use super::Instruction;

use std::array::IntoIter;
use std::iter::Peekable;


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

pub struct Decoder {
    bytes: Peekable<IntoIter<u8, 16>>,
}

impl Decoder {
    pub fn new(bytes: [u8; 16]) -> Decoder {
        Decoder {
            bytes: bytes.into_iter().peekable(),
        }
    }

    // TODO: we will have to finish modrm support
    fn modrm(&mut self) -> Result<ModRM, Error> {
        let modrm = ModRM::new(self.bytes.next().ok_or(Error::InsufficientBytes)?);

        match modrm.mod_ {
            0b00 => match modrm.rm {
                _ => {},
            },
            _ => {},
        }

        Ok(ModRM::new(0))
    }

    pub fn decode(&mut self) -> Result<Instruction, Error> {
        let rex = self.bytes.next_if(|byte| byte & 0xf0 == 0x40);

        match self.bytes.next() {
            Some(0xc7) => {
            },
            None => Err(Error::InsufficientBytes),
        }
    }
}

pub fn parse(bytes: [u8; 16]) -> Instruction {
    match bytes {
        [0x48, 0xc7, reg, ..] | [0xc7, reg, ..] => match (reg >> 3) & 7 {
            0x0 => {
                // TODO: here we will have to parse modrm and determine whether we its memory to
                // register or anything else
                todo!();
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}


