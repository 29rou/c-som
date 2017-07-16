extern crate rand;
extern crate generic_array;
use imgdata::ImgData;
use MiniBatch;
use self::generic_array::{ArrayLength,GenericArray};

pub type DataSet  = Vec<ImgData>;

pub trait DataSetTrait {
    fn new(p:&str)->Self;
    fn take_n_rand(&self)->MiniBatch;
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
    fn take_n_rand (&self)->MiniBatch{
        use self::rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        unsafe{
            use std;
            let mut minibatch:MiniBatch = std::mem::uninitialized();
            for i in minibatch.as_mut(){
                *i = rng.choose(&self).unwrap();
            }
            minibatch
        }
    }
}