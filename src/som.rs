extern crate generic_array;
extern crate typenum;
extern crate num;
use std;
use self::generic_array::{GenericArray, ArrayLength};
use self::typenum::*;


type Array2D<T, R, C> = GenericArray<GenericArray<T, C>, R>;

fn convolution<T, R, C>(array: Array2D<T, R, C>) -> Array2D<GenericArray<T, U9>, R, C>
    where T: Copy,
          R: ArrayLength<GenericArray<T, C>> + ArrayLength<GenericArray<GenericArray<T, U9>, C>>,
          C: ArrayLength<T> + ArrayLength<GenericArray<T, U9>>
{
    let mut result: Array2D<GenericArray<T, U9>, R, C>;
    unsafe {
        result = std::mem::uninitialized();
        for row in 1..(R::to_usize() - 1) {
            for col in 1..(C::to_usize() - 1) {
                for r_k in 0..2 {
                    for c_k in 0..2 {
                        result[row][col][c_k + r_k * 3] = array[row + r_k - 1][col + c_k - 1];
                    }
                }
            }
        }
    }
    result
}

fn som_dist<T, R, C>(img: &Array2D<T, R, C>,
                     csomcell: &GenericArray<T, U9>,
                     row: usize,
                     col: usize)
                     -> T
    where T: Copy + num::Float,
          R: ArrayLength<GenericArray<T, C>>,
          C: ArrayLength<T>
{
    let mut result: T = num::zero();
    for r in 0..3 {
        for c in 0..3 {
            let index = c + r * 3;
            let row = r + row;
            let col = c + col;
            result = result + num::pow((img[row][col] - csomcell[index]), 2);
        }
    }
    T::sqrt(result.into()).into()
}
