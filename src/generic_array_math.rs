extern crate generic_array;
extern crate num_traits;
extern crate typenum;
extern crate rand;

use type_compute::*;
use std::marker::PhantomData;
use generic_array::{GenericArray, ArrayLength};
use num_traits::FromPrimitive;

#[derive(Default, PartialEq, Hash)]
pub struct MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,
{
    item: GenericArray<Type, Total<Shape>>,
    _phantom: PhantomData<Shape>,
}

impl<Type, Shape> MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive + Clone,
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,

{
    pub fn from_generic_array(input: GenericArray<Type, Total<Shape>>) -> Self {
        Self {
            item: input,
            _phantom: PhantomData,
        }
    }
    pub fn new_rnd(normal: &rand::distributions::Normal, rng: &mut rand::ThreadRng) -> Self
    {
        Self::from_generic_array(InitNormalRandomTrait::init_random(normal, rng))
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

impl<Type, Shape> ::std::clone::Clone for MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,
        GenericArray<Type, Total<Shape>>: Clone,
{
    fn clone(&self) -> Self { Self { item: self.item.clone(), _phantom: self._phantom } }
}

impl<Type, Shape> ::std::ops::Deref for MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>
{
    type Target = [Type];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.item.deref()
    }
}

impl<Type, Shape> ::std::ops::DerefMut for MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut [Type] {
        self.item.deref_mut()
    }
}

impl<Type,Shape> ::std::fmt::Debug for MathArrayBase<Type,Shape>
    where
        Type: ::std::fmt::Debug,
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result{
        self[..].fmt(f)
    }
}

impl<Type, Shape> ::std::ops::Add for MathArrayBase<Type, Shape>
    where
        Type: ::std::ops::Add<Output=Type> + Clone,
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,

{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = ::std::mem::uninitialized();
            for i in 0..Shape::total_to_usize() {
                output[i] = self[i].clone() + other[i].clone();
            }
            output
        }
    }
}

impl<'a, Type, Shape> ::std::ops::Add for &'a MathArrayBase<Type, Shape>
    where
        Type: ::std::ops::Add<Output=Type> + Clone,
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,
{
    type Output = MathArrayBase<Type, Shape>;
    fn add(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = ::std::mem::uninitialized();
            for i in 0..Shape::total_to_usize() {
                output[i] = self[i].clone() + other[i].clone();
            }
            output
        }
    }
}

impl<Type, Shape> ::std::ops::AddAssign for MathArrayBase<Type, Shape>
    where
        Type: ::std::ops::AddAssign + Clone,
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,

{
    fn add_assign(&mut self, other: Self) {
        for i in 0..Shape::total_to_usize() {
            self[i] += other[i].clone();
        }
    }
}

impl<Type, Shape> ::std::ops::Sub for MathArrayBase<Type, Shape>
    where
        Type: ::std::ops::Sub<Output=Type> + Clone,
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,

{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        unsafe {
            let mut output: Self = ::std::mem::uninitialized();
            for i in 0..Shape::total_to_usize() {
                output[i] = self[i].clone() - other[i].clone();
            }
            output
        }
    }
}

impl<'a, Type, Shape> ::std::ops::Sub for &'a MathArrayBase<Type, Shape>
    where
        Type: ::std::ops::Sub<Output=Type> + Clone,
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,

{
    type Output = MathArrayBase<Type, Shape>;
    fn sub(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = ::std::mem::uninitialized();
            for i in 0..Shape::total_to_usize() {
                output[i] = self[i].clone() - other[i].clone();
            }
            output
        }
    }
}

impl<Type, Shape> ::std::ops::SubAssign for MathArrayBase<Type, Shape>
    where
        Type: ::std::ops::SubAssign + Clone,
        Shape: ShapeTrait,
        Total<Shape>: ArrayLength<Type>,

{
    fn sub_assign(&mut self, other: Self){
            for i in 0..Shape::total_to_usize() {
                self[i] -= other[i].clone();
            }
    }
}

trait InitNormalRandomTrait
{
    fn init_random(normal: &rand::distributions::Normal, rng: &mut rand::ThreadRng) -> Self;
}

impl<T, N> InitNormalRandomTrait for GenericArray<T, N>
    where
        T: FromPrimitive + Clone,
        N: ArrayLength<T>
{
    fn init_random(normal: &rand::distributions::Normal, rng: &mut rand::ThreadRng) -> Self
    {
        use rand::{ThreadRng, distributions::{Normal, IndependentSample}};
        use std::iter::{FromIterator, repeat};
        let normal_rnd = |normal: &Normal, rng: &mut ThreadRng| -> T{
            let rand = normal.ind_sample(rng);
            FromPrimitive::from_f64(rand).unwrap()
        };
        let rnd_iter = repeat(()).map(|()| normal_rnd(&normal, rng));
        GenericArray::from_iter(rnd_iter.take(N::to_usize()))
    }
}