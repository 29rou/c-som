extern crate walkdir;
use std;
use imgdata::ImgData;

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
    }
}