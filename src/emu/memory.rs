use object::{File, Object, ObjectSegment};

use std::ops::{Index, Range};

// NOTE: there can only be null values after the segment address as read asserts that both the
// start and end range are beyond the segment address


#[derive(Debug, PartialEq)]
pub struct SegmentRead<'a> {
    data: &'a [u8],
    null: Range<usize>,
}

impl<'a> SegmentRead<'a> {
    pub fn new(data: &'a [u8], null: Range<usize>) -> SegmentRead<'a> {
        SegmentRead {
            data,
            null,
        }
    }
}

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

    pub fn read<'a>(&'a self, range: std::ops::Range<usize>) -> SegmentRead<'a> {
        assert!(range.start >= self.address && range.end >= self.address);

        let data = &self.data[range.start - self.address..(range.end - self.address).min(self.data.len())];

        SegmentRead {
            data,
            null: range.start + data.len()..range.end,
        }
    }

    pub fn len(&self, address: usize) -> Option<usize> {
        address.checked_sub(self.address)
            .and_then(|address| address.checked_sub(self.data.len()))
    }
}

#[derive(Debug)]
pub struct Memory {
    segments: Vec<Segment>,
    base: usize,
}

impl Index<Range<usize>> for Memory {
    type Output = Vec<u8>;

    fn index(&self, range: Range<usize>) -> Vec<u8> {
    }
}

impl Memory {
    pub fn new(file: &File) -> Memory {
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

    pub fn write(&mut self, data: SegmentRead) {
    }

    pub fn get_segment<'a>(&'a self, address: usize) -> Option<&'a Segment> {
        self.segments.iter()
            .filter(|segment| address >= segment.address)
            .min_by(|x, y| x.address.cmp(&y.address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment() {
        let segment = Segment::new(100, vec![1, 2, 3]);

        assert_eq!(segment.read(100..110), SegmentRead::new(&[1, 2, 3], 103..110));
        assert_eq!(segment.read(100..102), SegmentRead::new(&[1, 2], 102..102));
        assert_eq!(segment.read(100..104), SegmentRead::new(&[1, 2, 3], 103..104));
    }
}


