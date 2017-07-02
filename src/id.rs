use {IdRange, IntegerHandle, Identifier, FromIndex, ToIndex};
use std::marker::PhantomData;
use std::fmt;
use std::hash::{Hash, Hasher};
use num_traits::One;

#[repr(C)]
pub struct Id<Tag, Handle = u32> {
    pub handle: Handle,
    pub _marker: PhantomData<Tag>,
}

impl<T, H: fmt::Display> fmt::Debug for Id<T, H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id#{}", self.handle)
    }
}

impl<T, H: Copy> Copy for Id<T, H> {}

impl<T, H: Copy> Clone for Id<T, H> {
    fn clone(&self) -> Id<T, H> {
        *self
    }
}

impl<T, H: PartialEq> PartialEq for Id<T, H> {
    fn eq(&self, other: &Id<T, H>) -> bool {
        self.handle.eq(&other.handle)
    }
}

impl<T, H: Copy + Eq> Eq for Id<T, H> {}

impl<T, H: IntegerHandle> Id<T, H> {
    pub fn new(idx: H) -> Id<T, H> {
        Id {
            handle: idx,
            _marker: PhantomData,
        }
    }

    pub fn as_range(&self) -> IdRange<T, H> {
        IdRange::new(self.handle..self.handle + One::one())
    }
}

impl<T, H: IntegerHandle> Identifier for Id<T, H> {
    type Handle = H;
    type Tag = T;
}

impl<T, H: ToIndex> ToIndex for Id<T, H> {
    fn to_index(&self) -> usize {
        self.handle.to_index()
    }
}

impl<T, H: IntegerHandle> FromIndex for Id<T, H> {
    fn from_index(idx: usize) -> Id<T, H> {
        Id::new(FromIndex::from_index(idx))
    }
}

impl<T, Handle: Hash> Hash for Id<T, Handle> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}
