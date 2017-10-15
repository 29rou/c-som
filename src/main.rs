#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#[macro_use]
extern crate lazy_static;

mod cifar;

lazy_static!{
    static ref CIFARDATASET:cifar::CifarDataset = {
        //let args = &std::env::args().collect::<Vec<String>>();
        const PATH: &str = "./cifar-10-batches-bin/";
        let path:&std::path::Path = std::path::Path::new(PATH);
        cifar::CifarDataset::new(path).unwrap()
    };
}
fn main() {
    let cifar_dataset = &CIFARDATASET;
    println!("Count:{}", cifar_dataset.count);
    cifar_dataset.for_test_get_image_by_save().unwrap();
}
