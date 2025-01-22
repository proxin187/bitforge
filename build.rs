use std::io::{BufReader, BufRead, Lines, Write};
use std::path::Path;
use std::fs::File;
use std::env;


#[derive(Debug)]
pub enum OperandKind {
    Register,
    ModRM,
    Vex,
    Immediate,
    Is4Imz2,
    Implicit,
    Index,
}

impl OperandKind {
    pub fn size(&self) -> Option<&'static str> {
        match self {
            OperandKind::Register
                | OperandKind::ModRM
                | OperandKind::Vex => Some("u8"),
            OperandKind::Immediate
                | OperandKind::Is4Imz2
                | OperandKind::Index => Some("u64"),
            OperandKind::Implicit => None,
        }
    }
}

impl From<char> for OperandKind {
    fn from(value: char) -> OperandKind {
        match value {
            'r' => OperandKind::Register,
            'm' => OperandKind::ModRM,
            'v' => OperandKind::Vex,
            'i' => OperandKind::Immediate,
            's' => OperandKind::Is4Imz2,
            '-' => OperandKind::Implicit,
            'x' => OperandKind::Index,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Operands {
    operands: Vec<OperandKind>,
}

impl std::fmt::Display for Operands {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for operand in self.operands.iter() {
            f.write_fmt(format_args!("{:?}", operand))?;
        }

        Ok(())
    }
}

impl From<&str> for Operands {
    fn from(value: &str) -> Operands {
        Operands {
            operands: value.trim_matches(['[', ':'])
                .chars()
                .map(|c| OperandKind::from(c))
                .collect::<Vec<OperandKind>>(),
        }
    }
}

#[derive(Debug)]
pub enum PlainCode {
    O16,
    O32,
    O64,
    Hlexr,
}

impl PlainCode {
    pub fn prefix(&self) -> String {
        match self {
            PlainCode::O64 => String::from("0x48"),
            _ => String::new(),
        }
    }
}

impl From<&str> for PlainCode {
    fn from(value: &str) -> PlainCode {
        match value {
            "o16" => PlainCode::O16,
            "o32" => PlainCode::O32,
            "o64" => PlainCode::O64,
            "hlexr" => PlainCode::Hlexr,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum ImmCode {
    Imm8,
    Imm8Unsigned,
    Imm8Extended,
    Imm16,
    Imm32,
    Imm32Extended,
    Imm64,
    Rel,
    Rel8,
    Rel16,
    Rel32,
    Opsize,
    Addrsize,
    Seg,
}

impl ImmCode {
    pub fn length() {
    }

    pub fn from(value: &str) -> Option<ImmCode> {
        match value {
            "ib" => Some(ImmCode::Imm8),
            "ib,u" => Some(ImmCode::Imm8Unsigned),
            "ib,s" => Some(ImmCode::Imm8Extended),
            "iw" => Some(ImmCode::Imm16),
            "id" => Some(ImmCode::Imm32),
            "id,s" => Some(ImmCode::Imm32Extended),
            "iq" => Some(ImmCode::Imm64),
            "rel" => Some(ImmCode::Rel),
            "rel8" => Some(ImmCode::Rel8),
            "rel16" => Some(ImmCode::Rel16),
            "rel32" => Some(ImmCode::Rel32),
            "iwd" => Some(ImmCode::Opsize),
            "iwdq" => Some(ImmCode::Addrsize),
            "seg" => Some(ImmCode::Seg),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    Basic {
        value: u8,
    },
    Reg {
        value: u8,
    },
    PlainCode {
        code: PlainCode,
    },
    ImmCode {
        code: ImmCode,
    },
    Rm,
}

impl Opcode {
    pub fn pattern(&self) -> String {
        match self {
            Opcode::Basic { value } => format!("{:#x?},", value),
            // TODO: maybe use or operator here?
            Opcode::Reg { value } => format!("reg,"),
            Opcode::PlainCode { code } => code.prefix(),
            Opcode::ImmCode { code } => 
        }
    }
}

impl TryFrom<&str> for Opcode {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Opcode, Box<dyn std::error::Error>> {
        match value {
            "/r" => Ok(Opcode::Rm),
            _ => {
                if let Ok(value) = u8::from_str_radix(value, 16) {
                    Ok(Opcode::Basic {
                        value,
                    })
                } else if value.starts_with("/") {
                    let last = value.chars()
                        .last()
                        .ok_or::<Box<dyn std::error::Error>>("failed to get last character".into())?
                        .to_string();

                    Ok(Opcode::Reg {
                        value: u8::from_str_radix(last.as_str(), 10)?,
                    })
                } else if let Some(code) = ImmCode::from(value) {
                    Ok(Opcode::ImmCode {
                        code,
                    })
                } else {
                    Ok(Opcode::PlainCode {
                        code: PlainCode::from(value),
                    })
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    mnemonic: String,
    operands: Operands,
    opcodes: Vec<Opcode>,
}

impl Instruction {
    pub fn parse(line: &str) -> Option<Instruction> {
        let mut parts = line.split('\t').filter(|token| !token.is_empty());

        let mnemonic = parts.next().map(|mnemonic| mnemonic.to_string())?;

        parts.next();

        let operands = parts.next().map(|operands| Operands::from(operands))?;

        let opcodes = parts.next()
            .map(|part| {
                part.trim_matches(']')
                    .split(' ')
                    .filter_map(|opcode| Opcode::try_from(opcode).ok())
                    .collect::<Vec<Opcode>>()
            })?;

        Some(Instruction {
            mnemonic,
            operands,
            opcodes,
        })
    }
}

pub struct Schematic {
    lines: Lines<BufReader<File>>,
    kind: File,
    out: File,
}

impl Schematic {
    pub fn new(path: &Path) -> Result<Schematic, Box<dyn std::error::Error>> {
        let reader = BufReader::new(File::open("schematics/simple.dat")?);

        Ok(Schematic {
            lines: reader.lines(),
            kind: File::create(path.join("kind.rs"))?,
            out: File::create(path.join("instructions.rs"))?,
        })
    }

    fn add_inst(&mut self, instruction: Instruction) -> Result<(), Box<dyn std::error::Error>> {
        self.kind.write_all(format!("{}{} {{", instruction.mnemonic, instruction.operands).as_bytes())?;

        for operand in instruction.operands.operands.iter() {
            if let Some(size) = operand.size() {
                self.kind.write_all(format!("{:?}: {},", operand, size).as_bytes())?;
            }
        }

        self.kind.write_all(b"}")?;

        self.out.write_all(b"[")?;

        for opcode in instruction.opcodes.iter() {
        }

        Ok(())
    }

    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.kind.write_all(b"
            pub enum Kind {
        ")?;

        self.out.write_all(b"
            pub mod kind;

            use kind::Kind;

            pub struct Instruction {
                kind: Kind,
                size: u8,
            }

            pub fn parse(bytes: [u8; 16]) -> Instruction {
                match bytes {
        ")?;

        while let Some(line) = self.lines.next() {
            match line {
                Ok(line) => {
                    if let Some(instruction) = Instruction::parse(&line) {
                        self.add_inst(instruction)?;
                    }
                },
                Err(err) => {
                    println!("cargo:warning=err: {:?}", err);
                },
            }
        }

        self.out.write_all(b"
                _ => unreachable!(),
            }}
        ")?;

        self.kind.write_all(b"}").map_err(|err| err.into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = env::var_os("OUT_DIR").ok_or::<Box<dyn std::error::Error>>("failed to get OUT_DIR".into())?;
    let mut schematic = Schematic::new(Path::new(&dir))?;

    schematic.run()?;

    println!("cargo:warning={:?}", std::fs::read_to_string(Path::new(&dir).join("kind.rs"))?);
    println!("cargo:warning={:?}", std::fs::read_to_string(Path::new(&dir).join("instructions.rs"))?);

    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}


