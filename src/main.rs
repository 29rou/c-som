extern crate generic_array;

mod imgdata;
use imgdata::ImgData;

mod csom;
mod dataset;
use generic_array::{GenericArray,ArrayLength,typenum};
type MiniBatch<'a> = GenericArray<&'a ImgData,typenum::U100>;

fn main() {
    let path = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset:dataset::DataSet = dataset::DataSetTrait::new(path);
    use typenum::U9;
    let csom:csom::CSom<f32,U9,U9,U9> = csom::CSom::new();
    for i in csom.layer_1{
        for j in i{
            println!("{}",j);
        }
    }
    /*let img = &dataset.get(4).unwrap().load_img(32);
    let a = img.subview(ndarray::Axis(0),0);
    let b = img.subview(ndarray::Axis(0),1);
    let c = (&a - &b).map(|x|(x.pow(2)as f32)).scalar_sum().sqrt();
    println!("{}\n\n{}\n\n{}\n\n{}",img,a,b,c);
    let w =img.windows((3,3));
    let mut w = w.into_iter();
    let w = w.next().unwrap();
    println!("{}",w);*/
    csom.train(10,100,&dataset);
}
