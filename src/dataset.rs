extern crate rand;
extern crate walkdir;
extern crate generic_array;
use std;
use imgdata::ImgData;
use MiniBatch;
use self::generic_array::{ArrayLength,GenericArray};

pub type DataSet<T,R,C>  = std::vec::Vec<ImgData<T,R,C>>;

pub trait DataSetTrait <T,R,C>
where   T:From<u8>,
        R:ArrayLength<GenericArray<T,C>>,
        C:ArrayLength<T>
{
    fn new(p:&str)->Self;
    fn get_minibatch(&self,n:usize)->MiniBatch<T,R,C>;
}

impl <T,R,C> DataSetTrait<T,R,C> for DataSet<T,R,C>
where   T:From<u8>+'static,
        R:ArrayLength<GenericArray<T,C>>+'static,
        C:ArrayLength<T>+'static,
        <R as generic_array::ArrayLength<generic_array::GenericArray<T, C>>>::ArrayType: std::marker::Send
{
    fn new (p: &str) ->  Self{
        use std::thread;
        use self::walkdir::WalkDir;
        WalkDir::new(p)
            .into_iter()
            .map(|x| x.unwrap().path().to_path_buf())
            .filter(|x| x.is_file())
            .map(|x| thread::spawn(||ImgData::new(x)))
            .map(|x| x.join().expect("Thread Error!"))
            .collect::<DataSet<T,R,C>>()
        /*WalkDir::new(p)
            .into_iter()
            .map(|x| x.unwrap().path().to_path_buf())
            .filter(|x| x.is_file())
            .map(|x| ImgData::new(x))
            .collect::<DataSet<T,R,C>>()*/
    } 
    fn get_minibatch (&self,n:usize)->MiniBatch<T,R,C>{
        use self::rand::Rng;
        let rng = &mut rand::thread_rng();
        (1..n)
            .filter_map(|_| rng.choose(&self))
            .collect()
    }
}