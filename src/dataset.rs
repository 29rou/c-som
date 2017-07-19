extern crate rand;
extern crate walkdir;
extern crate generic_array;
use std;
use imgdata::ImgData;
use MiniBatch;
use self::generic_array::{ArrayLength,GenericArray};

pub type DataSet  = std::vec::Vec<ImgData>;


pub trait DataSetTrait 
{
    fn new(p:&str)->Self;
}

impl  DataSetTrait for DataSet
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
            .collect::<DataSet>()
        /*WalkDir::new(p)
            .into_iter()
            .map(|x| x.unwrap().path().to_path_buf())
            .filter(|x| x.is_file())
            .map(|x| ImgData::new(x))
            .collect::<DataSet<T,R,C>>()*/
    } 
}