extern crate ndarray;
extern crate image;
extern crate generic_array;
use std;
use generic_array::{GenericArray,ArrayLength,typenum};

type Image<T,R,C> = GenericArray<GenericArray<T,R>,C>;


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
    pub fn load_img<T,R,C>(&self) -> Image<T,R,C>
    where T:From<u8>,
          R:ArrayLength<T>,
          C:ArrayLength<GenericArray<T,R>>
    {
        use self::ndarray::Array;
        let mut img_array:Image<T,R,C>;
        unsafe{
            img_array = std::mem::uninitialized();
            let size = img_array.iter().count();
            let img = image::imageops::resize(
                &image::open(&self.path.as_path())
                .expect("Can't read image!!")
                .to_luma(),
                size as u32,
                size as u32,
                image::FilterType::Lanczos3
            );
            for i in 0..size{
                for j in 0..size{
                    let iter = i*size+j;
                    img_array[i][j] = (*img.get(iter).unwrap()).into();
                }
            }
        }
        img_array
    }
}
