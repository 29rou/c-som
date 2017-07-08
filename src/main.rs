extern crate image;
use std::path;
//use std::borrow::Cow;

type DataSet <'a>  = Vec<ImgData>;

struct ImgData  {
    path: Box<path::PathBuf>,
    class: Box<String>,
}

trait HasImg{
    fn load_img(&self) -> image::RgbImage;
}

impl <'a> ImgData{
    fn new (path:  &path::Path) -> Self{
        let path = path.to_path_buf();
        let path = Box::new(path);
        //let path :Cow<'a, path::Path>= Cow::Borrowed(path);
        let c = path.parent().unwrap().to_str().unwrap().to_string();
        //let c = path;path::Path::new(path)path::Path::new(path)
        //let c = c.parent().unwrap().to_str().unwrap();
        //ImgData{path: path, class: Cow::Borrowed(c)}
         ImgData{path: path, class: Box::new(c)}
    }
}

impl <'a> HasImg for  ImgData{
    fn load_img(&self) -> image::RgbImage{
        let jpeg_img = image::open(&self.path.as_path());
        match jpeg_img{
            Ok(value) => value.to_rgb(),
            Err(_) => panic!("Can't read image!!"),
        }
    }
}

fn prepare_dataset<'a> (p: &str) ->  DataSet{
    extern crate walkdir;
    let ps = walkdir::WalkDir::new(p)
        .into_iter()
        .map(|x| ImgData::new(x.unwrap().path()))
        .collect::<Vec<ImgData>>();
    ps
}                                                                               

fn main() {
    println!("Hello, world!");
    let path:&str = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset:DataSet = prepare_dataset(path);
    for entry in dataset{
       println!("{}",entry.path.display());
    }
}
