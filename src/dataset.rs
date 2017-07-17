extern crate rand;
extern crate walkdir;
extern crate generic_array;
use imgdata::ImgData;
use MiniBatch;
use self::generic_array::{ArrayLength,GenericArray};

pub type DataSet<T,R,C>  = Vec<ImgData<T,R,C>>;

pub trait DataSetTrait <T,R,C>
where   T:From<u8>,
        R:ArrayLength<GenericArray<T,C>>,
        C:ArrayLength<T>
{
    fn new(p:&str)->Self;
    fn get_minibatch(&self)->MiniBatch<T,R,C>;
}

impl <T,R,C> DataSetTrait<T,R,C> for DataSet<T,R,C>
where   T:From<u8>,
        R:ArrayLength<GenericArray<T,C>>,
        C:ArrayLength<T>
{
    fn new (p: &str) ->  Self{
        use self::walkdir::WalkDir;
        WalkDir::new(p)
            .into_iter()
            .map(|x| x.unwrap().path().to_path_buf())
            .filter(|x| x.is_file())
            .map(|x| { ImgData::new(x)})
            .collect::<DataSet<T,R,C>>()
    } 
    fn get_minibatch (&self)->MiniBatch<T,R,C>{
        use self::rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let mut minibatch:MiniBatch<T,R,C>;
        unsafe{
            use std;
            minibatch = std::mem::uninitialized();
            for i in minibatch.as_mut(){
                *i = rng.choose(&self).unwrap();
            }
            minibatch
        }
    }
}