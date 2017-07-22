extern crate typenum;
mod imgdata;
mod csom;
mod dataset;

use typenum::*;

fn main() {
    use dataset::{DataSet, DataSetTrait};
    const PATH: &str = "/home/yoshiki/Downloads/101_ObjectCategories";
    let dataset: DataSet = DataSetTrait::new(PATH);
    let csom: csom::CSom<f32, U9, U2, U9, U9> = csom::CSom::new();
    for entry in csom.mid_layers.as_ref(){
        let entry = entry.lock().unwrap()[0];
        for entry in entry{
            print!("{} ",entry);
        }
        println!("");
    }
    println!("START!! Train!!");
    csom.train(10, 1000, &dataset);
}

#[cfg(test)]
mod tests {
    #![feature(test)]
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_main(b: &mut Bencher) {
        use dataset::{DataSet, DataSetTrait};
        const PATH: &str = "/home/yoshiki/Downloads/101_ObjectCategories";
        let dataset: DataSet = DataSetTrait::new(PATH);
        let csom: csom::CSom<f32, U9, U2, U9, U9> = csom::CSom::new();
        b.iter(move || csom.train(10, 1000, &dataset));
    }
}
