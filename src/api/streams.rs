use super::*;

pub trait Stream<T, R> {
    fn open(reader : R) -> Self ;
    fn has_next(&self)  -> bool ;
    fn next(&mut self)  -> T    ;
    fn get_index(&self) -> usize;
}

pub struct ByteStream(ByteReader);
pub struct NullStream;
pub struct ByteSliceStream<'a> {
    slice   : &'a [u8],
    index   : usize
}

impl Stream<u8, ByteReader> for ByteStream {
    fn open(reader : ByteReader) -> ByteStream {
        ByteStream(reader)
    }

    fn has_next(&self) -> bool {
        self.0.at_end()
    }

    fn next(&mut self) -> u8 {
        self.0.read().unwrap()
    }

    fn get_index(&self) -> usize {
        0 //TODO (Npex42): Add Proper Indexing
    }
}

impl Stream<u8, u8> for NullStream {
    fn open(reader : u8) -> NullStream {
        NullStream
    }

    fn has_next(&self) -> bool {
        true
    }

    fn next(&mut self) -> u8 {
        0
    }

    fn get_index(&self) -> usize {
        0
    }
} 

impl<'a> Stream<u8, &'a [u8]> for ByteSliceStream<'a> {

    fn open(slice : &'a [u8]) -> ByteSliceStream {

        ByteSliceStream {slice : slice, index : 0}

    }

    fn has_next(&self) -> bool {
        self.index < self.slice.len()
    }

    fn next(&mut self) -> u8 {
        let output = self.slice[self.index];
        self.index += 1;
        output
    }

    fn get_index(&self) -> usize {
        self.index
    }
}