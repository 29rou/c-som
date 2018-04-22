extern crate generic_array;
extern crate core;
extern crate typenum;
extern crate num;
extern crate num_traits;
extern crate rand;

use shape::{ShapeTrait, Prod};
use std::{default::Default, marker::PhantomData, fmt::Debug, mem::uninitialized};
//use std::ops::{Deref, DerefMut, Add, AddAssign, Sub, SubAssign};
use self::generic_array::{GenericArray, ArrayLength};
use self::typenum::Unsigned;
use self::num::FromPrimitive;
use self::num_traits::NumAssign;
use self::rand::{ThreadRng, distributions::{Normal, IndependentSample}};

#[derive(PartialEq, Hash)]
pub struct MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,
{
    item: GenericArray<Type, Prod<Shape>>,
    _phantom: PhantomData<Shape>,
}

impl<Type, Shape> MathArrayBase<Type, Shape>
    where
        Type: FromPrimitive,
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,

{
    pub fn from_generic_array(input: GenericArray<Type, Prod<Shape>>) -> Self {
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
        let item = GenericArray::from_iter(rnd_iter.take(Shape::Prod::to_usize()));
        Self { item: item, _phantom: PhantomData }
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

impl<Type, Shape> Default for MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,
        GenericArray<Type, Prod<Shape>>: Default,
{
    fn default() -> Self {
        Self { item: <GenericArray<Type, Prod<Shape>> as Default>::default(), _phantom: PhantomData }
    }
}

impl<Type, Shape> Clone for MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,
        GenericArray<Type, Prod<Shape>>: Clone,
{
    #[inline]
    fn clone(&self) -> Self { Self { item: self.item.clone(), _phantom: self._phantom } }
}

impl<Type, Shape> ::std::ops::Deref for MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,
{
    type Target = [Type];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.item.deref()
    }
}

impl<Type, Shape> ::std::ops::DerefMut for MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,

{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut [Type] {
        self.item.deref_mut()
    }
}

impl<Type, Shape> Debug for MathArrayBase<Type, Shape>
    where
        Type: Debug,
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self[..].fmt(f)
    }
}

impl<Type, Shape> ::std::ops::Add for MathArrayBase<Type, Shape>
    where
        Type: ::std::ops::Add<Output=Type> + Clone,
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,

{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = uninitialized();
            for i in 0..Shape::Prod::to_usize() {
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
        Prod<Shape>: ArrayLength<Type>,
{
    type Output = MathArrayBase<Type, Shape>;
    fn add(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = uninitialized();
            for i in 0..Shape::Prod::to_usize() {
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
        Prod<Shape>: ArrayLength<Type>,

{
    fn add_assign(&mut self, other: Self) {
        for i in 0..Shape::Prod::to_usize() {
            self[i] += other[i].clone();
        }
    }
}

impl<Type, Shape> ::std::ops::Sub for MathArrayBase<Type, Shape>
    where
        Type: ::std::ops::Sub<Output=Type> + Clone,
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,

{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        unsafe {
            let mut output: Self = uninitialized();
            for i in 0..Shape::Prod::to_usize() {
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
        Prod<Shape>: ArrayLength<Type>,

{
    type Output = MathArrayBase<Type, Shape>;
    fn sub(self, other: Self) -> Self::Output {
        unsafe {
            let mut output: Self::Output = uninitialized();
            for i in 0..Shape::Prod::to_usize() {
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
        Prod<Shape>: ArrayLength<Type>,

{
    fn sub_assign(&mut self, other: Self) {
        for i in 0..Shape::Prod::to_usize() {
            self[i] -= other[i].clone();
        }
    }
}

impl<'a, Type, Shape> IntoIterator for &'a MathArrayBase<Type, Shape>
    where
        Type: 'a,
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,
{
    type IntoIter = ::std::slice::Iter<'a, Type>;
    type Item = &'a Type;
    fn into_iter(self: Self) -> Self::IntoIter {
        use ::std::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, Type, Shape> IntoIterator for &'a mut MathArrayBase<Type, Shape>
    where
        Type: 'a,
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,
{
    type IntoIter = ::std::slice::IterMut<'a, Type>;
    type Item = &'a mut Type;
    fn into_iter(self: Self) -> Self::IntoIter {
        use ::std::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

impl<Type, Shape> self::core::iter::FromIterator<Type> for MathArrayBase<Type, Shape>
    where
        Shape: ShapeTrait,
        Prod<Shape>: ArrayLength<Type>,
{
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item=Type>,
    {
        use self::core::iter::FromIterator;;
        let item = < GenericArray < Type, Prod < Shape >> as FromIterator::< Type >> ::from_iter(iter);
        Self { item: item, _phantom: PhantomData }
    }
}