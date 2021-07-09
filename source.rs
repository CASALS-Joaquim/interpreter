// tokens
use std::io::{
    BufRead,
    Read,
};

use std::num::NonZeroUsize;

pub struct Source<Src: BufRead> {
    line: Option<Vec<u8>>,
    raw_column: usize,
    column: usize,
    src: Enumerate<Lines<BufRead>>
}

impl<Src: BufRead> Source<Src> {
    pub fn read<'a>(&mut self, buffer: &'a mut [u8]) -> Option<NonZeroUsize> {
        let count = buffer.len();
        
        let len = buffer
            .iter_mut()
            .map(|buf| {
                *buf = self.read_byte::<u8>();
                *buf
            })
            .collect::<Vec<_>>()
            .len();
        let delta = len - count;
        if delta == 0 {
            None
        } else {
            delta
        }
    }
}