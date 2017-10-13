extern crate walkdir;
extern crate image;

use std;
use std::collections::LinkedList;

#[derive (Clone)]
pub struct CifarDataset {
    pub labels: Vec<String>,
    pub dataset: Vec<CifarImage>
}

#[derive (Clone)]
pub struct CifarImage {
    pub label: u8,
    pub image: image::DynamicImage
}

impl CifarDataset{
    pub fn new(path: &std::path::Path) -> Self{
        let ref paths = walkdir::WalkDir::new(path)
            .into_iter()
            .flat_map(|x| x)
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .collect::<LinkedList<std::path::PathBuf>>();
        let meta_data = CifarDataset::get_meta_data(paths);
        let binary_datas = CifarDataset::get_byte_datas(paths);
        let cifar_images = binary_datas.into_iter()
            .map(|ref mut x| CifarImage::new(x))
            .collect();
        CifarDataset{labels:meta_data ,dataset:cifar_images}
    }
    fn get_meta_data(paths: &LinkedList<std::path::PathBuf>) -> Vec<String>{
        use std::io::Read;
        let meta_path= paths
            .iter()
            .find(|x| x.extension().unwrap()=="txt")
            .expect("Can't Find MetaFile!!");
        let mut lines = String::new();
        std::fs::File::open(meta_path)
            .expect("Cant Open MetaFile!!")
            .read_to_string(&mut lines)
            .expect("Cant Read MetaFile!!");
        lines.lines()
            .map(|x| x.to_string())
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>()
    }
    fn get_byte_datas(paths: &LinkedList<std::path::PathBuf>) -> Vec<Vec<u8>>{
        use std::io::{BufReader, Read};
        paths.iter()
        .filter(|x| x.extension().unwrap()=="bin")
        .map(|x| -> Vec<u8>{
            let file = std::fs::File::open(x).expect("Can't Open File!!");
            let mut binaray_data:Vec<u8> = Vec::new();
            BufReader::new(file)
                .read_to_end(&mut binaray_data)
                .expect("Can't Read File!!");
            binaray_data
        })
        .map(|x| -> Vec<Vec<u8>>{
            x.chunks(3073).map(|x| x.to_vec()).collect()
        })
        .fold(Vec::new(),|mut sum,ref mut x| -> Vec<Vec<u8>>{
            sum.append(x);
            sum
        })
    }
}

impl CifarImage{
    fn new(bytes: &mut Vec<u8>)-> Self{
        use std::io::Read;
        use std::mem;
        use self::image::GenericImage;
        let mut bytes:&[u8] = bytes;
        let label = unsafe{
            let ref mut label:[u8;1] = mem::uninitialized();
            bytes.read_exact(label).unwrap();
            *label.get_unchecked(0)
        };
        let img = unsafe{
            let mut img = image::DynamicImage::new_rgb8(32,32);
            let ref mut red:[u8;1024] = mem::uninitialized();
            let ref mut green:[u8;1024] = mem::uninitialized();
            let ref mut blue:[u8;1024] = mem::uninitialized();
            bytes.read_exact(red).unwrap();
            bytes.read_exact(green).unwrap();
            bytes.read_exact(blue).unwrap();
            for y in 0..32{
                for x in 0..32{
                    let i = x + y * 32;
                    let mut pixel:image::Rgba<u8> = mem::uninitialized();
                    pixel.data = [
                        *red.get_unchecked(i),
                        *green.get_unchecked(i),
                        *blue.get_unchecked(i),
                        255
                    ];
                    img.unsafe_put_pixel(x as u32 ,y as u32 ,pixel);
                }
            }
            img
        };
        CifarImage{label: label, image: img}
    }
}