extern crate rand;
extern crate typenum;
extern crate generic_array;
use std;
use imgdata::Image;
use imgdata::ImgData;
use dataset::DataSet;
use dataset::DataSetTrait;

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
 where  T:From<f32>+PartialOrd+SampleRange,K:ArrayLength<T>, 
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
    {
        let minibatchs = (0..train_count)
            .map(|_| take_n_rand(dataset,batch_size))
            .collect::<Vec<Vec<ImgData>>>();
        let img:Image<f32,U8,U8> = dataset.get(1).unwrap().load_img();
    }  
}

fn take_n_rand<'a,T:Clone> (vec:&'a Vec<T>,n:usize)->Vec<T>{
    use self::rand::Rng;
    let rng = &mut rand::thread_rng();
    (1..n)
        .filter_map(|_| rng.choose(&vec))
        .map(|x|x.clone())
        .collect()
}