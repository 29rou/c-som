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
    /*let csom:csom::CSom<f32,U9,U2,U9,U9> = csom::CSom::new();
    for i in csom.mid_layers{
        for j in i{
            for k in j{
                println!("{}",k);
            }
        }
    }*/
    /*let img = &dataset.get(4).unwrap().load_img(32);
    let a = img.subview(ndarray::Axis(0),0);
    let b = img.subview(ndarray::Axis(0),1);
    let c = (&a - &b).map(|x|(x.pow(2)as f32)).scalar_sum().sqrt();
    println!("{}\n\n{}\n\n{}\n\n{}",img,a,b,c);
    let w =img.windows((3,3));
    let mut w = w.into_iter();
    let w = w.next().unwrap();
    println!("{}",w);*/
    //csom.train(10,100,&dataset);
}
