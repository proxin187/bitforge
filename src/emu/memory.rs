use object::{File, Object, ObjectSegment};

use std::borrow::Borrow;
use std::ops::Range;


#[derive(Debug)]
pub struct Segment {
    address: usize,
    data: Vec<u8>,
}

impl Segment {
    pub fn new(address: usize, data: Vec<u8>) -> Segment {
        Segment {
            address,
            data,
        }
    }

    pub fn read<'a, T>(&'a self, range: T) -> &'a [u8]
    where T:
        Borrow<Range<usize>>,
    {
        assert!(range.borrow().start >= self.address && range.borrow().end >= self.address);

        &self.data[range.borrow().start - self.address..(range.borrow().end - self.address).min(self.data.len())]
    }

    pub fn intersection<T>(&self, range: T) -> Option<Range<usize>>
    where T:
        Borrow<Range<usize>>,
    {
        (range.borrow().start <= self.address + self.data.len() && range.borrow().end >= self.address)
            .then(|| self.address.max(range.borrow().start)..(self.address + self.data.len()).min(range.borrow().end))
    }

    pub fn len(&self, address: usize) -> Option<usize> {
        address.checked_sub(self.address)
            .and_then(|address| address.checked_sub(self.data.len()))
    }
}

#[derive(Debug, Default)]
pub struct Memory {
    segments: Vec<Segment>,
    base: usize,
}

impl<'a> From<&File<'a>> for Memory {
    fn from(file: &File) -> Memory {
        // TODO: preserve memory flags

        let segments = file.segments()
            .filter_map(|segment| {
                segment.data()
                    .map(|data| Segment::new(segment.address() as usize, data.to_vec()))
                    .ok()
            })
            .collect::<Vec<Segment>>();

        Memory {
            segments,
            base: file.relative_address_base() as usize,
        }
    }

}

impl Memory {
    pub fn read(&self, range: Range<usize>) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; range.end - range.start];

        for (intersection, segment) in self.segments.iter().filter_map(|segment| segment.intersection(&range).map(|intersection| (intersection, segment))) {
            let data = segment.read(&intersection);

            buffer[intersection.start - range.start..intersection.end - range.start].copy_from_slice(&data);
        }

        buffer
    }

    // TODO: this function will clean segments that overlap
    pub fn clean(&mut self) {
    }

    // TODO: we only need to get the first intersecting and resize it to fit the data, then we need to
    // trim/remove the other segments which are overlapping
    pub fn write(&mut self, address: usize, data: &[u8]) {
        let intersections = self.segments.iter()
            .filter_map(|segment| segment.intersection(address..address + data.len()).map(|intersection| (intersection, segment)))
            .collect::<Vec<(Range<usize>, &Segment)>>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory() {
        let mut memory = Memory::default();

        memory.write(99, &[28, 54, 45, 74]);

        assert_eq!(memory.read(100..105), vec![54, 45, 74, 0, 0]);
    }
}


