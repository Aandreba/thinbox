use std::{alloc::{Allocator, Global}, marker::Tuple};
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

impl<F: ?Sized + FnMut<Args>, Args: Tuple, A: Allocator> FnOnce<Args> for ThinBox<F, A> {
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
        <F as FnMut<Args>>::call_mut(&mut self, args)
    }
}

impl<'a, Args: Tuple, Output> ThinBox<dyn 'a + FnMut<Args, Output = Output>> {
    /// Creates a new [`dyn FnMut`] from an [`impl FnOnce`] without checking if it has been ran before. Calling this function multiple times is undefined behaviour.
    #[inline]
    pub unsafe fn from_once_unchecked<F: 'a + FnOnce<Args, Output = Output>> (f: F) -> Self {
        Self::from_once_unchecked_in(f, Global)
    }

    /// Creates a new [`dyn FnMut`] from an [`impl FnOnce`]. If the new underlying function is called multiple times, it will panic.
    #[inline]
    pub fn from_once<F: 'a + FnOnce<Args, Output = Output>> (f: F) -> Self {
        Self::from_once_in(f, Global)
    }
}

impl<'a, Args: Tuple, Output> ThinBox<dyn 'a + FnMut<Args, Output = Option<Output>>> {
    /// Creates a new [`dyn FnMut`] from an [`impl FnOnce`]. If the new underlying function is called multiple times, it will return `None`.
    #[inline]
    pub fn from_once_checked<F: 'a + FnOnce<Args, Output = Output>> (f: F) -> Self {
        Self::from_once_checked_in(f, Global)
    }
}

impl<'a, Args: Tuple, Output, A: Allocator> ThinBox<dyn 'a + FnMut<Args, Output = Output>, A> {
    /// Creates a new [`dyn FnMut`] from an [`impl FnOnce`] without checking if it has been ran before. Calling this function multiple times is undefined behaviour.
    #[inline]
    pub unsafe fn from_once_unchecked_in<F: 'a + FnOnce<Args, Output = Output>> (f: F, alloc: A) -> Self {
        #[repr(transparent)]
        struct UncheckedImpl<F> (Option<F>);

        impl<Args: Tuple, F: FnOnce<Args>> FnOnce<Args> for UncheckedImpl<F> {
            type Output = F::Output;

            #[inline]
            extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
                unsafe { F::call_once(self.0.take().unwrap_unchecked(), args) }
            }
        }

        impl<Args: Tuple, F: FnOnce<Args>> FnMut<Args> for UncheckedImpl<F> {
            #[inline]
            extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
                unsafe { F::call_once(self.0.take().unwrap_unchecked(), args) }
            }
        }

        return Self::new_unsize_in(UncheckedImpl(Some(f)), alloc)
    }

    /// Creates a new [`dyn FnMut`] from an [`impl FnOnce`]. If the new underlying function is called multiple times, it will panic.
    #[inline]
    pub fn from_once_in<F: 'a + FnOnce<Args, Output = Output>> (f: F, alloc: A) -> Self {
        #[repr(transparent)]
        struct CheckedImpl<F> (Option<F>);

        impl<Args: Tuple, F: FnOnce<Args>> FnOnce<Args> for CheckedImpl<F> {
            type Output = F::Output;

            #[inline]
            extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
                F::call_once(self.0.take().expect("tried to execute FnOnce multiple times"), args)
            }
        }

        impl<Args: Tuple, F: FnOnce<Args>> FnMut<Args> for CheckedImpl<F> {
            #[inline]
            extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
                F::call_once(self.0.take().expect("tried to execute FnOnce multiple times"), args)
            }
        }

        return Self::new_unsize_in(CheckedImpl(Some(f)), alloc)
    }
}

impl<'a, Args: Tuple, Output, A: Allocator> ThinBox<dyn 'a + FnMut<Args, Output = Option<Output>>, A> {
    /// Creates a new [`dyn FnMut`] from an [`impl FnOnce`]. If the new underlying function is called multiple times, it will return `None`.
    #[inline]
    pub fn from_once_checked_in<F: 'a + FnOnce<Args, Output = Output>> (f: F, alloc: A) -> Self {
        #[repr(transparent)]
        struct CheckedImpl<F> (Option<F>);

        impl<Args: Tuple, F: FnOnce<Args>> FnOnce<Args> for CheckedImpl<F> {
            type Output = Option<F::Output>;

            #[inline]
            extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
                self.0.take().map(|f| f.call_once(args))
            }
        }

        impl<Args: Tuple, F: FnOnce<Args>> FnMut<Args> for CheckedImpl<F> {
            #[inline]
            extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
                self.0.take().map(|f| f.call_once(args))
            }
        }

        return Self::new_unsize_in(CheckedImpl(Some(f)), alloc)
    }
}