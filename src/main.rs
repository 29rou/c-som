#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod cifar;
mod csom;


lazy_static!{
    static ref CIFARDATASET: cifar::dataset::CifarDataset = {
        //let args = &std::env::args().collect::<Vec<String>>();
        const PATH: &str = "./cifar-10-batches-bin/";
        let path:&std::path::Path = std::path::Path::new(PATH);
        cifar::dataset::CifarDataset::new(path).unwrap()
    };
}
fn main() {
    //use csom::Csom::Csom;
    let cifar_dataset = &CIFARDATASET;
    println!("Count:{}", cifar_dataset.count);
    cifar_dataset.for_test_get_image_by_save().unwrap();
    let rng = &mut rand::thread_rng();
    let csom = csom::Csom::Csom::new(rng);
    csom.train(cifar_dataset, rng);
    println!("{:?}", csom.fully_connected_layers.layer1[0])
}
