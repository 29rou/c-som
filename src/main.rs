#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#[macro_use]
extern crate lazy_static;
extern crate ndarray;
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
    let rng = &mut rand::thread_rng();
    cifar_dataset
        .for_test_get_image_from_train_save(rng)
        .unwrap();
    cifar_dataset
        .for_test_get_image_from_test_save(rng)
        .unwrap();
    let csom = csom::Csom::Csom::new(rng);
    println!("{:?}", cifar_dataset.labels);
    println!("Test Data Count: {}", cifar_dataset.test_count);
    println!("Train Data Count:{}", cifar_dataset.train_count);
    csom.train(cifar_dataset, rng);
    //println!("{:?}", csom.som_layers[0]);
    use ndarray::{Axis, arr3, aview2};
    println!("{:?}", csom.som_layers);
    println!("{:?}", csom.som_layers.subview(Axis(0), 0));
    println!("{:?}", csom.som_layers.subview(Axis(0), 1));
    println!("{:?}", csom.som_layers.subview(Axis(0), 2));
    use csom::Csom::SomLayersTrait;
    println!("{:?}", csom.som_layers.layer(0));
    let test = csom.som_layers.subview(Axis(0), 2);
    println!("{:?}", csom.som_layers.shape());
}
