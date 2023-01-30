use std::{alloc::Allocator, error::Error, hash::Hash};
use crate::ThinBox;

impl<T: Clone, A: Allocator + Clone> Clone for ThinBox<T, A> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new_in(T::clone(self), self.alloc.clone())
    }
}

impl<T: Default, A: Allocator + Default> Default for ThinBox<T, A> {
    #[inline]
    fn default() -> Self {
        Self::new_in(Default::default(), Default::default())
    }
}

impl<T: ?Sized + PartialEq, A: Allocator, B: Allocator> PartialEq<ThinBox<T, B>> for ThinBox<T, A> {
    #[inline]
    fn eq(&self, other: &ThinBox<T, B>) -> bool {
        T::eq(self, other)
    }
}

impl<T: ?Sized + PartialEq, A: Allocator> PartialEq<T> for ThinBox<T, A> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        T::eq(&self, other)
    }
}

impl<T: ?Sized + PartialOrd, A: Allocator, B: Allocator> PartialOrd<ThinBox<T, B>> for ThinBox<T, A> {
    #[inline]
    fn partial_cmp(&self, other: &ThinBox<T, B>) -> Option<std::cmp::Ordering> {
        T::partial_cmp(self, other)
    }
}

impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd<T> for ThinBox<T, A> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        T::partial_cmp(self, other)
    }
}

impl<T: ?Sized + Ord, A: Allocator> Ord for ThinBox<T, A> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        T::cmp(self, other)
    }
}

impl<T: ?Sized + Hash, A: Allocator> Hash for ThinBox<T, A> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl<T: ?Sized + Error, A: Allocator> Error for ThinBox<T, A> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        T::source(self)
    }
}

impl<T: ?Sized + Eq, A: Allocator> Eq for ThinBox<T, A> {}