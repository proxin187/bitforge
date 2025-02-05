use super::{Instruction, Operand, Register, Memory, Displacement, Code};
use super::error::Error;

use std::array::IntoIter;
use std::iter::Peekable;


#[derive(Debug, Clone, Copy)]
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

pub struct Sib {
    scale: u8,
    index: u8,
    base: u8,
}

impl Sib {
    pub fn new(byte: u8) -> Sib {
        Sib {
            scale: byte >> 6,
            index: (byte >> 3) & 7,
            base: byte & 7,
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

    fn rm(&mut self, modrm: ModRM) -> Result<Memory, Error> {
        match modrm.rm {
            0b100 => {
                let sib = Sib::new(self.bytes.next().ok_or(Error::InsufficientBytes)?);

                let mut memory = Memory::new(Some(Register::from(sib.base)), None, Some(sib.scale * 2), None);

                if sib.index != 0b100 {
                    memory.index.replace(Register::from(sib.index));
                }

                if sib.base == 0b101 {
                    match modrm.mod_ {
                        0b00 | 0b10 => {
                            memory.displacement.replace(self.disp32(None)?);
                        },
                        0b01 => {
                            memory.displacement.replace(self.disp8(None)?);
                        },
                        _ => return Err(Error::InvalidEncoding),
                    }
                }

                Ok(memory)
            },
            _ => Ok(Memory::new(Some(Register::from(modrm.rm)), None, None, None)),
        }
    }

    #[inline]
    fn disp8(&mut self, multiplier: Option<u8>) -> Result<Displacement, Error> {
        let byte = self.bytes.next().ok_or(Error::InsufficientBytes)?;

        Ok(Displacement::Disp8((byte, multiplier.unwrap_or(1))))
    }

    #[inline]
    fn disp32(&mut self, multiplier: Option<u8>) -> Result<Displacement, Error> {
        let bytes = self.bytes.next_chunk().map_err(|_| Error::InsufficientBytes)?;

        Ok(Displacement::Disp32((u32::from_ne_bytes(bytes), multiplier.unwrap_or(1))))
    }

    fn modrm(&mut self) -> Result<(Operand, Register), Error> {
        let modrm = ModRM::new(self.bytes.next().ok_or(Error::InsufficientBytes)?);

        match modrm.mod_ {
            0b00 => match modrm.rm {
                0b101 => Ok((Operand::Memory(Memory::new(None, None, None, Some(self.disp32(Some(2))?))), Register::from(modrm.reg))),
                _ => Ok((Operand::Memory(self.rm(modrm)?), Register::from(modrm.reg))),
            },
            0b01 => {
                let mut memory = self.rm(modrm)?;

                memory.displacement.replace(self.disp8((modrm.rm == 0).then(|| 3))?);

                Ok((Operand::Memory(memory), Register::from(modrm.reg)))
            },
            0b10 => {
                let mut memory = self.rm(modrm)?;

                memory.displacement.replace(self.disp32(None)?);

                Ok((Operand::Memory(memory), Register::from(modrm.reg)))
            },
            0b11 => Ok((Operand::Register(Register::from(modrm.rm)), Register::from(modrm.reg))),
            _ => Err(Error::InvalidEncoding),
        }
    }

    pub fn decode(&mut self) -> Result<Instruction, Error> {
        let rex = self.bytes.next_if(|byte| byte & 0xf0 == 0x40);

        match self.bytes.next() {
            Some(0xc7) => {
                let (operand, register) = self.modrm()?;

                match register {
                    Register::Rax => {
                        Ok(Instruction {
                            code: Code
                        })
                    },
                    _ => unimplemented!(),
                }
            },
            Some(_) | None => Err(Error::InsufficientBytes),
        }
    }
}

pub fn decode(bytes: &[u8]) -> Result<Instruction, Error> {
    let mut decoder = Decoder::new(bytes.try_into().map_err(|_| Error::InsufficientBytes)?);

    decoder.decode()
}


