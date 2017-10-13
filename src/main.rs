extern crate walkdir;

fn read_cifar(path: &std::path::Path) -> i32{
    use std::{fs, mem};
    use std::io::{BufReader, Read};
    use self::walkdir::WalkDir;
    struct BinaryImg {
        label:[u8;1],
        red:[u8;1024],
        green:[u8;1024],
        blue:[u8;1024]
    } 
    let paths = WalkDir::new(path)
        .into_iter()
        .flat_map(|x| x)
        .map(|x| x.path().to_path_buf())
        .filter(|x| x.is_file())
        .collect::<Vec<std::path::PathBuf>>();
    let meta_paths:Vec<std::path::PathBuf> = (&paths)
        .into_iter()
        .by_ref()
        .filter(|x| x.extension().unwrap()=="txt")
        .map(|x| x.clone())
        .collect();
    let img_paths:Vec<std::path::PathBuf> = paths
        .into_iter()
        .filter(move |x| x.extension().unwrap()=="bin")
        .collect();
    println!("{:?}",img_paths);
    let mut imgs:Vec<Vec<BinaryImg>> = Vec::new();
    for img_path in img_paths{
        let file = 
            fs::File::open(img_path)
            .expect("Can't Open File!!");
        let mut reader = BufReader::new(file);
        let mut binary_data:Vec<u8> = Vec::new();
        let binary_data_size = reader.read_to_end(&mut binary_data).expect("Can't Read File!!");
        let mut binary_data = &binary_data[..];
        println!("{:?}",binary_data_size);
        let mut binary_imgs:Vec<BinaryImg> = 
            Vec::with_capacity(binary_data_size/((1+1024*3)*8));
        while !binary_data.is_empty(){
            let img = unsafe{
                let mut tmp:BinaryImg;
                tmp = mem::uninitialized();
                binary_data.read_exact(&mut tmp.label).unwrap();
                binary_data.read_exact(&mut tmp.red).unwrap();
                binary_data.read_exact(&mut tmp.green).unwrap();
                binary_data.read_exact(&mut tmp.blue).unwrap();
                tmp
            };
            binary_imgs.push(img);
        }
        imgs.push(binary_imgs);
    }

    0
}

fn main(){
    const PATH: &str = "./cifar-10-batches-bin/";
    let path = std::path::Path::new(PATH);
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}",args);
    read_cifar(path);
}