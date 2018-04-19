extern crate generic_array;
extern crate itertools;
extern crate num;
extern crate num_traits;
extern crate rand;
extern crate typenum;

use ::std;
use self::generic_array::{GenericArray, ArrayLength};
use self::typenum::{Prod, Quot};
use self::std::ops::{Mul, Div};
use std::marker::PhantomData;




#[derive(Clone, Debug)]
pub struct MathArray<Dim, D, Total: ArrayLength<Type>, Type> {
    item: GenericArray<Type, Total>,
    _phantom: PhantomData<(Dim,D)>,
}

struct Unknown;

type Scalar<Type> = MathArray<typenum::U1, typenum::U1, typenum::U1, Type>;
pub type Array1D<Total, Type> = MathArray<typenum::U1, Total, Total, Type>;
pub type Array2D<D, Total, Type> = MathArray<typenum::U2, D, Total, Type>;
type Array3D<D, Total, Type> = MathArray<typenum::U3, D, Total, Type>;

impl<Dim,D, Total, Type> MathArray<Dim, D, Total, Type>
    where
        Total: ArrayLength<Type>,
        Type: num_traits::FromPrimitive+num_traits::Float,
{
    pub fn new(normal: &rand::distributions::Normal, rng: &mut rand::ThreadRng) -> Self {
        use rand::{ThreadRng, distributions::{Normal, IndependentSample}};
        use std::iter::FromIterator;
        let get_normal_random = |normal: &Normal, rng: &mut ThreadRng| -> Type{
            let rand = normal.ind_sample(rng);
            num_traits::FromPrimitive::from_f64(rand).unwrap()
        };
        let rnd_iter = std::iter::repeat(()).map(|()| get_normal_random(&normal, rng)).take(Total::to_usize());
        MathArray {
            item: GenericArray::from_iter(rnd_iter),
            _phantom:PhantomData,
        }
    }
    fn get_minimum_location(&self)->usize
    {
        let mut iter = self.item.iter();
        let min_fn = |x: &Type, y: &Type| {
            x.partial_cmp(y).unwrap()
        };
        let min = iter.by_ref().min_by(|x, y| min_fn(x, y)).unwrap();
        iter.by_ref().position(|x| x == min).unwrap()
    }
}

impl<D, Total, Type> MathArray<typenum::U2, D, Total, Type>
    where
        D: ArrayLength<Type>,
        Total: ArrayLength<Type>+Div<D>,
        Type: num_traits::FromPrimitive+num_traits::Float,
{
    fn broadcast_sub(&self,input:&MathArray<typenum::U1,D,D,Type>) -> Self
    {
        MathArray {
            item: self::for_matharray::broadcast_sub( &self.item, &input.item),
            _phantom: PhantomData,
        }
    }
    fn norms(&self)-> Array1D<Quot<Total,D>,Type>
    where Quot<Total,D>:ArrayLength<Type>,
    {
        MathArray {
            item: self::for_matharray::norms( &self.item),
            _phantom: PhantomData,
        }
    }
}

