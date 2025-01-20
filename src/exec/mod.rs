mod decode;

use object::{Object, ObjectSegment, File};
use log::{info, warn};

use std::fs;


#[derive(Debug)]
pub struct Segment {
    address: u64,
    data: Vec<u8>,
}

impl Segment {
    pub fn new(address: u64, data: Vec<u8>) -> Segment {
        Segment {
            address,
            data,
        }
    }

    pub fn read(&self, address: u64) {
        // TODO: implement read
    }
}

#[derive(Debug)]
pub struct Memory {
    segments: Vec<Segment>,
    base: u64,
}

impl Memory {
    pub fn new(file: &File) -> Memory {
        // TODO: preserve memory flags

        let segments = file.segments()
            .filter_map(|segment| {
                segment.data()
                    .map(|data| Segment::new(segment.address(), data.to_vec()))
                    .ok()
            })
            .collect::<Vec<Segment>>();

        Memory {
            segments,
            base: file.relative_address_base(),
        }
    }

    pub fn read(&self) {
    }
}

#[derive(Debug)]
pub struct Executor {
    memory: Memory,
    entry: u64,
}

impl Executor {
    pub fn new(path: &str) -> Result<Executor, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        let file = File::parse(&*data)?;

        Ok(Executor {
            memory: Memory::new(&file),
            entry: file.entry(),
        })
    }

    pub fn exec(&mut self) {
        info!("executor: {:x?}", self);
    }
}


