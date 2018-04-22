use std::{ops, marker::PhantomData};

pub struct Zero;

pub struct Succ<N>(PhantomData<N>);

pub trait Nat {}

impl Nat for Zero {}

impl<N: Nat> Nat for Succ<N> {}

pub trait ToUsize: Nat {
    fn to_usize() -> usize;
}

impl ToUsize for Zero {
    #[inline(always)]
    fn to_usize() -> usize {
        0
    }
}

impl<N: Nat + ToUsize> ToUsize for Succ<N> {
    #[inline]
    fn to_usize() -> usize {
        N::to_usize() + 1
    }
}

pub trait Add<A: Nat>: Nat {
    type Result: Nat;
}

impl<A: Nat> Add<A> for Zero {
    type Result = A;
}

impl<A: Nat, B: Nat> Add<A> for Succ<B>
    where B: Add<Succ<A>>,
{
    type Result = <B as Add<Succ<A>>>::Result;
}

pub trait Mul<A: Nat>: Nat {
    type Result: Nat;
}

impl<A: Nat> Mul<A> for Zero {
    type Result = Zero;
}

impl<A: Nat, B: Nat> Mul<A> for Succ<B>
    where
        A: Add<<B as Mul<A>>::Result>,
        B: Mul<A>,
{
    type Result = <A as Add<<B as Mul<A>>::Result>>::Result;
}

