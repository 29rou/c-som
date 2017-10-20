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
        let cifar_dataset = cifar::dataset::CifarDataset::new(PATH).unwrap();
        let rng = &mut rand::thread_rng();
        println!("{:?}", cifar_dataset.labels);
        println!("Test Data Count: {}", cifar_dataset.test_count);
        println!("Train Data Count:{}", cifar_dataset.train_count);
        cifar_dataset.for_test_get_image_from_train_save(rng).unwrap();
        cifar_dataset.for_test_get_image_from_test_save(rng).unwrap();
        cifar_dataset
    };
}
fn main() {
    //let cifar_dataset = &CIFARDATASET;
    let rng = &mut rand::thread_rng();
    let csom = csom::Csom::Csom::new(rng).unwrap();
    csom.train(&CIFARDATASET, rng);

    use csom::somlayers::SomLayersTrait;
    println!("{:?}", csom.som_layers.layer(0));
    println!("{:?}", csom.som_layers.shape());
}
