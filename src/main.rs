extern crate image;
use std::path;
extern crate rand;
use rand::{thread_rng, Rng};
extern crate ndarray;
extern crate ndarray_rand;

type DataSet  = Vec<ImgData>;

trait DataSetTrait {
    fn new(p:&str)->Self;
    fn take_n_rand(&self, n:u32)->MiniBatch;
}

type MiniBatch<'a>  = Vec<&'a ImgData>;

struct CSom {
    layer_1: ndarray::Array2<f32>,
    layer_2: ndarray::Array2<f32>,
    layer_3: ndarray::Array3<f32>,
}

struct ImgData  {
    path: path::PathBuf,
    class: String
}

impl  ImgData{
    fn new (path:  path::PathBuf) -> Self{
        let c = path.parent().unwrap()
            .iter().last().unwrap()
            .to_str().unwrap().to_string();
         ImgData{path: path, class: c}
    }
    fn load_img(&self) -> image::GrayImage{
        let size = 32;
        let img = match image::open(&self.path.as_path()){
                Ok(value) => value.to_luma(),
                Err(_) => panic!("Can't read image!!"),
            };
        image::imageops::resize(&img,size,size,image::FilterType::Lanczos3)
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
    fn take_n_rand  (&self,n: u32)->MiniBatch{
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
    fn new () ->Self{
        let get_rand = ||{
            use rand::distributions::Range;
            use ndarray::Array;
            use ndarray_rand::RandomExt;
            Array::random((9,9),Range::new(0.0, 255.0))
        };
        let layer_1 = get_rand();
        let layer_2 = get_rand();
        let layer_3 = {
            use rand::distributions::Range;
            use ndarray::Array;
            use ndarray_rand::RandomExt;
            Array::random((9,9,9),Range::new(0.0, 255.0))
        };
        CSom{layer_1: layer_1,layer_2:layer_2,layer_3:layer_3}
    }
    fn train (&self, n: u32, dataset :&DataSet){
        let minibatch = dataset.take_n_rand(n);
        for entry in minibatch{
            entry.load_img();
            println!("{}",entry.class);
        }
    }
}


fn main() {
    let path = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset:DataSet = DataSetTrait::new(path);
    let csom = CSom::new();
    csom.train(100,&dataset);
    //let minibatch = take_n_rand(100,dataset);
    //train(100,&dataset);
}
