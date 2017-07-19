extern crate image;
extern crate generic_array;
use std;
use generic_array::{GenericArray,ArrayLength};

pub type Image<T,R,C> = GenericArray<GenericArray<T,C>,R>;

#[derive (Clone)]
pub struct ImgData
{
    path: std::path::PathBuf,
    class: String
}

impl  ImgData{
    pub fn new (path:std::path::PathBuf) -> Self{
        let c = path.parent().unwrap()
            .iter().last().unwrap()
            .to_str().unwrap().to_string();
         ImgData{path: path, class: c}
    }
    pub fn load_img<T,R,C>(&self) -> Image<T,R,C>
    where   T:From<u8>,
            R:ArrayLength<GenericArray<T,C>>,
            C:ArrayLength<T>
    {
        let img = &image::open(self.path.as_path())
            .expect("Can't read image!!")
            .to_luma();
        let mut img_array:Image<T,R,C>;
        unsafe{
            img_array = std::mem::uninitialized();
            let img = image::imageops::resize(
                img,
                C::to_u32(), 
                R::to_u32(),
                image::FilterType::Triangle
            );
            for i in 0..R::to_usize(){
                for j in 0..C::to_usize(){
                    let iter = i * R::to_usize() + j;
                    img_array[i][j] = (*img.get(iter).unwrap()).into();
                }
            }
        }
        img_array
    }
}
