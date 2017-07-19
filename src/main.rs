extern crate generic_array;
#[macro_use]extern crate typenum;
mod imgdata;
use imgdata::ImgData;

mod csom;
mod dataset;


use generic_array::{GenericArray,ArrayLength};
use typenum::{U0,U1,U2,U3,U9,U32,U100,consts};
type MiniBatch<'a,T,R,C> = Vec<&'a ImgData<T,R,C>>;

fn main() {
    use dataset::{DataSet,DataSetTrait};
    const PATH:&str = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset:DataSet<f32,U32,U32> = DataSetTrait::new(PATH);
    let csom:csom::CSom<f32,U9,U2,U9,U9> = csom::CSom::new();
    csom.train(10,100,&dataset);
}
