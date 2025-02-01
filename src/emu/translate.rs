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

    pub fn process(&mut self, instruction: Instruction, bytes: &[u8]) {
        match instruction.kind {
             _ => self.out.extend(bytes),
        }
    }
}


