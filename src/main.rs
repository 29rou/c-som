extern crate typenum;
mod imgdata;
mod csom;
mod dataset;

use typenum::*;
use dataset::{DataSet, DataSetTrait};
use csom::{CSomTrait, CSom};

fn main() {
    const PATH: &str = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset: DataSet = DataSetTrait::new(PATH);
    let csom: CSom<f32, U9, U2, U9, U9> = CSomTrait::new();
    println!("START!! Train!!");
    csom.train(10, 1000, &dataset);
}
