#![allow(unused_imports)]

use std::alloc::Allocator;
use docfg::docfg;
use crate::ThinBox;

#[docfg(feature = "serde")]
impl<T: ?Sized + serde::Serialize, A: Allocator> serde::Serialize for ThinBox<T, A> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        T::serialize(self, serializer)
    }
}

#[docfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>, A: Allocator + Default> serde::Deserialize<'de> for ThinBox<T, A> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let v = T::deserialize(deserializer)?;
        return Ok(Self::new_in(v, Default::default()))
    }
}