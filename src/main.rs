extern crate walkdir;
extern crate image;

struct CifarImage {
    label: String,
    image: image::DynamicImage
}

fn read_cifar(path: &std::path::Path) -> Vec<CifarImage>{
    use std::{fs, mem};
    use std::collections::LinkedList;
    use std::io::{BufReader, Read};
    use self::walkdir::WalkDir;
    let paths = &WalkDir::new(path)
        .into_iter()
        .flat_map(|x| x)
        .map(|x| x.path().to_path_buf())
        .filter(|x| x.is_file())
        .collect::<LinkedList<std::path::PathBuf>>();
    let meta_data:Vec<String> = {
        let meta_path= paths
            .iter()
            .find(|x| x.extension().unwrap()=="txt")
            .expect("Can't Find MetaFile!!");
        let mut lines = String::new();
        fs::File::open(meta_path)
            .expect("Cant Open MetaFile!!")
            .read_to_string(&mut lines)
            .expect("Cant Read MetaFile!!");
        lines.lines()
            .map(|x| x.to_string())
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>()
    };
    let mut binary_datas:Vec<u8> = Vec::new();
    for img_path in paths.iter().filter(|x| x.extension().unwrap()=="bin"){
        let file = 
            fs::File::open(img_path)
            .expect("Can't Open File!!");
        let mut binary_data:Vec<u8> = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut binary_data)
            .expect("Can't Read File!!");
        binary_datas.append(&mut binary_data);
    }
    let mut cifar_images:Vec<CifarImage> = Vec::new();
    {
        let mut binary_datas = &binary_datas[..];
        while !binary_datas.is_empty(){
            let (label,img) = unsafe{
                use  image::GenericImage;
                let mut label:[u8;1] = mem::uninitialized();
                let mut red:[u8;1024] = mem::uninitialized();
                let mut green:[u8;1024] = mem::uninitialized();
                let mut blue:[u8;1024] = mem::uninitialized();
                binary_datas.read_exact(&mut label).unwrap();
                binary_datas.read_exact(&mut red).unwrap();
                binary_datas.read_exact(&mut green).unwrap();
                binary_datas.read_exact(&mut blue).unwrap();
                let mut img = image::DynamicImage::new_rgb8(32,32);
                for y in 0..32{
                    for x in 0..32{
                        let i = x + y * 32;
                        let mut pixel:image::Rgba<u8> = mem::uninitialized();
                        pixel.data = [red[i],green[i],blue[i],255];
                        img.unsafe_put_pixel(x as u32 ,y as u32 ,pixel);
                    }
                }
                (label[0],img)
            };
            let tmp = CifarImage{
                label: meta_data[label as usize].clone(),
                image: img
            };
            cifar_images.push(tmp);
        }
    }
    cifar_images
}

fn main(){
    const PATH: &str = "./cifar-10-batches-bin/";
    let path = std::path::Path::new(PATH);
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}",args);
    let cifar_images = read_cifar(path);
    let img = cifar_images[500].image.resize(500,500,image::FilterType::Lanczos3);
    println!("{}",cifar_images[500].label);
    let ref mut fout = std::fs::File::create(&std::path::Path::new("test.png")).unwrap();
    let _ = img.save(fout, image::PNG).unwrap();
    ()
}