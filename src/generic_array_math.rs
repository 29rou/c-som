extern crate generic_array;
extern crate num;
extern crate num_traits;
extern crate rand;

use type_compute::{ListTrait, Total};
use std::{marker::PhantomData, fmt::Debug, mem::uninitialized};
use std::ops::{Deref, DerefMut, Add, AddAssign, Sub, SubAssign};
use self::generic_array::{GenericArray, ArrayLength};
use self::num::FromPrimitive;
use self::num_traits::NumAssign;
use self::rand::{ThreadRng, distributions::{Normal, IndependentSample}};

#[derive(Default, PartialEq, Hash)]
pub struct MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,
        GenericArray<Type, Total<Shape>>: Clone,
{
    item: GenericArray<Type, Total<Shape>>,
    _phantom: PhantomData<Shape>,
}

impl<Type, Shape> MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,

{
    #[inline]
    pub fn from_generic_array(input: GenericArray<Type, Total<Shape>>) -> Self {
        Self {
            item: input,
            _phantom: PhantomData,
        }
    }
    pub fn new_rnd(normal: &Normal, rng: &mut ThreadRng) -> Self
    {
        use std::iter::{FromIterator, repeat};
        let normal_rnd = |normal: &Normal, rng: &mut ThreadRng| -> Type{
            let rand = normal.ind_sample(rng);
            FromPrimitive::from_f64(rand).unwrap()
        };
        let rnd_iter = repeat(()).map(|()| normal_rnd(&normal, rng));
        let item = GenericArray::from_iter(rnd_iter.take(Shape::total_to_usize()));
        Self::from_generic_array(item)
    }
    #[inline]
    pub fn as_slice(&self) -> &[Type] {
        use ::std::ops::Deref;
        self.deref()
    }
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Type] {
        use ::std::ops::DerefMut;
        self.deref_mut()
    }
}

impl<Type, Shape> Clone for MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,
        GenericArray<Type, Total<Shape>>: Clone,
{
    fn clone(&self) -> Self { Self { item: self.item.clone(), _phantom: self._phantom } }
}

impl<Type, Shape> Deref for MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>
{
    type Target = [Type];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.item.deref()
    }
}

impl<Type, Shape> DerefMut for MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut [Type] {
        self.item.deref_mut()
    }
}

impl<Type, Shape> Debug for MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self[..].fmt(f)
    }
}

impl<Type, Shape> Add for MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,

{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = uninitialized();
            for i in 0..Shape::total_to_usize() {
                output[i] = self[i].clone() + other[i].clone();
            }
            output
        }
    }
}

impl<'a, Type, Shape> Add for &'a MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,
{
    type Output = MathArrayBase<Type, Shape>;
    fn add(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = uninitialized();
            for i in 0..Shape::total_to_usize() {
                output[i] = self[i].clone() + other[i].clone();
            }
            output
        }
    }
}

impl<Type, Shape> AddAssign for MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,

{
    fn add_assign(&mut self, other: Self) {
        for i in 0..Shape::total_to_usize() {
            self[i] += other[i].clone();
        }
    }
}

impl<Type, Shape> Sub for MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,

{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        unsafe {
            let mut output: Self = uninitialized();
            for i in 0..Shape::total_to_usize() {
                output[i] = self[i].clone() - other[i].clone();
            }
            output
        }
    }
}

impl<'a, Type, Shape> Sub for &'a MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,

{
    type Output = MathArrayBase<Type, Shape>;
    fn sub(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = uninitialized();
            for i in 0..Shape::total_to_usize() {
                output[i] = self[i].clone() - other[i].clone();
            }
            output
        }
    }
}

impl<Type, Shape> SubAssign for MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + NumAssign + Debug + Clone,
        Shape: ListTrait,
        Total<Shape>: ArrayLength<Type>,

{
    fn sub_assign(&mut self, other: Self) {
        for i in 0..Shape::total_to_usize() {
            self[i] -= other[i].clone();
        }
    }
}
