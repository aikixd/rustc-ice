
use core::marker::PhantomData;


#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct U256Repr(pub(crate) [u64; 4]);

pub trait Endiannes { }

#[derive(Debug, Clone)]
pub struct LE {}
impl Endiannes for LE {}

// WARNING: If representation changes to wariable size, all unsafe code for `IntX` needs reviewing.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct IntX<const N: usize, E: Endiannes>
{
    pub(crate) repr: U256Repr,
    phantom: PhantomData<[E; N]>,
}

impl<const N: usize, E: Endiannes> IntX<N, E> 
{
    pub fn new() -> Self {
        loop {}
    }
}

impl<const N: usize> IntX<N, LE>
    where Assert<{ size_bound(N) }>: IsTrue
{
    pub fn from_usize(_: usize) -> Self {
        loop {}
    }
}

// Utils
pub struct Assert<const B: bool> {}
pub trait IsTrue: 'static {}
impl IsTrue for Assert<true> {}

pub const fn size_bound(n: usize) -> bool { n > 0 && n <= 32 }

