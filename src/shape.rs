extern crate typenum;

use std::{ops, marker::PhantomData};
use self::typenum::{U1, Unsigned, NonZero, Add1};

#[macro_export]
macro_rules! shp {
    ($n:ty) => ($crate::shape::Shape<$n,$crate::shape::Nil>);
    ($n:ty, ) =>($crate::shape::Shape<$n,$crate::shape::Nil>);
    ($n:ty, $($tail:ty),+) => ($crate::shape::Shape<$n,shp![$($tail),+]>);
    ($n:ty, $($tail:ty),+,) => ($crate::shape::Shpae<$n,shp![$($tail),+]>);
    () => ("""Macro requires types, e.g. `type Shape = shp![typenum::U1,typenum::U3];`");
}

pub type Shape<A, B> = List<A, B>;
pub type Car<T> = <T as ListTrait>::Car;
pub type Cdr<T> = <T as ListTrait>::Cdr;
pub type Prod<T> = <T as ProdTrait>::Prod;
pub type Len<T> = <T as LenTrait>::Len;

pub struct Nil;

pub struct List<A, B>(PhantomData<(A, B)>);

pub trait ShapeTrait: ListTrait + ProdTrait + LenTrait + ListToVecTrait {}

impl<A: Unsigned + NonZero> ShapeTrait for Shape<A, Nil> {}

impl<A, B> ShapeTrait for Shape<A, B>
    where
        Self: ProdTrait + LenTrait,
        A: Unsigned + NonZero,
        B: ShapeTrait
{}

pub trait ListTrait {
    type Car: Unsigned + NonZero;
    type Cdr;
}

impl<A: Unsigned + NonZero> ListTrait for List<A, Nil>
{
    type Car = A;
    type Cdr = Nil;
}

impl<A, B> ListTrait for List<A, B>
    where
        A: Unsigned + NonZero,
        B: ListTrait,
{
    type Car = A;
    type Cdr = B;
}


pub trait ProdTrait {
    type Prod: Unsigned + NonZero;
}

impl<A: Unsigned + NonZero> ProdTrait for List<A, Nil>
{
    type Prod = A;
}

impl<A, B> ProdTrait for List<A, B>
    where
        A: Unsigned + NonZero + ops::Mul<B::Prod>,
        B: ProdTrait,
        typenum::Prod<A, B::Prod>: Unsigned + NonZero,
{
    type Prod = typenum::Prod<A, B::Prod>;
}

pub trait LenTrait {
    type Len: Unsigned + NonZero;
}

impl<A: Unsigned + NonZero> LenTrait for List<A, Nil>
{
    type Len = U1;
}

impl<A, B> LenTrait for List<A, B>
    where
        A: Unsigned + NonZero,
        B: LenTrait,
        B::Len: ops::Add<typenum::B1>,
        Add1<B::Len>: Unsigned + NonZero,
{
    type Len = Add1<B::Len>;
}

pub trait ListToVecTrait {
    fn list_to_vec() -> Vec<usize>;
}

impl<A> ListToVecTrait for List<A, Nil>
    where
        A: Unsigned + NonZero,
{
    #[inline(always)]
    fn list_to_vec() -> Vec<usize> {
        vec![A::to_usize()]
    }
}

impl<A, B> ListToVecTrait for List<A, B>
    where
        A: Unsigned + NonZero,
        B: ListToVecTrait,
{
    #[inline]
    fn list_to_vec() -> Vec<usize> {
        let mut vec = vec![A::to_usize()];
        vec.append(&mut B::list_to_vec());
        vec
    }
}
