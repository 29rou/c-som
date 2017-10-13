extern crate image;

mod cifar;

fn main(){
    const PATH: &str = "./cifar-10-batches-bin/";
    let path = std::path::Path::new(PATH);
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}",args);
    let cifar_dataset = cifar::CifarDataset::new(path);
    let img = cifar_dataset.dataset[323].image.resize(500,500,image::FilterType::Lanczos3);
    let ref mut fout = std::fs::File::create(&std::path::Path::new("test.png")).unwrap();
    let _ = img.save(fout, image::PNG).unwrap();
    ()
}