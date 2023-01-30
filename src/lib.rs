#![feature(
    ptr_metadata,
    unsize,
    alloc_layout_extra,
    allocator_api,
    layout_for_ptr,
    pointer_byte_offsets,
    unboxed_closures,
    fn_traits,
    tuple_trait
)]
#![cfg_attr(feature = "unsized_locals", allow(incomplete_features), feature(unsized_locals))]
#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )+
    }
}

use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    fmt::{Debug, Display},
    marker::{PhantomData, Unsize},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr::{NonNull, Pointee},
};

flat_mod! { r#fn, iter, ops, future, ser_de, io }

pub struct ThinBox<T: ?Sized, A: Allocator = Global> {
    ptr: NonNull<u8>,
    alloc: A,
    _phtm: PhantomData<NonNull<T>>,
}

impl<T> ThinBox<T> {
    #[inline]
    pub fn new(v: T) -> Self {
        Self::new_in(v, Global)
    }

    #[inline]
    pub fn try_new(t: T) -> Result<Self, AllocError> {
        Self::try_new_in(t, Global)
    }
}

impl<T: ?Sized> ThinBox<T> {
    #[inline]
    pub fn new_unsize<U: Unsize<T>>(v: U) -> Self {
        Self::new_unsize_in(v, Global)
    }

    #[inline]
    pub fn try_new_unsize<U: Unsize<T>>(v: U) -> Result<Self, AllocError> {
        Self::try_new_unsize_in(v, Global)
    }
}

impl<T, A: Allocator> ThinBox<T, A> {
    #[inline]
    pub fn new_in(v: T, alloc: A) -> Self {
        Self::try_new_in(v, alloc).expect("error allocating thin value")
    }

    #[inline]
    pub fn try_new_in(t: T, alloc: A) -> Result<Self, AllocError> {
        unsafe { Self::try_new_by_parts_in((), t, alloc) }
    }
}

impl<T: ?Sized, A: Allocator> ThinBox<T, A> {
    #[inline]
    pub fn new_unsize_in<U: Unsize<T>>(v: U, alloc: A) -> Self {
        Self::try_new_unsize_in(v, alloc).expect("error allocating thin value")
    }

    #[inline]
    pub fn try_new_unsize_in<U: Unsize<T>>(v: U, alloc: A) -> Result<Self, AllocError> {
        unsafe { Self::try_new_by_parts_in(core::ptr::metadata(&v as &T), v, alloc) }
    }

    unsafe fn try_new_by_parts_in<U>(
        meta: <T as Pointee>::Metadata,
        v: U,
        alloc: A,
    ) -> Result<Self, AllocError> {
        let (layout, offset) =
            match Layout::new::<<T as Pointee>::Metadata>().extend(Layout::new::<U>()) {
                Ok(x) => x,
                Err(e) => {
                    #[cfg(debug_assertions)]
                    eprintln!("{e}");
                    return Err(AllocError);
                }
            };

        let ptr = alloc.allocate(layout)?.as_ptr().cast::<u8>().add(offset);
        debug_assert!(!ptr.is_null());

        unsafe {
            core::ptr::write(ptr.cast(), v);
            core::ptr::write(
                ptr.sub(core::mem::size_of::<<T as Pointee>::Metadata>())
                    .cast(),
                meta,
            );
        }

        return Ok(Self {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            alloc,
            _phtm: PhantomData,
        });
    }
}

impl<T: ?Sized> ThinBox<T> {
    #[inline]
    pub fn into_raw(self) -> NonNull<()> {
        let this = ManuallyDrop::new(self);
        return this.ptr.cast();
    }

    #[inline]
    pub unsafe fn from_raw(ptr: NonNull<()>) -> Self {
        return Self::from_raw_with_alloc(ptr, Global);
    }
}

impl<T: ?Sized, A: Allocator> ThinBox<T, A> {
    #[inline]
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        unsafe {
            let this = ManuallyDrop::new(self);
            let value = core::ptr::read(this.deref().deref());

            let (layout, offset) = Layout::new::<<T as Pointee>::Metadata>()
                .extend(Layout::new::<T>())
                .unwrap_unchecked();

            let ptr = this.value_ptr().sub(offset);
            this.alloc.deallocate(NonNull::new_unchecked(ptr), layout);
            return value
        }
    }

    #[inline]
    pub fn into_inner_with_alloc(self) -> (T, A)
    where
        T: Sized,
    {
        unsafe {
            let this = ManuallyDrop::new(self);
            let value = core::ptr::read(this.deref().deref());

            let (layout, offset) = Layout::new::<<T as Pointee>::Metadata>()
                .extend(Layout::new::<T>())
                .unwrap_unchecked();

            let ptr = this.value_ptr().sub(offset);
            this.alloc.deallocate(NonNull::new_unchecked(ptr), layout);
            return (value, core::ptr::read(&this.alloc))
        }
    }

    #[inline]
    pub fn into_raw_with_alloc(self) -> (NonNull<()>, A) {
        let this = ManuallyDrop::new(self);
        return unsafe { (this.ptr.cast(), core::ptr::read(&this.alloc)) };
    }

    #[inline]
    pub unsafe fn from_raw_with_alloc(ptr: NonNull<()>, alloc: A) -> Self {
        return Self {
            ptr: ptr.cast(),
            alloc,
            _phtm: PhantomData,
        };
    }

    #[inline]
    pub fn metadata(&self) -> <T as Pointee>::Metadata {
        unsafe {
            *self
                .ptr
                .as_ptr()
                .sub(core::mem::size_of::<<T as Pointee>::Metadata>())
                .cast()
        }
    }

    #[inline]
    pub fn allocator(&self) -> &A {
        return &self.alloc;
    }

    #[inline]
    unsafe fn value_ptr(&self) -> *mut u8 {
        return self.ptr.as_ptr();
    }
}

impl<T: ?Sized + Debug, A: Allocator> Debug for ThinBox<T, A> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: ?Sized + Display, A: Allocator> Display for ThinBox<T, A> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: ?Sized, A: Allocator> Deref for ThinBox<T, A> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*core::ptr::from_raw_parts(self.value_ptr() as *const (), self.metadata()) }
    }
}

impl<T: ?Sized, A: Allocator> DerefMut for ThinBox<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *core::ptr::from_raw_parts_mut(self.value_ptr() as *mut (), self.metadata()) }
    }
}


impl<T: ?Sized, A: Allocator> Drop for ThinBox<T, A> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let ptr: *mut T = self.deref_mut();
            
            let layout = Layout::for_value_raw(ptr);
            let (layout, offset) = Layout::new::<<T as Pointee>::Metadata>()
            .extend(layout)
            .unwrap_unchecked();
            
            core::ptr::drop_in_place(ptr);
            let ptr = ptr.cast::<u8>().sub(offset);
            self.alloc.deallocate(NonNull::new_unchecked(ptr), layout);
        }
    }
}

unsafe impl<T: ?Sized + Send, A: Allocator + Send> Send for ThinBox<T, A> {}
unsafe impl<T: ?Sized + Sync, A: Allocator + Sync> Sync for ThinBox<T, A> {}
impl<T: ?Sized, A: 'static + Allocator> Unpin for ThinBox<T, A> {}
