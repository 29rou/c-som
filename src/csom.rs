extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;
extern crate generic_array;
use std;
use imgdata::ImgData;
use dataset::DataSet;
use dataset::DataSetTrait;

use self::generic_array::{ArrayLength,GenericArray};
use self::rand::{Rng,thread_rng};
use self::rand::distributions::range::SampleRange;

type CsomLayer<T,D,S> 
    = GenericArray<GenericArray<T,D>,S>;

trait CsomLayerTrait{
    fn new(rng:&mut rand::ThreadRng)->Self;
}

impl<T,D,S> CsomLayerTrait for CsomLayer<T,D,S>
where T:From<f32>+PartialOrd+SampleRange,
      D:ArrayLength<T>,
      S:ArrayLength<GenericArray<T,D>>
 {
    fn new(rng:&mut rand::ThreadRng)->Self{
        let mut init_inner = |x:&mut GenericArray<T,D>|{
            for j in x.as_mut(){
                *j =  rng.gen_range((0.0).into(),(255.0).into());
            }
        };
        unsafe{
            let mut csomlayer:CsomLayer<T,D,S> = std::mem::uninitialized();
            for i in csomlayer.as_mut(){
                init_inner(i);
            }
            csomlayer
        }
    }
}

pub struct CSom<T,D,N,M> 
where T:Sized,
      D:ArrayLength<T>, 
      N:ArrayLength<GenericArray<T,D>>,
      M:ArrayLength<GenericArray<T,D>>
{
    pub layer_1: CsomLayer<T,D,N>,
    layer_2: CsomLayer<T,D,N>,
    layer_3: CsomLayer<T,D,M>,
}


impl <T:From<f32>+PartialOrd+SampleRange,D:ArrayLength<T>, N:ArrayLength<GenericArray<T,D>>,M:ArrayLength<GenericArray<T,D>>>
 CSom <T,D,N,M> {
    pub fn new () ->Self{
        let rng = &mut thread_rng();
        CSom{
           layer_1: CsomLayerTrait::new(rng),
           layer_2: CsomLayerTrait::new(rng),
           layer_3: CsomLayerTrait::new(rng) 
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
        let img = &CSom::get_conv9(imgdata.load_img(32));
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

    pub fn train (&self, batch_size:usize,train_count: usize, dataset :&DataSet){
        let minibatchs = std::iter::repeat(())
            .map(|_| dataset.take_n_rand(batch_size))
            .take(train_count);
        for (i,minibatch) in minibatchs.enumerate(){
            let mut vec = Vec::new();
            for entry in minibatch{
                let winl1 = CSom::get_winners(self.get_distances_layer1(&entry));
                let winl2 = CSom::get_winners(self.get_distances_layer2(&winl1));
                let winl3 = CSom::get_winners(self.get_distances_layer3(&winl2));
                vec.push((winl1,winl2,winl3));
            }
        }
    }  */
}