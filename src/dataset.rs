extern crate walkdir;
use std;
use imgdata::ImgData;

pub type DataSet = Vec<std::sync::Arc<ImgData>>;

pub trait DataSetTrait {
    fn new(p: &str) -> Self;
}

impl DataSetTrait for DataSet {
    fn new(p: &str) -> Self {
        use self::walkdir::WalkDir;
        WalkDir::new(p)
            .into_iter()
            .flat_map(|x| x)
            .map(|x| x.path().to_path_buf())
            .filter(|x| x.is_file())
            .map(move |x| std::sync::Arc::new(ImgData::new(x)))
            .collect::<DataSet>()
    }
}
