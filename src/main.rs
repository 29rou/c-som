#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate cifar_10_loader;
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod csom;


lazy_static!{
    static ref CIFARDATASET: cifar_10_loader::CifarDataset = {
        const PATH: &str = "./cifar-10-batches-bin/";
        let cifar_dataset = cifar_10_loader::CifarDataset::new(PATH).unwrap();
        let rng = &mut rand::thread_rng();
        cifar_dataset.info_output();
        cifar_dataset.test_output(rng).unwrap();
        cifar_dataset
    };
}
fn main() {
    println!("START!!");
    let rng = &mut rand::thread_rng();
    let csom = csom::Csom::new(rng).unwrap();
    csom.train(&CIFARDATASET, rng);
    //csom.output().unwrap();
}
