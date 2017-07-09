extern crate image;
use std::path;
extern crate rand;
use rand::{thread_rng, Rng};

type DataSet  = Vec<ImgData>;
type MiniBatch<'a>  = Vec<&'a ImgData>;

type CSomFilter = [[f32; 9] ;9];

struct CSom {
    layer_1: CSomFilter,
    layer_2: CSomFilter
}



struct ImgData  {
    path: Box<path::PathBuf>,
    class: Box<String>,
}

trait HasImg{
    fn load_img(&self) -> image::GrayImage;
}

impl  ImgData{
    fn new (path:  path::PathBuf) -> Self{
        let path = Box::new(path);
        let c = path.parent().unwrap()
            .iter().last().unwrap()
            .to_str().unwrap().to_string();
         ImgData{path: path, class: Box::new(c)}
    }
}

impl  HasImg for  ImgData{
    fn load_img(&self) -> image::GrayImage{
        let size = 32;
        let img = match image::open(&self.path.as_path()){
                Ok(value) => value.to_luma(),
                Err(_) => panic!("Can't read image!!"),
            };
        image::imageops::resize(&img,size,size,image::FilterType::Lanczos3)
    }
}

fn prepare_dataset (p: &str) ->  DataSet{
    extern crate walkdir;
    walkdir::WalkDir::new(p)
        .into_iter()
        .map(|x| x.unwrap().path().to_path_buf())
        .filter(|x| x.is_file())
        .map(|x| { ImgData::new(x)})
        .collect::<DataSet>()
}                                                                               

fn take_n_rand (n: u32 ,dataset :&DataSet)->MiniBatch{
    let mut rng = thread_rng();
    let mut v = Vec::new();
    for _ in 0..n{
        let r = rng.choose(dataset).unwrap();
        v.push(r);
    }
    v
}

fn main() {
    let path = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset = &prepare_dataset(path);
    //let minibatch = take_n_rand(100,dataset);
    for _ in 0..10{
        for entry in take_n_rand(100,dataset){
            entry.load_img();
            println!("{}",entry.class);
        }
    }
}
