extern crate walkdir;
extern crate image;
extern crate rand;
extern crate itertools;

use std;
use std::collections::LinkedList;

pub struct CifarDataset {
    pub count: usize,
    pub labels: Vec<String>,
    pub dataset: Vec<CifarImage>
}

pub struct CifarImage {
    pub label: u8,
    pub image: image::DynamicImage
}

impl CifarDataset{
    pub fn new(path: &std::path::Path) -> Self{
        let ref paths = CifarDataset::get_file_paths(path);
        let meta_data = CifarDataset::get_meta_data(paths);
        let byte_datas = CifarDataset::get_byte_datas(paths);
        let cifar_images = CifarDataset::get_images(byte_datas);
        let count =  cifar_images.len();
        CifarDataset{labels:meta_data ,dataset:cifar_images,count:count}
    }
    fn get_file_paths(path: &std::path::Path) -> LinkedList<std::path::PathBuf>{
        walkdir::WalkDir::new(path)
            .into_iter()
            .flat_map(|x| x)
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .collect::<LinkedList<std::path::PathBuf>>()
    }
    fn get_meta_data(paths: &LinkedList<std::path::PathBuf>) -> Vec<String>{
        use std::io::Read;
        use self::itertools::Itertools;
        paths.iter()
            .filter(|x| x.extension().expect("Can't Find MetaFile!!")=="txt")
            .map(|meta_path| ->String{
                let mut lines = String::new();
                std::fs::File::open(meta_path)
                    .expect("Cant Open MetaFile!!")
                    .read_to_string(&mut lines)
                    .expect("Cant Read MetaFile!!");
                lines
            })
            .map(|lines| -> Vec<String>{
                lines.lines()
                    .map(|x| x.to_string())
                    .filter(|x| !x.is_empty())
                    .collect_vec()
            })
            .concat()
    }
    fn get_byte_datas(paths: &LinkedList<std::path::PathBuf>) -> Vec<Vec<u8>>{
        use std::io::{BufReader, Read};
        use self::itertools::Itertools;
        paths.iter()
            .filter(|x| x.extension().expect("Can't Find Bin File!!")=="bin")
            .map(|x| -> Vec<u8>{
                let file = std::fs::File::open(x).expect("Can't Open Bin File!!");
                let mut binaray_data:Vec<u8> = Vec::new();
                BufReader::new(file)
                    .read_to_end(&mut binaray_data)
                    .expect("Can't Read Bin File!!");
                binaray_data
            })
            .map(|x| -> Vec<Vec<u8>>{
                x.chunks(3073).map(|x| x.to_vec()).collect_vec()
            })
            .concat()
    }
    fn get_images(byte_datas: Vec<Vec<u8>>) -> Vec<CifarImage>{
        byte_datas
            .into_iter()
            .map(move |x|{
                std::thread::spawn(move ||
                    CifarImage::new(& x)
                )
            })
            .map(|x| x.join().expect("Thread Error!!"))
            .collect()   
    }
    pub fn for_test_get_image_by_save(&self){
        use self::rand::{thread_rng, Rng};
        let ref nth:usize = thread_rng().gen_range(0,self.count);
        let ref data = self.dataset[*nth];
        let ref mut fout = 
            std::fs::File::create(&std::path::Path::new("test.jpeg"))
            .expect("Can't Ready To Save Image!!");
        data.image
            .resize(500,500,image::FilterType::Lanczos3)
            .save(fout, image::JPEG).expect("Can't Save Image!!");
        println!("No.{} {}",nth,self.labels[data.label as usize]);
    }
}

impl CifarImage{
    fn new(bytes: &Vec<u8>)-> Self{
        use std::io::Read;
        use std::mem;
        use self::image::GenericImage;
        use self::itertools::Itertools;
        let ref mut bytes:&[u8] = bytes.as_ref();
        let label = unsafe{
            let ref mut label:[u8;1] = mem::uninitialized();
            bytes.read_exact(label).expect("Can't Read Label!!");
            *label.get_unchecked(0)
        };
        let img = unsafe{
            let mut img = image::DynamicImage::new_rgb8(32,32);
            let ref mut red:[u8;1024] = mem::uninitialized();
            let ref mut green:[u8;1024] = mem::uninitialized();
            let ref mut blue:[u8;1024] = mem::uninitialized();
            bytes.read_exact(red).expect("Can't Read Red!!");
            bytes.read_exact(green).expect("Can't Read Green!!");
            bytes.read_exact(blue).expect("Can't Read Blue!!");
            for (x,y) in (0..32).cartesian_product((0..32)){
                let i = x + y * 32;
                let ref mut pixel:image::Rgba<u8> = mem::uninitialized();
                pixel.data = [
                    *red.get_unchecked(i),
                    *green.get_unchecked(i),
                    *blue.get_unchecked(i),
                    255
                ];
                img.unsafe_put_pixel(x as u32 ,y as u32 ,*pixel);
            }
            img
        };
        CifarImage{label: label, image: img}
    }
}