use log::warn;


struct ModRM {
    mod_: u8,
    reg: u8,
    rm: u8,
}

impl ModRM {
    pub fn new(byte: u8) -> ModRM {
        ModRM {
            mod_: byte & 0xc0 >> 6,
            reg: byte & 0x38 >> 3,
            rm: byte & 0x7,
        }
    }
}

pub enum Kind {
    Syscall,
    Other,
}

pub struct Instruction {
    kind: Kind,
    size: u64,
}

impl Instruction {
    pub fn new(kind: Kind, size: u64) -> Instruction {
        Instruction {
            kind,
            size,
        }
    }
}

// TODO: maybe we can procedurally generate this?

impl From<[u8; 16]> for Instruction {
    fn from(bytes: [u8; 16]) -> Instruction {
        match bytes {

            // MOV r/m64, imm32: REX.W + C7 /0 id
            [0x48, 0xc7, ..] => Instruction::new(Kind::Other, 7),

            // syscall: 0F 05
            [0x0f, 0x05, ..] => Instruction::new(Kind::Syscall, 2),
            _ => {
                warn!("unrecognized instruction sequence: {:#x?}", bytes);

                unreachable!();
            },
        }
    }
}

pub struct InstructionChunk {
    terminator: Instruction,
    chunk: Vec<u8>,
}

impl InstructionChunk {
    pub fn new(bytes: Vec<u8>) -> InstructionChunk {
        let chunk: Vec<u8> = Vec::new();

        // TODO: finish this

        InstructionChunk {
            terminator: Instruction::new(Kind::Syscall, 2),
            chunk,
        }
    }
}


