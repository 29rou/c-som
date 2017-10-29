#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate cifar_10_loader;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod csom;

fn for_test_get_image_from_train_save(
    data_set: &cifar_10_loader::CifarDataset,
    rng: &mut rand::ThreadRng,
) -> Result<(), String> {
    use self::rand::Rng;
    //use self::cifar_10_loader::image;
    let fout = &mut ::std::fs::File::create(&::std::path::Path::new("train.jpeg"))
        .map_err(|err| err.to_string())?;
    let nth: &usize = &rng.gen_range(0, data_set.train_count);
    let data: &cifar_10_loader::CifarImage = &data_set.train_dataset[*nth];
    data.image
        .resize(500, 500, image::FilterType::Lanczos3)
        .save(fout, image::JPEG)
        .map_err(|err| err.to_string())?;
    println!(
        "From Train No.{} {}",
        nth,
        data_set.labels[data.label as usize]
    );
    Ok(())
}
fn for_test_get_image_from_test_save(
    data_set: &cifar_10_loader::CifarDataset,
    rng: &mut rand::ThreadRng,
) -> Result<(), String> {
    //use self::cifar_10_loader::image;
    use self::rand::Rng;
    let fout = &mut ::std::fs::File::create(&::std::path::Path::new("test.jpeg"))
        .map_err(|err| err.to_string())?;
    let nth: &usize = &rng.gen_range(0, data_set.test_count);
    let data: &cifar_10_loader::CifarImage = &data_set.test_dataset[*nth];
    data.image
        .resize(500, 500, image::FilterType::Lanczos3)
        .save(fout, image::JPEG)
        .map_err(|err| err.to_string())?;
    println!(
        "From test No.{} {}",
        nth,
        data_set.labels[data.label as usize]
    );
    Ok(())
}
pub fn info_output(data_set: &cifar_10_loader::CifarDataset) {
    println!("{:?}", data_set.labels);
    println!("Test Data Count: {}", data_set.test_count);
    println!("Train Data Count:{}", data_set.train_count);
}
pub fn test_output(
    data_set: &cifar_10_loader::CifarDataset,
    rng: &mut rand::ThreadRng,
) -> Result<(), String> {
    for_test_get_image_from_train_save(data_set, rng)?;
    for_test_get_image_from_test_save(data_set, rng)?;
    Ok(())
}


lazy_static!{
    static ref CIFARDATASET: cifar_10_loader::CifarDataset = {
        const PATH: &str = "./cifar-10-batches-bin/";
        let cifar_dataset = cifar_10_loader::CifarDataset::new(PATH).unwrap();
        let rng = &mut rand::thread_rng();
        info_output(&cifar_dataset);
        test_output(&cifar_dataset,rng).unwrap();
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
