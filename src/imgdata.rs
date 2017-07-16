extern crate ndarray;
extern crate image;
use std;

pub struct ImgData  {
    path: std::path::PathBuf,
    class: String,
}

impl  ImgData{
    pub fn new (path:std::path::PathBuf) -> Self{
        let c = path.parent().unwrap()
            .iter().last().unwrap()
            .to_str().unwrap().to_string();
         ImgData{path: path, class: c}
    }
    pub fn load_img(&self,size:usize) -> ndarray::Array2<f32>{
        use self::ndarray::Array;
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
