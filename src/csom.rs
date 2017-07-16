extern crate ndarray;
extern crate ndarray_rand;
use std;
use imgdata::ImgData;
use dataset::DataSet;
use dataset::DataSetTrait;

type CsomLayer = ndarray::Array2<f32>;

pub struct CSom {
    layer_1: CsomLayer,
    layer_2: CsomLayer,
    layer_3: CsomLayer,
}

impl CSom {
    pub fn new (kernel:usize) ->Self{
        use self::ndarray::Array;;
        use self::ndarray_rand::RandomExt;
        use rand::distributions::Range;
        let r_dist:Range<f32> = Range::new(0.0, 255.0);
        CSom{
            layer_1: Array::random((kernel,kernel),r_dist),
            layer_2: Array::random((kernel,kernel),r_dist),
            layer_3: Array::random((kernel*10,kernel as usize),r_dist)
        }
    }
    
    fn get_conv9(image:CsomLayer) -> CsomLayer{
        let mut vec:Vec<f32> = Vec::new();
        let count = image.windows((3,3)).into_iter().count();
        for kernel in image.windows((3,3)){
            for entry in &kernel{
                vec.push(*entry);
            }
        }
        ndarray::Array::from_shape_vec((count,9),vec).unwrap()
    }

    fn get_distances_layer1(&self,imgdata:&ImgData) -> Vec<Vec<(usize,f32)>>{
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
    fn get_distances_layer2(&self,winners:&Vec<usize>) -> Vec<Vec<(usize,f32)>>{
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
    fn get_distances_layer3(&self,winners:&Vec<usize>) -> Vec<Vec<(usize,f32)>>{
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
    }  
}