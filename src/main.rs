extern crate image;
use std::path;
use std::borrow::Cow;

struct ImgData<'a>{
    file_path: &'a path::Path,
}

type DataSet <'a> = Vec<&'a ImgData<'a>>;

trait HasImg{
    fn get_class(&self) -> &str;
    fn load_img(&self) -> image::RgbImage;
}

impl <'a> HasImg for  ImgData<'a>{
    fn get_class (&self) -> &str{
        self.file_path.parent().unwrap().to_str().unwrap()
    }
    fn load_img(&self) -> image::RgbImage{
        let jpeg_img = image::open(&self.file_path);
        match jpeg_img{
            Ok(value) => value.to_rgb(),
            Err(_) => panic!("Can't read image!!"),
        }
    }
}

fn prepare_dataset<'a> (p: &path::Path) ->  DataSet{
    extern crate walkdir;
    let vec = 
        walkdir::WalkDir::new(p)
        .into_iter()
        .map(|x| match x {
            Ok(value) => ImgData{file_path: & value.path()},
            Err(_) => panic!("Can't get path!!"),
        });
        //.collect::<DataSet<'a>>();
        vec
}                                                     
                                

fn main() {
    println!("Hello, world!");
    let path = path::Path::new("/home/yoshiki/Downloads/101_ObjectCategories");
    let dataset = prepare_dataset(path);
    for entry in dataset{
       println!("{}",entry.file_path.display());
    }
}
