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
    /*fn get_conv9<Size:ArrayLength<T>>
    (image:CsomLayer<T,Size>) -> CsomLayer<T,Size>{
        let mut vec:Vec<T> = Vec::new();
        let count = image.windows((3,3)).into_iter().count();
        for kernel in image.windows((3,3)){
            for entry in &kernel{
                vec.push((*entry).into());
            }
        }
        ndarray::Array::from_shape_vec((count,9),vec).unwrap()
    }

    fn get_distances_layer1(&self,imgdata:&ImgData) -> Vec<Vec<(usize,T)>>{
        let mut distances = Vec::new();
        let img = &CSom::get_conv9(imgdata.load_img());
        for kernel in img.genrows(){
            let mut vec = Vec::new();
            for (i,cell) in self.layer_1.genrows().into_iter().enumerate(){
                let diff = &cell - &kernel;
                let dist = &diff
                    .map(|x|x.powf(2.0))
                    .scalar_sum()
                    .sqrt();
                vec.push((i,*dist));
            }
            distances.push(vec);
        }
        distances
    }
    fn get_distances_layer2(&self,winners:&Vec<usize>) -> Vec<Vec<(usize,T)>>{
        let mut distances = Vec::new();
        let count = (winners.iter().count() as f32).sqrt() as usize;
        let winners = winners.iter().map(|x| *x as f32).collect();
        let winners = ndarray::Array::from_shape_vec((count,count),winners)
                    .expect("Can't convert!!:layer2");
        let img = &CSom::get_conv9(winners);
        for kernel in img.genrows() {
            let mut vec = Vec::new();
            for (i,cell) in self.layer_2.genrows().into_iter().enumerate(){
                let diff = &cell - &kernel;
                let dist = &diff
                    .map(|x|x.powf(2.0))
                    .scalar_sum()
                    .sqrt();
                vec.push((i,*dist));
            }
            distances.push(vec);
        }
        distances
    }
    fn get_distances_layer3(&self,winners:&Vec<usize>) -> Vec<Vec<(usize,T)>>{
        let mut distances = Vec::new();
        let count = (winners.iter().count() as f32).sqrt() as usize;
        let winners = winners.iter().map(|x| *x as f32).collect();
        let winners = ndarray::Array::from_shape_vec((count,count),winners)
                    .expect("Can't convert!!:layer3");
        let img = &CSom::get_conv9(winners);
        for kernel in img.genrows() {
            let mut vec = Vec::new();
            for (i,cell) in self.layer_3.genrows().into_iter().enumerate(){
                let diff = &cell - &kernel;
                let dist = &diff
                    .map(|x|x.powf(2.0))
                    .scalar_sum()
                    .sqrt();
                vec.push((i,*dist));
            }
            distances.push(vec);
        }
        distances
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
    */
    pub fn train<R,C> (&self, batch_size:usize,train_count: usize, dataset :&DataSet<T,R,C>)
    where   T:From<u8>+Clone,
            R:ArrayLength<GenericArray<T,C>>+Clone,
            C:ArrayLength<T>+Clone
    {
        let minibatchs = std::iter::repeat(())
            .map(|_| take_n_rand(dataset,batch_size))
            .take(train_count);
        /*for (i,minibatch) in minibatchs.enumerate(){
            let mut vec = Vec::new();
            for entry in minibatch{
                let winl1 = CSom::get_winners(self.get_distances_layer1(&entry));
                let winl2 = CSom::get_winners(self.get_distances_layer2(&winl1));
                let winl3 = CSom::get_winners(self.get_distances_layer3(&winl2));
                vec.push((winl1,winl2,winl3));
            }
        }*/
    }  
}
fn take_n_rand<'a,T:Clone> (vec:&'a Vec<T>,n:usize)->Vec<T>
where 
{
    use self::rand::Rng;
    let rng = &mut rand::thread_rng();
    (1..n)
        .filter_map(|_| rng.choose(&vec))
        .map(|x|x.clone())
        .collect()
}