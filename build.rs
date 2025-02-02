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
    pub fn pattern(&self, size: usize) -> String {
        match self {
            OperandKind::Register => String::from("rm"),
            OperandKind::ModRM => String::from("reg"),
            OperandKind::Vex => String::from("vex"),
            OperandKind::Immediate => format!("imm{}", size * 8),
            OperandKind::Is4Imz2 => String::from("is4imz2"),
            OperandKind::Implicit => String::from("implicit"),
            OperandKind::Index => String::from("index"),
        }
    }

    pub fn size(&self, imm_size: Option<usize>) -> Option<String> {
        match self {
            OperandKind::Register
                | OperandKind::ModRM => Some(String::from("ModRM")),
            OperandKind::Vex => Some(String::from("u8")),
            OperandKind::Is4Imz2
                | OperandKind::Index => Some(String::from("u64")),
            OperandKind::Immediate => Some(format!("u{}", imm_size.expect("immediate operand but no immediate opcode") * 8)),
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
    pub fn size(&self) -> usize {
        match self {
            PlainCode::O64 => 1,
            _ => 0,
        }
    }

    pub fn prefix(&self) -> String {
        match self {
            // TODO: this currently assumes REX.W
            PlainCode::O64 => String::from("0x48,"),
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
    pub fn pattern(&self, deref: bool) -> String {
        (1..=self.size())
            .map(|byte| format!("{}imm{},", deref.then(|| "*").unwrap_or_default(), byte * 8))
            .collect::<String>()
            .to_string()
    }

    pub fn value_ref(&self) -> String {
        format!("imm{}: u{}::from_ne_bytes([{}]),", self.size() * 8, self.size() * 8, self.pattern(true))
    }

    // TODO: not sure if these sizes are correct, check later, also some sizes are implicit
    pub fn size(&self) -> usize {
        match self {
            ImmCode::Imm8
                | ImmCode::Imm8Unsigned
                | ImmCode::Imm8Extended
                | ImmCode::Rel8 => 1,
            ImmCode::Imm16
                | ImmCode::Rel16 => 2,
            ImmCode::Imm32
                | ImmCode::Imm32Extended
                | ImmCode::Rel32 => 4,
            ImmCode::Imm64
                | ImmCode::Rel
                | ImmCode::Addrsize
                | ImmCode::Opsize
                | ImmCode::Seg => 8,
        }
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
    pub fn identifier(&self) -> Option<String> {
        match self {
            Opcode::ImmCode { code } => Some(format!("{:?}", code)),
            Opcode::PlainCode { code } => Some(format!("{:?}", code)),
            _ => None,
        }
    }

    pub fn imm_size(&self) -> Option<usize> {
        match self {
            Opcode::ImmCode { code } => Some(code.size()),
            _ => None,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Opcode::Rm | Opcode::Reg { .. } | Opcode::Basic { .. } => 1,
            Opcode::PlainCode { code } => code.size(),
            Opcode::ImmCode { code } => code.size(),
        }
    }

    pub fn value_ref(&self) -> String {
        match self {
            Opcode::Basic { .. } | Opcode::PlainCode { .. } => String::new(),
            Opcode::ImmCode { code } => code.value_ref(),
            Opcode::Reg { .. } => String::from("reg: ModRM::new(*reg),"),
            Opcode::Rm => String::from("rm: ModRM::new(*rm),"),
        }
    }

    pub fn pattern(&self) -> String {
        match self {
            Opcode::Basic { value } => format!("{:#x?},", value),
            // TODO: maybe use "or" operator here in order to make sure we only match with values
            // that contains the expected register value?
            Opcode::Reg { value } => format!("reg,"),
            Opcode::PlainCode { code } => code.prefix(),
            Opcode::ImmCode { code } => code.pattern(false),
            Opcode::Rm => format!("rm,"),
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

        // TODO: we will have to parse the args and generate better instruction decoding with this.
        // eg. we can have only one instruction per mnemonic and multiple options for the args of
        // the instruction through for example an enum
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

    pub fn identifier(&self) -> String {
        format!("{}{}", self.mnemonic, self.opcodes.iter().filter_map(|opcode| opcode.identifier()).collect::<String>())
    }

    pub fn size(&self) -> usize {
        self.opcodes.iter().map(|opcode| opcode.size()).sum()
    }

    pub fn imm_size(&self) -> Option<usize> {
        self.opcodes.iter()
            .filter_map(|opcode| opcode.imm_size())
            .next()
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
        self.kind.write_all(instruction.identifier().as_bytes())?;

        if !instruction.operands.operands.is_empty() {
            self.kind.write_all(b"{")?;

            for operand in instruction.operands.operands.iter() {
                if let Some((type_, size)) = operand.size(instruction.imm_size()).zip(instruction.imm_size()) {
                    self.kind.write_all(format!("{}: {},", operand.pattern(size), type_).as_bytes())?;
                }
            }

            self.kind.write_all(b"}")?;
        }

        self.kind.write_all(b",")?;

        self.out.write_all(b"[")?;

        for opcode in instruction.opcodes.iter() {
            self.out.write_all(opcode.pattern().as_bytes())?;
        }

        self.out.write_all(format!("..] => Instruction {{ size: {}, kind: Kind::{} {{", instruction.size(), instruction.identifier()).as_bytes())?;

        for opcode in instruction.opcodes.iter() {
            self.out.write_all(opcode.value_ref().as_bytes())?;
        }

        self.out.write_all(b"}},")?;

        Ok(())
    }

    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: we need to implement SIB displacement bytes, maybe we can generate multiple
        // versions of one instruction for each displacement or maybe only for each different mod
        // byte

        self.kind.write_all(b"
            #[derive(Debug)]
            pub struct ModRM {
                pub mod_: u8,
                pub reg: u8,
                pub rm: u8,
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

            #[derive(Debug)]
            pub enum Kind {
        ")?;

        self.out.write_all(b"
            pub mod kind;

            use kind::{Kind, ModRM};

            #[derive(Debug)]
            pub struct Instruction {
                size: u8,
                kind: Kind,
            }

            pub fn parse(bytes: &[u8]) -> Instruction {
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


