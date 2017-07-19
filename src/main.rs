extern crate generic_array;
extern crate typenum;
mod imgdata;

mod csom;
mod dataset;

use typenum::*;

use generic_array::{GenericArray,ArrayLength};
use typenum::{U0,U1,U2,U3,U9,U32,U100,consts};

fn main() {
    use dataset::{DataSet,DataSetTrait};
    const PATH:&str = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset:DataSet = DataSetTrait::new(PATH);
    let csom:csom::CSom<f32,U9,U2,U9,U9> = csom::CSom::new();
    csom.train(100,100,&dataset);
}