impl<D, Total, Type> MathArray<typenum::U3, D, Total, Type>
    where
        D: ArrayLength<Type>,
        Total: ArrayLength<Type>+Div<D>,
        Type: num_traits::FromPrimitive+num_traits::Float,
{
    fn broadcast_sub(&self,input:&MathArray<typenum::U1,D,D,Type>) -> Self
    {
        MathArray {
            item: self::for_matharray::broadcast_sub( &self.item, &input.item),
            _phantom: PhantomData,
        }
    }
    fn norms(&self)-> Array2D<Unknown,Quot<Total,D>,Type>
        where Quot<Total,D>:ArrayLength<Type>,
    {
        MathArray {
            item: self::for_matharray::norms( &self.item),
            _phantom: PhantomData,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Som1d<T, W, H>
    where
        W: Mul<H>,
        Prod<W, H>: ArrayLength<T>+std::clone::Clone+std::fmt::Debug,
{
    width: usize,
    height: usize,
    total: usize,
    cells: Array2D<W, Prod<W,H>, T>,
}

impl<T, W, H> Som1d<T, W, H>
    where
        T: num_traits::FromPrimitive+num_traits::Float,
        H: typenum::Unsigned,
        W: Mul<H> + typenum::Unsigned,
        Prod<W, H>: ArrayLength<T>+std::clone::Clone+std::fmt::Debug,
{
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        use rand::distributions::Normal;
        let normal = Normal::new(0.0, 1.0);
        Som1d {
            width: W::to_usize(),
            height: H::to_usize(),
            total: W::to_usize() * H::to_usize(),
            cells: Array2D::new(&normal,rng),
        }
    }
}

pub trait Som<T,Dim, D>
    where
        T: num::Float,
        D: ArrayLength<T>,
        Quot<Self::Total, D>: ArrayLength<T>,
{
    type Total: ArrayLength<T> + Div<D>;
    fn train(&self, &Array1D<D,T>) -> MathArray<Dim, D, Self::Total,T>;
    fn ref_cells(&self) -> &MathArray<Dim, D, Self::Total,T>;
}


impl<T, W, H> Som<T, typenum::U2,W> for Som1d<T, W, H>
    where
        T: num_traits::Float+num_traits::FromPrimitive,
        H: typenum::Unsigned + ArrayLength<T>,
        W: Mul<H> + ArrayLength<T>,
        Prod<W, H>: ArrayLength<T> + Div<W>+std::clone::Clone+std::fmt::Debug,
        Quot<Prod<W, H>, W>: ArrayLength<T>,
{
    type Total = Prod<W, H>;
    fn train(&self, input: &Array1D<W,T>)
             -> Array2D<W, Self::Total,T>
    {
        let diffs = self.ref_cells().broadcast_sub(input);
        let norms = diffs.norms();
        //let diffs = self.broadcast_sub(input);
        //let norms = self.norms(&diffs);
        diffs
    }
    fn ref_cells(&self) ->  &Array2D<W, Prod<W,H>, T>{
        &(self.cells)
    }
}

/*
#[derive(Clone, Debug)]
struct Som2d<T, D, W, H>
    where H: std::ops::Mul<D>,
          W: std::ops::Mul<<H as std::ops::Mul<D>>::Output>,
          <W as std::ops::Mul<<H as std::ops::Mul<D>>::Output>>::Output: ArrayLength<T>,
{
    depth: usize,
    width: usize,
    height: usize,
    total: usize,
    cells: GenericArray<T, <W as std::ops::Mul<<H as std::ops::Mul<D>>::Output>>::Output>,
}


impl<T, D, W, H> Som<T> for Som2d<T, D, W, H>
    where D: ArrayLength<T>,
          H: std::ops::Mul<D>,
          W: std::ops::Mul<<H as std::ops::Mul<D>>::Output>,
          <W as std::ops::Mul<<H as std::ops::Mul<D>>::Output>>::Output: ArrayLength<T>,
{
    type N = D;
    type M = <W as std::ops::Mul<<H as std::ops::Mul<D>>::Output>>::Output;
    fn train(&self, input: &GenericArray<T, D>) -> GenericArray<T, <W as std::ops::Mul<<H as std::ops::Mul<D>>::Output>>::Output> {
        self.clone().cells
    }
}
*/

mod for_matharray {
    extern crate generic_array;
    extern crate  num_traits;
    extern crate  typenum;

    use ::std;
    use std::ops::Div;
    use self::num_traits::Float;
    use self::generic_array::{GenericArray,ArrayLength};
    use self::typenum::{Unsigned,Quot};

    pub fn broadcast_sub<T, N, M>(cells: &GenericArray<T, N>, input: &GenericArray<T, M>)
                              -> GenericArray<T, N>
        where T: Float,
              N: ArrayLength<T>,
              M: ArrayLength<T>,
    {
        let y_len = N::to_usize() / M::to_usize();
        let x_len = M::to_usize();
        unsafe {
            let mut diffs: GenericArray<T, N> = std::mem::uninitialized();
            for y in 0..y_len {
                for x in 0..x_len {
                    let location = x + y * x_len;
                    diffs[location] = cells[location] - input[x];
                }
            }
            diffs
        }
    }

    pub fn norms<T, N, M>(input: &GenericArray<T, M>) -> GenericArray<T, Quot<M, N>>
        where T: Float,
              N: Unsigned,
              M: ArrayLength<T> + Div<N>,
              Quot<M, N>: ArrayLength<T>,
    {
        let y_len = M::to_usize() / N::to_usize();
        let x_len = N::to_usize();
        let two: T = num_traits::one::<T>() + num_traits::one::<T>();
        unsafe {
            let mut norms: GenericArray<T, Quot<M, N>> = std::mem::uninitialized();
            for y in 0..y_len {
                let mut tmp: T = num_traits::zero();
                for x in 0..x_len {
                    tmp = tmp + input[x + y * x_len].powf(two);
                }
                norms[y] = tmp.sqrt();
            }
            norms
        }
    }
}