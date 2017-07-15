extern crate image;
extern crate rand;
#[macro_use(array)]
extern crate ndarray;
extern crate ndarray_rand;

type DataSet  = Vec<ImgData>;

trait DataSetTrait {
    fn new(p:&str)->Self;
    fn take_n_rand(&self, n:usize)->MiniBatch;
}

type MiniBatch<'a>  = Vec<&'a ImgData>;

struct CSom {
    layer_1: ndarray::Array2<f32>,
    layer_2: ndarray::Array2<f32>,
    layer_3: ndarray::Array2<f32>,
}

struct ImgData  {
    path: std::path::PathBuf,
    class: String,
}

impl  ImgData{
    fn new (path:std::path::PathBuf) -> Self{
        let c = path.parent().unwrap()
            .iter().last().unwrap()
            .to_str().unwrap().to_string();
         ImgData{path: path, class: c}
    }
    fn load_img(&self,size:usize) -> ndarray::Array2<f32>{
        use ndarray::Array;
        let img = image::imageops::resize(
            &image::open(&self.path.as_path())
            .expect("Can't read image!!")
            .to_luma(),
            size as u32,
            size as u32,
            image::FilterType::Lanczos3
        );
        let img:Vec<f32> = img.iter().map(|x| *x as f32).collect();
        let img_array = Array::from_shape_vec(
            (size,size),
            img
        ).unwrap();
        img_array
    }
}

impl DataSetTrait for DataSet{
    fn new (p: &str) ->  Self{
        extern crate walkdir;
        walkdir::WalkDir::new(p)
            .into_iter()
            .map(|x| x.unwrap().path().to_path_buf())
            .filter(|x| x.is_file())
            .map(|x| { ImgData::new(x)})
            .collect::<DataSet>()
    } 
    fn take_n_rand  (&self,n: usize)->MiniBatch{
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let mut v :MiniBatch = Vec::new();
        v.reserve(n as usize);
        for _ in 0..n{
            let r= rng.choose(&self).unwrap();
            v.push(r);
        }
        v
    }
}

impl CSom {
    fn new (kernel:usize) ->Self{
        use ndarray::Array;;
        use ndarray_rand::RandomExt;
        use rand::distributions::Range;
        let r_dist:Range<f32> = Range::new(0.0, 255.0);
        CSom{
            layer_1: Array::random((kernel,kernel),r_dist),
            layer_2: Array::random((kernel,kernel),r_dist),
            layer_3: Array::random((kernel*10,kernel as usize),r_dist)
        }
    }
    
    fn get_conv9(image:ndarray::Array2<f32>) -> ndarray::Array2<f32>{
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

    fn train (&self, batch_size:usize,train_count: usize, dataset :&DataSet){
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

fn main() {
    let path = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset:DataSet = DataSetTrait::new(path);
    let csom:CSom = CSom::new(9);
    /*let img = &dataset.get(4).unwrap().load_img(32);
    let a = img.subview(ndarray::Axis(0),0);
    let b = img.subview(ndarray::Axis(0),1);
    let c = (&a - &b).map(|x|(x.pow(2)as f32)).scalar_sum().sqrt();
    println!("{}\n\n{}\n\n{}\n\n{}",img,a,b,c);
    let w =img.windows((3,3));
    let mut w = w.into_iter();
    let w = w.next().unwrap();
    println!("{}",w);*/
    csom.train(10,100,&dataset);
}
