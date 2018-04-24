extern crate typenum;

use self::typenum::{Add1, NonZero, U1, Unsigned};
use std::{marker::PhantomData, ops};




#[macro_export]
macro_rules! shp {
    ($n:ty) => ($crate::shape::Shape<$n,$crate::shape::Nil>);
    ($n:ty, ) =>($crate::shape::Shape<$n,$crate::shape::Nil>);
    ($n:ty, $($tail:ty),+) => ($crate::shape::Shape<$n,shp![$($tail),+]>);
    ($n:ty, $($tail:ty),+,) => ($crate::shape::Shape<$n,shp![$($tail),+]>);
    () => ("""Macro requires types, e.g. `type Shape = shp![typenum::U1,typenum::U3];`");
}

#[test]
fn shp_macro_test() {
    use std::any::TypeId;
    use self::typenum::{U1, U2};
    {
        type Type1 = shp![U1];
        type Type2 = self::List<U1, Nil>;
        assert_eq!(TypeId::of::<Type1>(), TypeId::of::<Type2>());
    }
    {
        type Type1 = shp![U1, ];
        type Type2 = List<U1, Nil>;
        assert_eq!(TypeId::of::<Type1>(), TypeId::of::<Type2>());
    }
    {
        type Type1 = shp![U1, U2];
        type Type2 = List<U1, List<U2, Nil>>;
        assert_eq!(TypeId::of::<Type1>(), TypeId::of::<Type2>());
    }
    {
        type Type1 = shp![U1, U2, ];
        type Type2 = List<U1, List<U2, Nil>>;
        assert_eq!(TypeId::of::<Type1>(), TypeId::of::<Type2>());
    }
}


pub type Shape<A, B> = List<A, B>;
pub type Car<T> = <T as ListTrait>::Car;
pub type Cdr<T> = <T as ListTrait>::Cdr;
pub type Prod<T> = <T as ProdTrait>::Prod;
pub type Len<T> = <T as LenTrait>::Len;


pub struct Nil;

pub struct List<A: Unsigned + NonZero, B>(PhantomData<(A, B)>);


pub trait ShapeTrait: ListTrait + ProdTrait + LenTrait + ListToVecTrait {}

impl<A> ShapeTrait for Shape<A, Nil>
    where
        Self: ListTrait + ProdTrait + LenTrait + ListToVecTrait,
        A: Unsigned + NonZero,
{}

impl<A, B> ShapeTrait for Shape<A, B>
    where
        Self: ListTrait + ProdTrait + LenTrait + ListToVecTrait,
        A: Unsigned + NonZero,
        B: ShapeTrait
{}


pub trait ListTrait {
    type Car: Unsigned + NonZero;
    type Cdr;
    //const Car:usize;
}

impl<A: Unsigned + NonZero> ListTrait for List<A, Nil>
{
    type Car = A;
    type Cdr = Nil;
    //const Car:usize = A::to_usize();
}

impl<A, B> ListTrait for List<A, B>
    where
        A: Unsigned + NonZero,
        B: ListTrait,
{
    type Car = A;
    type Cdr = B;
    //const Car:usize = A::to_usize();
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
