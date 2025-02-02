use crate::{Instruction, Kind};


pub struct Translate {
    pub out: Vec<u8>,
}

impl Translate {
    pub fn new() -> Translate {
        Translate {
            out: Vec::new(),
        }
    }

    // TODO: this process is mostly going to be about checking the modrm byte and passing it to the
    // read or write function
    pub fn process(&mut self, instruction: Instruction, bytes: &[u8]) {
        match instruction.kind {
            Kind::MOVHlexrO64Imm32Extended { reg, imm32 } => match reg.mod_ {
                0b00 | 0b01 | 0b10 => {
                },
                _ => {},
            },
             _ => {},
        }

        self.out.extend(bytes);
    }
}


