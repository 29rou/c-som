extern crate image;
extern crate itertools;
extern crate rand;
extern crate regex;
extern crate walkdir;

pub struct CifarDataset {
    pub labels: Vec<String>,
    pub train_count: usize,
    pub test_count: usize,
    pub train_dataset: Vec<::cifar::image::CifarImage>,
    pub test_dataset: Vec<::cifar::image::CifarImage>,
}

struct CifarFilePaths {
    meta_data_paths: Vec<::std::path::PathBuf>,
    train_data_paths: Vec<::std::path::PathBuf>,
    test_data_paths: Vec<::std::path::PathBuf>,
}

impl CifarDataset {
    pub fn new(path: &::std::path::Path) -> Result<Self, String> {
        let CifarFilePaths {
            meta_data_paths,
            train_data_paths,
            test_data_paths,
        } = CifarDataset::get_file_paths(path)?;
        let meta_data: Vec<String> =
            CifarDataset::get_meta_data(&meta_data_paths).map_err(|err| err.to_string())?;
        let train_images: Vec<::cifar::image::CifarImage> = {
            let byte_datas: Vec<Vec<u8>> =
                CifarDataset::get_byte_datas(&train_data_paths).map_err(|err| err.to_string())?;
            CifarDataset::get_images(byte_datas)?
        };
        let test_images: Vec<::cifar::image::CifarImage> = {
            let byte_datas: Vec<Vec<u8>> =
                CifarDataset::get_byte_datas(&test_data_paths).map_err(|err| err.to_string())?;
            CifarDataset::get_images(byte_datas)?
        };
        let cifar_dataset = CifarDataset {
            labels: meta_data,
            train_count: train_images.len() as usize,
            train_dataset: train_images,
            test_count: test_images.len() as usize,
            test_dataset: test_images,
        };
        Ok(cifar_dataset)
    }
    fn get_file_paths(path: &::std::path::Path) -> Result<CifarFilePaths, String> {
        use self::regex::Regex;
        let paths = &walkdir::WalkDir::new(path)
            .into_iter()
            .flat_map(|x| x.map(|x| x.path().to_path_buf()))
            .filter(|x| x.is_file())
            .collect::<Vec<::std::path::PathBuf>>();
        let train_data_path_re = Regex::new("data_batch_[1-5].bin").map_err(|err| err.to_string())?;
        let test_data_path_re = Regex::new("test_batch.bin").map_err(|err| err.to_string())?;
        let cifar_file_paths = CifarFilePaths {
            meta_data_paths: CifarDataset::get_meta_data_paths(paths)?,
            train_data_paths: CifarDataset::get_paths_regex(paths, &train_data_path_re)?,
            test_data_paths: CifarDataset::get_paths_regex(paths, &test_data_path_re)?,
        };
        Ok(cifar_file_paths)
    }
    fn get_meta_data_paths(
        paths: &[::std::path::PathBuf],
    ) -> Result<Vec<::std::path::PathBuf>, String> {
        use std::path::{Path, PathBuf};
        let meta_data_file_name = Path::new("batches.meta.txt");
        let fpaths: Vec<PathBuf> = paths
            .into_iter()
            .filter(|path| {
                path.file_name()
                    .map(|file_name| file_name == meta_data_file_name)
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        if fpaths.is_empty() {
            Err("Can't Find Meta Data Files!!".to_string())
        } else {
            Ok(fpaths)
        }
    }
    fn get_paths_regex(
        paths: &[::std::path::PathBuf],
        re: &self::regex::Regex,
    ) -> Result<Vec<::std::path::PathBuf>, String> {
        use std::path::PathBuf;
        let fpaths: Vec<PathBuf> = paths
            .iter()
            .filter(|path| {
                path.file_name()
                    .map(|file_name| {
                        let file_name = file_name.to_string_lossy();
                        re.is_match(file_name.as_ref())
                    })
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        if fpaths.is_empty() {
            Err("Can't Find Train Data Files!!".to_string())
        } else {
            Ok(fpaths)
        }
    }
    fn get_meta_data(paths: &[::std::path::PathBuf]) -> Result<Vec<String>, ::std::io::Error> {
        use std::io::Read;
        use self::itertools::Itertools;
        paths
            .iter()
            .map(|meta_path| -> Result<String, ::std::io::Error> {
                ::std::fs::File::open(meta_path).and_then(|mut file| {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).map(|_| contents)
                })
            })
            .map(|lines| -> Result<Vec<String>, ::std::io::Error> {
                lines.map(|l| -> Vec<String> {
                    l.lines()
                        .filter(|x| !x.is_empty())
                        .map(|x| x.into())
                        .collect_vec()
                })
            })
            .collect::<Result<Vec<Vec<String>>, ::std::io::Error>>()
            .map(|v| v.concat())
    }
    fn get_byte_datas(paths: &[::std::path::PathBuf]) -> Result<Vec<Vec<u8>>, ::std::io::Error> {
        use std::io::{BufReader, Read};
        use self::itertools::Itertools;
        paths
            .iter()
            .map(|file_path| -> Result<Vec<u8>, ::std::io::Error> {
                ::std::fs::File::open(file_path).and_then(|file| {
                    let mut byte_data: Vec<u8> = Vec::new();
                    BufReader::new(file)
                        .read_to_end(&mut byte_data)
                        .map(|_| byte_data)
                })
            })
            .map(|byte_data| -> Result<Vec<Vec<u8>>, ::std::io::Error> {
                byte_data.map(|b| -> Vec<Vec<u8>> {
                    b.chunks(3073)
                        .map(|byte_img| -> Vec<u8> { byte_img.to_vec() })
                        .collect_vec()
                })
            })
            .collect::<Result<Vec<Vec<Vec<u8>>>, ::std::io::Error>>()
            .map(|v| v.concat())
    }
    fn get_images(byte_datas: Vec<Vec<u8>>) -> Result<Vec<::cifar::image::CifarImage>, String> {
        byte_datas
            .into_iter()
            .map(|byte_img| {
                ::std::thread::spawn(move || ::cifar::image::CifarImage::new(&byte_img))
            })
            .map(|img| -> Result<::cifar::image::CifarImage, String> {
                img.join()
                    .map_err(|_| "thread panicked".to_string())
                    .map(|content| content.map_err(|err| err.to_string()))?
            })
            .collect::<Result<Vec<::cifar::image::CifarImage>, String>>()
    }
    pub fn for_test_get_image_from_train_save(
        &self,
        rng: &mut rand::ThreadRng,
    ) -> Result<(), String> {
        use self::rand::Rng;
        let fout = &mut ::std::fs::File::create(&::std::path::Path::new("train.jpeg"))
            .map_err(|err| err.to_string())?;
        let nth: &usize = &rng.gen_range(0, self.train_count);
        let data: &::cifar::image::CifarImage = &self.train_dataset[*nth];
        data.image
            .resize(500, 500, image::FilterType::Lanczos3)
            .save(fout, image::JPEG)
            .map_err(|err| err.to_string())?;
        println!("From Train No.{} {}", nth, self.labels[data.label as usize]);
        Ok(())
    }
    pub fn for_test_get_image_from_test_save(
        &self,
        rng: &mut rand::ThreadRng,
    ) -> Result<(), String> {
        use self::rand::Rng;
        let fout = &mut ::std::fs::File::create(&::std::path::Path::new("test.jpeg"))
            .map_err(|err| err.to_string())?;
        let nth: &usize = &rng.gen_range(0, self.test_count);
        let data: &::cifar::image::CifarImage = &self.test_dataset[*nth];
        data.image
            .resize(500, 500, image::FilterType::Lanczos3)
            .save(fout, image::JPEG)
            .map_err(|err| err.to_string())?;
        println!("From test No.{} {}", nth, self.labels[data.label as usize]);
        Ok(())
    }
}
