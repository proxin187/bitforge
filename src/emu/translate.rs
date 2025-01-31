use crate::Instruction;


pub struct Translate {
    out: Vec<u8>,
}

impl Translate {
    pub fn new() -> Translate {
        Translate {
            out: Vec::new(),
        }
    }

    pub fn process(&mut self, instruction: Instruction, bytes: &[u8]) {
    }
}


