extern crate typenum;
extern crate generic_array;

use std::{ops::Mul, ops::Add, marker::PhantomData,ptr::write};
use self::typenum::{U1, Prod, Unsigned, NonZero, Add1};
use self::generic_array::{GenericArray, ArrayLength};

#[macro_export]
macro_rules! shp {
    ($n:ty) => ($crate::type_compute::List<$n,$crate::type_compute::Nil>);
    ($n:ty, ) =>($crate::type_compute::List<$n,$crate::type_compute::Nil>);
    ($n:ty, $($tail:ty),+) => ($crate::type_compute::List<$n,shp![$($tail),+]>);
    ($n:ty, $($tail:ty),+,) => ($crate::type_compute::Shpae<$n,shp![$($tail),+]>);
    () => ("""Macro requires types, e.g. `type List = shp![typenum::U1,typenum::U3];`");
}

pub type Car<T> = <T as ListTrait>::Car;
pub type Cdr<T> = <T as ListTrait>::Cdr;
pub type Total<T> = <T as ListTrait>::Total;

pub struct Nil;

pub struct List<A,B>(PhantomData<(A, B)>);

pub trait ShapeTrait: ListTrait{}

impl<A:Unsigned + NonZero> ShapeTrait for List<A,Nil>{}

impl<A:Unsigned + NonZero,B:ShapeTrait> ShapeTrait for List<A,B>{}


pub trait ListTrait: ForListToArrayTrait {
    type Car: Unsigned;
    type Cdr;
    type Total: Unsigned + NonZero;
    type Dim: Unsigned + NonZero + ArrayLength<usize>;
    #[inline(always)]
    fn car_to_usize() -> usize {
        Self::Car::to_usize()
    }
    #[inline(always)]
    fn total_to_usize() -> usize {
        Self::Total::to_usize()
    }
    #[inline(always)]
    fn dim_to_usize() -> usize {
        Self::Dim::to_usize()
    }
    #[inline]
    fn list_to_array() -> GenericArray<usize, Self::Dim> {
        let mut array = <GenericArray<usize, Self::Dim> as Default>::default();
        unsafe { Self::for_list_to_array(array.as_mut_ptr()) };
        array
    }
}

impl<A: Unsigned + NonZero> ListTrait for List<A, Nil>
{
    type Car = A;
    type Cdr = Nil;
    type Total = A;
    type Dim = U1;
}

impl<A, B> ListTrait for List<A, B>
    where
        A: Unsigned + NonZero + Mul<B::Total>,
        B: ListTrait,
        B::Total: Unsigned + NonZero,
        B::Dim: NonZero + Add<typenum::B1>,
        Prod<A, B::Total>: Unsigned + NonZero,
        Add1<B::Dim>: Unsigned + NonZero + generic_array::ArrayLength<usize>,
{
    type Car = A;
    type Cdr = B;
    type Total = Prod<A, B::Total>;
    type Dim = Add1<B::Dim>;
}

pub trait ProdTrait{
    type Prod: Unsigned;
}

impl<A: typenum::Integer> ProdTrait for List<A, Nil>
{
    type Prod = A;
}

impl<A, B> ProdTrait for List<A, B>
    where
        A: typenum::Integer + Mul<B::Prod>,
        B: ProdTrait,
        B::Prod: typenum::Integer,
        Prod<A, B::Prod>: typenum::Integer,
{
    type Prod = Prod<A, B::Prod>;
}

pub trait LenTrait{
    type Len: Unsigned;
}

impl<A> LenTrait for List<A, Nil>
{
    type Len = U1;
}

impl<A, B> LenTrait for List<A, B>
    where
        B: LenTrait,
        B::Len: NonZero + Add<typenum::B1>,
        Add1<B::Len>: Unsigned,
{
    type Len = Add1<B::Len>;
}

pub trait ForListToArrayTrait {
    unsafe fn for_list_to_array(*mut usize);
}

impl<A:typenum::Integer> ForListToArrayTrait for List<A, Nil>
{
    #[inline(always)]
    unsafe fn for_list_to_array(ptr: *mut usize) {
        write(ptr, A::to_usize());
    }
}

impl<A, B> ForListToArrayTrait for List<A, B>
    where
        A: typenum::Integer,
        B: ForListToArrayTrait,
{
    #[inline(always)]
    unsafe fn for_list_to_array(ptr: *mut usize) {
        write(ptr, A::to_usize());
        B::for_list_to_array(ptr.offset(1));
    }
}
