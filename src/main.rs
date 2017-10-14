
extern crate image;
#[macro_use]
extern crate lazy_static;

mod cifar;

fn main(){
    const PATH: &str = "./cifar-10-batches-bin/";
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}",args);
    lazy_static!{
        static ref CIFAR_DATASET:cifar::CifarDataset = {
            let path = std::path::Path::new(PATH);
            cifar::CifarDataset::new(path)
        };
    }
    let img = CIFAR_DATASET.dataset[1023].image.resize(500,500,image::FilterType::Lanczos3);
    let ref mut fout = std::fs::File::create(&std::path::Path::new("test.png")).unwrap();
    let _ = img.save(fout, image::PNG).unwrap();
    ()
}