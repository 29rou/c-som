extern crate rand;
extern crate typenum;
extern crate generic_array;
use std;
use imgdata::Image;
use imgdata::ImgData;
use dataset::DataSet;

use self::generic_array::{ArrayLength, GenericArray};
use self::typenum::*;
use self::rand::{Rng, thread_rng};
use self::rand::distributions::range::SampleRange;

type CsomLayer<T, K, S> = GenericArray<GenericArray<T, K>, S>;

trait CsomLayerTrait {
    fn new(rng: &mut rand::ThreadRng) -> Self;
}


impl<T, K, S> CsomLayerTrait for CsomLayer<T, K, S>
    where T: From<f32> + PartialOrd + SampleRange,
          K: ArrayLength<T>,
          S: ArrayLength<GenericArray<T, K>>
{
    fn new(rng: &mut rand::ThreadRng) -> Self {
        let mut csomlayer: CsomLayer<T, K, S>;
        unsafe {
            csomlayer = std::mem::uninitialized();
            for i in &mut csomlayer[..] {
                for j in &mut i[..] {
                    *j = rng.gen_range((0.0).into(), (255.0).into());
                }
            }
        }
        csomlayer
    }
}
#[derive(Debug)]

pub struct CSom<T, K, L, N, M>
    where T: Sized,
          K: ArrayLength<T>,
          L: ArrayLength<CsomLayer<T, K, N>>,
          N: ArrayLength<GenericArray<T, K>>,
          M: ArrayLength<GenericArray<T, K>>
{
    mid_layers: GenericArray<CsomLayer<T, K, N>, L>,
    final_layer: CsomLayer<T, K, M>,
}

impl<T, K, L, N, M> CSom<T, K, L, N, M>
    where T: From<f32> + From<u8> + Copy + PartialOrd + SampleRange + std::marker::Send + std::fmt::Display,
          K: ArrayLength<T>,
          L: ArrayLength<CsomLayer<T, K, N>>,
          N: ArrayLength<GenericArray<T, K>>,
          M: ArrayLength<GenericArray<T, K>>
{
    pub fn new() -> Self {
        let rng = &mut thread_rng();
        let mut csom: CSom<T, K, L, N, M>;
        unsafe {
            use std;
            csom = std::mem::uninitialized();
            for i in &mut csom.mid_layers[..] {
                *i = CsomLayerTrait::new(rng);
            }
            csom.final_layer = CsomLayerTrait::new(rng);
            csom
        }
    }
    pub fn train<'a>(&self, batch_size: usize, train_count: usize, dataset: & DataSet)
    {
        let (tx, rx) = std::sync::mpsc::channel();
        let rng = &mut rand::thread_rng();
        let minibatchs = (0..train_count)
            .map(|_| take_n_rand(dataset, batch_size, rng));
        for (i, minibatch) in minibatchs.enumerate() {
            println!("Train:{}/{}", i, train_count);
            let _ = minibatch
                .into_iter()
                .map( |x| {
                        let tx =tx.clone();
                        let x = x.clone();
                         std::thread::spawn(move || {
                        let x = x.load_img() as Image<T,U32,U32>;
                        let result = convolution(x);
                        tx.send(0)
                    })
                     })
                .map(|_| rx.recv().expect("Thread Error!"))
                .count();
                //.collect::<Vec<Array2D<GenericArray<_, U9>, _, _>>>();
            /*for img in minibatch {
                for i in img.as_ref().into_iter() {
                    for j in i.as_ref().into_iter() {
                        print!("{:^3}", j.get(4).unwrap());
                    }
                    println!("");
                }
                println!("\n\n");
            }*/
        }
    }
}

fn take_n_rand<'a, T>(vec: &'a Vec<T>, n: usize, rng: &mut rand::ThreadRng) -> Vec<&'a T> {
    (0..n)
        .filter_map(|_| rng.choose(vec))
        .collect::<Vec<&'a T>>()
}

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
