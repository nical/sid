extern crate num_traits;

use std::cmp;
use std::ops::{Add, Sub};
use num_traits::{Zero, One};

mod id;
mod id_range;
mod id_vector;
mod id_list;

pub use id_vector::{IdSlice, MutIdSlice, IdVec};
pub use id_list::{IdFreeList, NullId, NoneAsNullId};
pub use id::Id;
pub use id_range::{IdRange, ReverseIdRange};

pub trait FromUsize {
    fn from_usize(idx: usize) -> Self;
}

pub trait ToUsize {
    fn to_usize(&self) -> usize;
}

pub trait IntegerHandle
    : Copy
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + cmp::Ord
    + PartialEq
    + PartialOrd
    + FromUsize
    + ToUsize
    + Zero
    + One {
}

pub trait Identifier: Copy + FromUsize + ToUsize + PartialEq {
    type Handle: IntegerHandle;
    type Tag;
}

impl Identifier for u8 {
    type Handle = u8;
    type Tag = ();
}
impl Identifier for u16 {
    type Handle = u16;
    type Tag = ();
}
impl Identifier for u32 {
    type Handle = u32;
    type Tag = ();
}
impl Identifier for u64 {
    type Handle = u64;
    type Tag = ();
}
impl Identifier for i8 {
    type Handle = i8;
    type Tag = ();
}
impl Identifier for i16 {
    type Handle = i16;
    type Tag = ();
}
impl Identifier for i32 {
    type Handle = i32;
    type Tag = ();
}
impl Identifier for i64 {
    type Handle = i64;
    type Tag = ();
}

impl ToUsize for u8 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
impl ToUsize for u16 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
impl ToUsize for u32 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
impl ToUsize for u64 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
impl ToUsize for usize {
    #[inline]
    fn to_usize(&self) -> usize {
        *self
    }
}
impl ToUsize for i8 {
    #[inline]
    fn to_usize(&self) -> usize {
        debug_assert!(*self >= 0);
        *self as usize
    }
}
impl ToUsize for i16 {
    #[inline]
    fn to_usize(&self) -> usize {
        debug_assert!(*self >= 0);
        *self as usize
    }
}
impl ToUsize for i32 {
    #[inline]
    fn to_usize(&self) -> usize {
        debug_assert!(*self >= 0);
        *self as usize
    }
}
impl ToUsize for i64 {
    #[inline]
    fn to_usize(&self) -> usize {
        debug_assert!(*self >= 0);
        *self as usize
    }
}
impl ToUsize for isize {
    #[inline]
    fn to_usize(&self) -> usize {
        debug_assert!(*self >= 0);
        *self as usize
    }
}

impl FromUsize for u8 {
    #[inline]
    fn from_usize(idx: usize) -> u8 {
        idx as u8
    }
}
impl FromUsize for u16 {
    #[inline]
    fn from_usize(idx: usize) -> u16 {
        idx as u16
    }
}
impl FromUsize for u32 {
    #[inline]
    fn from_usize(idx: usize) -> u32 {
        idx as u32
    }
}
impl FromUsize for u64 {
    #[inline]
    fn from_usize(idx: usize) -> u64 {
        idx as u64
    }
}
impl FromUsize for usize {
    #[inline]
    fn from_usize(idx: usize) -> usize {
        idx
    }
}
impl FromUsize for i8 {
    #[inline]
    fn from_usize(idx: usize) -> i8 {
        idx as i8
    }
}
impl FromUsize for i16 {
    #[inline]
    fn from_usize(idx: usize) -> i16 {
        idx as i16
    }
}
impl FromUsize for i32 {
    #[inline]
    fn from_usize(idx: usize) -> i32 {
        idx as i32
    }
}
impl FromUsize for i64 {
    #[inline]
    fn from_usize(idx: usize) -> i64 {
        idx as i64
    }
}
impl FromUsize for isize {
    #[inline]
    fn from_usize(idx: usize) -> isize {
        idx as isize
    }
}

impl IntegerHandle for u8 {}
impl IntegerHandle for u16 {}
impl IntegerHandle for u32 {}
impl IntegerHandle for u64 {}
impl IntegerHandle for usize {}
impl IntegerHandle for i8 {}
impl IntegerHandle for i16 {}
impl IntegerHandle for i32 {}
impl IntegerHandle for i64 {}
impl IntegerHandle for isize {}


/*
// TODO: remove it or implement traits manually

pub trait Generation {
    fn get_gen(&self) -> u32;
}

#[derive(Copy, Clone)]
pub struct GenId<T, H: Copy, G> {
    pub id: Id<T, H>,
    pub gen: G,
}

impl<T, H: Copy + std::fmt::Display, G: std::fmt::Display> std::fmt::Debug for GenId<T, H, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GenId#{}({})", self.id.handle, self.gen)
    }
}

impl<T, H: IntegerHandle, G: PartialEq> PartialEq for GenId<T, H, G> {
    fn eq(&self, other: &GenId<T, H, G>) -> bool {
        self.id == other.id && self.gen == other.gen
    }
}

impl<T, H: IntegerHandle, G> ToUsize for GenId<T, H, G> {
    fn to_usize(&self) -> usize {
        self.id.to_usize()
    }
}

impl Generation for u8 {
    fn get_gen(&self) -> u32 {
        *self as u32
    }
}
impl Generation for u16 {
    fn get_gen(&self) -> u32 {
        *self as u32
    }
}
impl Generation for u32 {
    fn get_gen(&self) -> u32 {
        *self as u32
    }
}
impl Generation for u64 {
    fn get_gen(&self) -> u32 {
        *self as u32
    }
}

impl<T, H: Copy, G: Generation> Generation for GenId<T, H, G> {
    fn get_gen(&self) -> u32 {
        self.gen.get_gen()
    }
}
*/


#[test]
fn test_copy_id() {
    #[derive(Debug)]
    struct Foo;

    // check that Id is Copy
    let a: Id<Foo, u32> = Id::new(0);
    let b = a;
    let c = a;
    assert_eq!(b, c);

    // check that IdRange is Copy
    let r1: IdRange<Foo, u32> = IdRange::new(0..10);

    let r2 = r1;
    let r3 = r1;
    assert_eq!(r2, r3);
}

#[test]
fn test_reverese_id_range() {
    use std::iter::FromIterator;
    fn range(first: u16, count: u16) -> IdRange<u16, u16> {
        IdRange::new(first..(first + count))
    }
    let v: Vec<Id<u16, u16>> = Vec::from_iter(ReverseIdRange::new(range(1, 5)));
    assert_eq!(
        v,
        vec![Id::new(5), Id::new(4), Id::new(3), Id::new(2), Id::new(1)]
    );
}
