use std::{io::*, alloc::Allocator};
use crate::ThinBox;

impl<T: ?Sized + Read, A: Allocator> Read for ThinBox<T, A> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        T::read(self, buf)
    }
}

impl<T: ?Sized + BufRead, A: Allocator> BufRead for ThinBox<T, A> {
    #[inline]
    fn fill_buf(&mut self) -> Result<&[u8]> {
        T::fill_buf(self)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        T::consume(self, amt)
    }
}

impl<T: ?Sized + Write, A: Allocator> Write for ThinBox<T, A> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        T::write(self, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        T::flush(self)
    }
}

impl<T: ?Sized + Seek, A: Allocator> Seek for ThinBox<T, A> {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        T::seek(self, pos)
    }
}