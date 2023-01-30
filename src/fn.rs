use std::{alloc::Allocator, marker::Tuple};
use crate::ThinBox;

impl<F: ?Sized + Fn<Args>, Args: Tuple, A: Allocator> Fn<Args> for ThinBox<F, A> {
    #[inline]
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        <F as Fn<Args>>::call(self, args)
    }
}

impl<F: ?Sized + FnMut<Args>, Args: Tuple, A: Allocator> FnMut<Args> for ThinBox<F, A> {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        <F as FnMut<Args>>::call_mut(self, args)
    }
}

#[cfg(not(feature = "unsized_locals"))]
impl<F: ?Sized + FnMut<Args>, Args: Tuple, A: Allocator> FnOnce<Args> for ThinBox<F, A> {
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
        <F as FnMut<Args>>::call_mut(&mut self, args)
    }
}

#[cfg(feature = "unsized_locals")]
impl<F: ?Sized + FnOnce<Args>, Args: Tuple, A: Allocator> FnOnce<Args> for ThinBox<F, A> {
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
        <F as FnOnce<Args>>::call_once(*self, args)
    }
}