extern crate typenum;
extern crate generic_array;

use std::{ops::Mul, ops::Add, marker::PhantomData};
use typenum::{U0, U1, Prod, Unsigned, NonZero, Add1};
use generic_array::{GenericArray, ArrayLength};

#[macro_export]
macro_rules! shp {
    ($n:ty) => (Shape<$n,Nil>);
    ($n:ty, ) =>(Shape<$n,Nil>);
    ($n:ty, $($tail:ty),+) => (Shape<$n,shp![$($tail),+]>);
    ($n:ty, $($tail:ty),+,) => (Shpae<$n,shp![$($tail),+]>);
    () => ("""Macro requires types, e.g. `type Shape = shp![typenum::U1,typenum::U3];`");
}
fn test() {
    unimplemented!();
}

pub type ShapeAlias<T: ShapeTrait> = T;
pub type Car<T> = <T as ShapeTrait>::Car;
pub type Cdr<T> = <T as ShapeTrait>::Cdr;
pub type Total<T> = <T as ShapeTrait>::Total;

pub struct Nil;

pub struct Shape<A: Unsigned + NonZero, B>(PhantomData<(A, B)>);

pub trait ShapeTrait {
    type Car: Unsigned;
    type Cdr;
    type Total: Unsigned;
    type Dim: Unsigned + generic_array::ArrayLength<usize>;
    #[inline(always)]
    fn car_to_usize() -> usize {
        use self::Unsigned;
        Self::Car::to_usize()
    }
    #[inline(always)]
    fn total_to_usize() -> usize {
        use self::Unsigned;
        Self::Total::to_usize()
    }
    #[inline(always)]
    fn dim_to_usize() -> usize {
        use self::Unsigned;
        Self::Dim::to_usize()
    }
    #[inline]
    fn shape_to_array() -> GenericArray<usize, Self::Dim> {
        let mut shape = <GenericArray<usize, Self::Dim> as Default>::default();
        unsafe { Self::for_shape_to_array(shape.as_mut_ptr()) };
        shape
    }
    unsafe fn for_shape_to_array(*mut usize);
}

impl<A: Unsigned + NonZero> ShapeTrait for Shape<A, Nil>
{
    type Car = A;
    type Cdr = Nil;
    type Total = A;
    type Dim = U1;
    #[inline(always)]
    unsafe fn for_shape_to_array(ptr: *mut usize) {
        ::std::ptr::write(ptr, Self::car_to_usize());
    }
}

impl<A, B> ShapeTrait for Shape<A, B>
    where
        A: Unsigned + NonZero + Mul<B::Total>,
        B: ShapeTrait,
        B::Total: Unsigned,
        B::Dim: NonZero + Add<typenum::B1>,
        Prod<A, B::Total>: Unsigned,
        Add1<B::Dim>: Unsigned + generic_array::ArrayLength<usize>,
{
    type Car = A;
    type Cdr = B;
    type Total = Prod<A, B::Total>;
    type Dim = Add1<B::Dim>;
    #[inline(always)]
    unsafe fn for_shape_to_array(ptr: *mut usize) {
        ::std::ptr::write(ptr, Self::car_to_usize());
        Self::Cdr::for_shape_to_array(ptr.offset(1));
    }
}

