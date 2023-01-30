use std::{alloc::Allocator, iter::{FusedIterator}};
use crate::ThinBox;

impl<B, I: FromIterator<B>, A: Allocator + Default> FromIterator<B> for ThinBox<I, A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        Self::new_in(I::from_iter(iter), Default::default())
    }
}

impl<B, I: Extend<B>, A: Allocator> Extend<B> for ThinBox<I, A> {
    #[inline]
    fn extend<T: IntoIterator<Item = B>>(&mut self, iter: T) {
        I::extend(self, iter)
    }
}

impl<T: ?Sized + Iterator, A: Allocator> Iterator for ThinBox<T, A> {
    type Item = T::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        T::next(self)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        T::size_hint(self)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        T::nth(self, n)
    }
}

impl<T: ?Sized + DoubleEndedIterator, A: Allocator> DoubleEndedIterator for ThinBox<T, A> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        T::next_back(self)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        T::nth_back(self, n)
    }
}

impl<T: ?Sized + ExactSizeIterator, A: Allocator> ExactSizeIterator for ThinBox<T, A> {
    #[inline]
    fn len(&self) -> usize {
        T::len(self)
    }
}

impl<T: ?Sized + FusedIterator, A: Allocator> FusedIterator for ThinBox<T, A> {}