extern crate rand;
extern crate typenum;
extern crate generic_array;
use std;
use imgdata::ImgData;
use dataset::DataSet;
use dataset::DataSetTrait;

use self::generic_array::{ArrayLength,GenericArray};
use self::rand::{Rng,thread_rng};
use self::rand::distributions::range::SampleRange;

type Kernel<T> 
    = GenericArray<T,generic_array::typenum::U9>;
type Array2D<T,R,C>
    = GenericArray<GenericArray<T,C>,R>;

type CsomLayer<T,K,S> 
    = GenericArray<GenericArray<T,K>,S>;

trait CsomLayerTrait{
    fn new(rng:&mut rand::ThreadRng)->Self;
}

trait GetConvTrait<T,R,C>
where   T:Clone+From<f32>,
        R:ArrayLength<GenericArray<T,C>>
            +ArrayLength<GenericArray<Kernel<T>,C>>,
        C:ArrayLength<T>+ArrayLength<Kernel<T>>
{
    fn get_conv9(image:&Array2D<T,R,C>) -> Array2D<Kernel<T>,R,C>;
}
impl <T,R,C>GetConvTrait<T,R,C> for Array2D<T,R,C>
where   T:Clone+From<f32>,
        R:ArrayLength<GenericArray<T,C>>
            +ArrayLength<GenericArray<Kernel<T>,C>>,
        C:ArrayLength<T>+ArrayLength<Kernel<T>>
{
    fn get_conv9(image:&Array2D<T,R,C>) -> Array2D<Kernel<T>,R,C>
    {
        let mut result:GenericArray<GenericArray<Kernel<T>,C>,R>;
        unsafe{
            result = std::mem::uninitialized();
            for r in 1..(R::to_usize() - 1){
                for c in 1..(C::to_usize() - 1){
                    for r_k in 0..2{
                        for c_k in 0..2{
                            let r_k = r_k -1;
                            let c_k = c_k -1;
                            result[r][c][c_k+r_k*3] = 
                                image[r+r_k][c+c_k].clone();
                        }
                    }
                }
            }
        }
        result
    }
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
    fn get_winners (distances:Vec<Vec<(usize,f32)>>) -> Vec<usize>{
        let init = (<usize>::max_value(), (0.0/0.0));
        let winers = distances.iter()
            .map(|x| {
                x.iter().fold(&init,|m,v| {
                    if m.1 > v.1{
                        m
                    }else{
                        v
                    }
                }).0
            });
        winers.collect()
    }
    pub fn train<R,C> (
        &self, 
        batch_size:usize, train_count:usize, 
        dataset:&DataSet<T,R,C>
        )
    where   T:From<u8>+Clone+From<f32>,
            R:ArrayLength<GenericArray<T,C>>
                +ArrayLength<GenericArray<Kernel<T>,C>>,
            C:ArrayLength<T>+ArrayLength<Kernel<T>>,
            DataSet<T,R,C>:DataSetTrait<T,R,C>,
            Array2D<T,R,C>:GetConvTrait<T,R,C>
    {
        let minibatchs = (0..train_count)
            .map(|_| dataset.get_minibatch(batch_size));
        for minibatch in minibatchs{
            let img:&Array2D<T,R,C>
                = &minibatch.get(1).unwrap().image;
            //let conv:Array2D<Kernel<T>,R,C> 
              //  = GetConvTrait::get_conv9(img);      
            }
    }  
}