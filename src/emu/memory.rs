use object::{File, Object, ObjectSegment};

use std::borrow::Borrow;
use std::ops::Range;


pub trait RangeExt {
    fn intersection<T>(&self, range: T) -> Option<Range<usize>> where T: Borrow<Range<usize>>;

    fn subsection_of<T>(&self, range: T) -> bool where T: Borrow<Range<usize>>;
}

impl RangeExt for Range<usize> {
    fn intersection<T>(&self, range: T) -> Option<Range<usize>>
    where T:
        Borrow<Range<usize>>,
    {
        (range.borrow().start <= self.end && range.borrow().end >= self.start)
            .then(|| self.start.max(range.borrow().start)..(self.end).min(range.borrow().end))
    }

    fn subsection_of<T>(&self, range: T) -> bool
    where T:
        Borrow<Range<usize>>,
    {
        self.start > range.borrow().start && self.end < range.borrow().end
    }
}

#[derive(Debug, PartialEq)]
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

    pub fn range(&self) -> Range<usize> {
        self.address..self.address + self.data.len()
    }

    pub fn read<'a, T>(&'a self, range: T) -> &'a [u8]
    where T:
        Borrow<Range<usize>>,
    {
        assert!(range.borrow().start >= self.address && range.borrow().end >= self.address);

        &self.data[range.borrow().start - self.address..(range.borrow().end - self.address).min(self.data.len())]
    }

    pub fn trim<T>(&mut self, range: T)
    where T:
        Borrow<Range<usize>> + std::fmt::Debug,
    {
        assert!(range.borrow().start >= self.address && range.borrow().end >= self.address);

        if range.borrow().end < self.address + self.data.len() {
            self.data = self.data[range.borrow().end - self.address..].to_vec();

            self.address = range.borrow().end;
        } else if range.borrow().start > self.address {
            self.data = self.data[..range.borrow().start - self.address].to_vec();
        }
    }
}

#[derive(Debug, Default)]
pub struct Memory {
    pub segments: Vec<Segment>,
    pub base: usize,
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

        for (intersection, segment) in self.segments.iter().filter_map(|segment| segment.range().intersection(&range).map(|intersection| (intersection, segment))) {
            let data = segment.read(&intersection);

            buffer[intersection.start - range.start..intersection.end - range.start].copy_from_slice(&data);
        }

        buffer
    }

    pub fn write(&mut self, address: usize, data: &[u8]) {
        let mut new: Vec<Segment> = vec![Segment::new(address, data.to_vec())];

        self.segments.retain(|segment| !segment.range().subsection_of(address..address + data.len()));

        let intersections = self.segments.iter_mut()
            .filter_map(|segment| segment.range().intersection(address..address + data.len()).map(|intersection| (intersection, segment)));

        for (intersection, segment) in intersections {
            if intersection.subsection_of(segment.address..segment.address + segment.data.len()) {
                let rest = segment.data.split_off(intersection.start - segment.address);

                new.push(Segment::new(intersection.end, rest[intersection.end - intersection.start..].to_vec()));
            } else {
                segment.trim(intersection);
            }
        }

        self.segments.extend(new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory() {
        let mut memory = Memory::default();

        memory.write(99, &[28, 54, 45, 74]);

        assert_eq!(memory.segments, vec![Segment::new(99, vec![28, 54, 45, 74])]);
        assert_eq!(memory.read(100..105), vec![54, 45, 74, 0, 0]);

        memory.write(101, &[85, 93]);

        assert_eq!(memory.segments, vec![Segment::new(99, vec![28, 54]), Segment::new(101, vec![85, 93])]);
        assert_eq!(memory.read(100..105), vec![54, 85, 93, 0, 0]);

        memory.write(100, &[14, 88]);

        assert_eq!(memory.segments, vec![Segment::new(99, vec![28]), Segment::new(102, vec![93]), Segment::new(100, vec![14, 88])]);

        memory.write(200, &[20, 30, 40, 50, 60, 70, 80]);
        memory.write(202, &[49, 50, 51]);

        assert_eq!(memory.segments, vec![
            Segment::new(99, vec![28]),
            Segment::new(102, vec![93]),
            Segment::new(100, vec![14, 88])
        ]);
    }
}


