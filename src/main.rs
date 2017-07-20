extern crate typenum;
mod imgdata;
mod csom;
mod dataset;

use typenum::*;


fn main() {
    use dataset::{DataSet,DataSetTrait};
    const PATH:&str = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset:DataSet = DataSetTrait::new(PATH);
    let csom:csom::CSom<f32,U9,U2,U9,U9> = csom::CSom::new();
    csom.train(10,1000,&dataset);
    //let mut dst = [0,0,0,0];
    //let src = [1,2,3,4,5,6];
    //println!("{}",dst.copy_from_slic(&src));
}
