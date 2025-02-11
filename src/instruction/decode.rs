use super::{Instruction, Operand, Register, MemoryAddr, Displacement, Code};
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

    #[inline]
    fn read8(&mut self) -> Result<u8, Error> {
        self.bytes.next().ok_or(Error::InsufficientBytes)
    }

    #[inline]
    fn read32(&mut self) -> Result<u32, Error> {
        let bytes = self.bytes.next_chunk().map_err(|_| Error::InsufficientBytes)?;

        Ok(u32::from_ne_bytes(bytes))
    }

    fn rm(&mut self, modrm: ModRM) -> Result<MemoryAddr, Error> {
        match modrm.rm {
            0b100 => {
                let sib = Sib::new(self.bytes.next().ok_or(Error::InsufficientBytes)?);

                let mut memory = MemoryAddr::new(Some(Register::from(sib.base)), None, Some(sib.scale * 2), None);

                if sib.index != 0b100 {
                    memory.index.replace(Register::from(sib.index));
                }

                if sib.base == 0b101 {
                    match modrm.mod_ {
                        0b00 | 0b10 => {
                            memory.displacement.replace(Displacement::Disp32((self.read32()?, 1)));
                        },
                        0b01 => {
                            memory.displacement.replace(Displacement::Disp8((self.read8()?, 1)));
                        },
                        _ => return Err(Error::InvalidEncoding),
                    }
                }

                Ok(memory)
            },
            _ => Ok(MemoryAddr::new(Some(Register::from(modrm.rm)), None, None, None)),
        }
    }

    fn modrm(&mut self) -> Result<(Operand, Register), Error> {
        let modrm = ModRM::new(self.bytes.next().ok_or(Error::InsufficientBytes)?);

        match modrm.mod_ {
            0b00 => match modrm.rm {
                0b101 => Ok((Operand::Memory(MemoryAddr::new(None, None, None, Some(Displacement::Disp32((self.read32()?, 2))))), Register::from(modrm.reg))),
                _ => Ok((Operand::Memory(self.rm(modrm)?), Register::from(modrm.reg))),
            },
            0b01 => {
                let mut memory = self.rm(modrm)?;

                memory.displacement.replace(Displacement::Disp8((self.read8()?, (modrm.rm == 0).then(|| 3).unwrap_or(1))));

                Ok((Operand::Memory(memory), Register::from(modrm.reg)))
            },
            0b10 => {
                let mut memory = self.rm(modrm)?;

                memory.displacement.replace(Displacement::Disp32((self.read32()?, 1)));

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
                            code: Code::MovRM64Imm32,
                            ops: vec![operand, Operand::Imm32(self.read32()?)],
                            size: 16 - self.bytes.by_ref().count(),
                            rex,
                        })
                    },
                    _ => unimplemented!(),
                }
            },
            Some(0x8b) => {
                let (operand, register) = self.modrm()?;

                Ok(Instruction  {
                    code: Code::MovR64RM64,
                    ops: vec![Operand::Register(register), operand],
                    size: 16 - self.bytes.by_ref().count(),
                    rex,
                })
            },
            Some(0x0f) => match self.bytes.next() {
                Some(0x05) => {
                    Ok(Instruction {
                        code: Code::Syscall,
                        ops: Vec::new(),
                        size: 16 - self.bytes.by_ref().count(),
                        rex,
                    })
                },
                Some(_) | None => Err(Error::InsufficientBytes),
            },
            Some(_) | None => Err(Error::InvalidEncoding),
        }
    }
}

pub fn decode(bytes: &[u8]) -> Result<Instruction, Error> {
    let mut decoder = Decoder::new(bytes.try_into().map_err(|_| Error::InsufficientBytes)?);

    decoder.decode()
}


