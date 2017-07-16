use imgdata::ImgData;
use MiniBatch;

pub type DataSet  = Vec<ImgData>;

pub trait DataSetTrait {
    fn new(p:&str)->Self;
    fn take_n_rand(&self, n:usize)->MiniBatch;
}

impl DataSetTrait for DataSet{
    fn new (p: &str) ->  Self{
        extern crate walkdir;
        walkdir::WalkDir::new(p)
            .into_iter()
            .map(|x| x.unwrap().path().to_path_buf())
            .filter(|x| x.is_file())
            .map(|x| { ImgData::new(x)})
            .collect::<DataSet>()
    } 
    fn take_n_rand  (&self,n: usize)->MiniBatch{
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let mut v :MiniBatch = Vec::new();
        v.reserve(n as usize);
        for _ in 0..n{
            let r= rng.choose(&self).unwrap();
            v.push(r);
        }
        v
    }
}