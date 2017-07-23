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
    //csom.train(10, 1000, &dataset);
    let a = [[0, 1,  2, 3],
             [4, 5, 6, 7],
             [8, 9,  10, 11],
             [12,13,14,15]];
    let mut slice = &[&a[0][0..(0+3)], &a[0+1][0..(0+3)], &a[0+2][0..(0+3)]];
    println!("{:?}",slice);
    let b = a.windows(2);
    let c = b.clone().map(|x| x.iter().map(|y| y.windows(2)));
    println!("{:?}", c);
    for x in c{
        for y in x{
            for z in y{
                println!("{:?}",z);
            }
        }
    }
}

