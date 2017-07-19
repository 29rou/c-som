extern crate rand;
extern crate typenum;
extern crate generic_array;
use std;
use imgdata::Image;
use imgdata::ImgData;
use dataset::DataSet;

use self::generic_array::{ArrayLength,GenericArray};
use self::typenum::*;
use self::rand::{Rng,thread_rng};
use self::rand::distributions::range::SampleRange;

type CsomLayer<T,K,S> 
    = GenericArray<GenericArray<T,K>,S>;

trait CsomLayerTrait{
    fn new(rng:&mut rand::ThreadRng)->Self;
}


impl<T,K,S> CsomLayerTrait for CsomLayer<T,K,S> 
 where T:From<f32>+PartialOrd+SampleRange,
       K:ArrayLength<T>,
       S:ArrayLength<GenericArray<T,K>>
 {
    fn new(rng:&mut rand::ThreadRng)->Self{
        let mut csomlayer:CsomLayer<T,K,S>;
        unsafe{
            csomlayer = std::mem::uninitialized();
            for i in csomlayer.as_mut(){
                for j in i.as_mut(){
                    *j =  rng.gen_range((0.0).into(),(255.0).into());
                }
            }
        }
        csomlayer
    }
}

pub struct CSom<T,K,L,N,M> 
where T:Sized,
      K:ArrayLength<T>, 
      L:ArrayLength<CsomLayer<T,K,N>>,
      N:ArrayLength<GenericArray<T,K>>,
      M:ArrayLength<GenericArray<T,K>>
{
    pub mid_layers: GenericArray<CsomLayer<T,K,N>,L>,
    final_layer: CsomLayer<T,K,M>,
}

impl <T,K,L,N,M> CSom <T,K,L,N,M>
 where  T:From<f32>+From<u8>+Clone+PartialOrd+SampleRange,
        K:ArrayLength<T>, 
        L:ArrayLength<CsomLayer<T,K,N>>,
        N:ArrayLength<GenericArray<T,K>>,
        M:ArrayLength<GenericArray<T,K>>
 {
    pub fn new () ->Self{
        let rng = &mut thread_rng();
        let mut csom:CSom<T,K,L,N,M>;
        unsafe{
            use std;
            csom = std::mem::uninitialized();
            for i in csom.mid_layers.as_mut(){
                *i = CsomLayerTrait::new(rng);
            }
            csom.final_layer = CsomLayerTrait::new(rng);
            csom
        }
    }
    pub fn train (
        &self, 
        batch_size:usize,train_count: usize, 
        dataset :&DataSet
    )
    where   T:std::marker::Send+'static+std::fmt::Display,
            DataSet:std::iter::FromIterator<ImgData>
    {
        let (tx, rx) = std::sync::mpsc::channel();
        let rng = &mut rand::thread_rng();
        let minibatchs = (0..train_count)
            .map(|_|take_n_rand(dataset,batch_size,rng));
        for minibatch in minibatchs{
            let t = minibatch
                .into_iter()
                //.map(|x|->Image<T,U32,U32>{x.load_img()})
                .map(|x| {
                    let tx = tx.clone();
                    std::thread::spawn(move ||{
                        let x:Image<T,U32,U32> = x.load_img();
                        tx.send(convolution(x))
                })})
                .map(|_| rx.recv().expect("Thread Error!"))
                .collect::<Vec<Array2D<GenericArray<T,U9>,U32,U32>>>();
            for img in t{
                for i in img.as_ref().into_iter(){
                    for j in i.as_ref().into_iter(){
                        print!("{:^3}",j.get(4).unwrap());
                    }
                    println!("");
                }
                println!("\n\n");
            }
        }
    }  
}

fn take_n_rand<T:Clone> (vec:&Vec<T>,n:usize,rng:&mut rand::ThreadRng )->Vec<T>
{
    //use self::rand::Rng;
    //let rng = &mut rand::thread_rng();
    (1..n)
        .filter_map(|_| rng.choose(vec))
        .map(|x|x.clone())
        .collect::<Vec<T>>()
}



type Array2D <T,R,C> = GenericArray<GenericArray<T,C>,R>;
fn convolution<T,R,C> (array:Array2D<T,R,C>) 
    -> Array2D<GenericArray<T,U9>,R,C>
where   T:Clone,
        R:ArrayLength<GenericArray<T,C>>
            +ArrayLength<GenericArray<GenericArray<T,U9>,C>>,
        C:ArrayLength<T>
            +ArrayLength<GenericArray<T,U9>>
{
    let mut result:Array2D<GenericArray<T,U9>,R,C>;
    unsafe{
        result = std::mem::uninitialized();
        for row in 1..(R::to_usize() - 1){
            for col in 1..(C::to_usize() -1){
                for r_k in 0..2{
                    for c_k in 0..2{
                        result[row][col][c_k + r_k * 3] =
                            array[row + r_k - 1][col + c_k -1]
                                .clone();
                    }
                }
            }
        }
    }
    result
}