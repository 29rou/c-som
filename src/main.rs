extern crate typenum;
mod imgdata;
mod csom;
mod dataset;

use typenum::*;

fn main() {
    use dataset::{DataSet, DataSetTrait};
    use csom::{CSomTrait, CSom};
    const PATH: &str = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset: DataSet = DataSetTrait::new(PATH);
    let csom: CSom<f32, U9, U2, U9, U9> = CSomTrait::new();
    for entry in csom.mid_layers.as_ref() {
        let entry = entry.lock().unwrap()[0];
        for entry in entry {
            print!("{} ", entry);
        }
        println!("");
    }
    println!("START!! Train!!");
    csom.train(10, 1000, &dataset);
}
