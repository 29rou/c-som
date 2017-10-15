extern crate image;
extern crate itertools;
extern crate rand;
extern crate walkdir;

use std;

pub struct CifarDataset {
    pub count: usize,
    pub labels: Vec<String>,
    pub dataset: Vec<CifarImage>,
}

pub struct CifarImage {
    pub label: u8,
    pub image: image::DynamicImage,
}

struct CifarFilePaths {
    meta_data_paths: Vec<std::path::PathBuf>,
    binary_data_paths: Vec<std::path::PathBuf>,
}

impl CifarDataset {
    pub fn new(path: &std::path::Path) -> Result<Self, String> {
        let CifarFilePaths {
            meta_data_paths,
            binary_data_paths,
        } = CifarDataset::get_file_paths(path).ok_or("Can't Find Files!!")?;
        let meta_data: Vec<String> =
            CifarDataset::get_meta_data(&meta_data_paths).map_err(|err| err.to_string())?;
        println!("{:?}", meta_data);
        let byte_datas: Vec<Vec<u8>> =
            CifarDataset::get_byte_datas(&binary_data_paths).map_err(|err| err.to_string())?;
        let cifar_images: Vec<CifarImage> = CifarDataset::get_images(byte_datas).unwrap();
        let count: usize = cifar_images.len();
        let cifar_dataset = CifarDataset {
            labels: meta_data,
            dataset: cifar_images,
            count: count,
        };
        Ok(cifar_dataset)
    }
    fn get_file_paths(path: &std::path::Path) -> Option<CifarFilePaths> {
        let paths = &walkdir::WalkDir::new(path)
            .into_iter()
            .flat_map(|x| x.map(|x| x.path().to_path_buf()))
            .filter(|x| x.is_file())
            .collect::<Vec<std::path::PathBuf>>();
        let meta_data_paths = CifarDataset::find_file_paths_by_ext(paths, "txt");
        let binary_data_paths = CifarDataset::find_file_paths_by_ext(paths, "bin");
        match (meta_data_paths, binary_data_paths) {
            (Some(meta), Some(binary)) => Some(CifarFilePaths {
                meta_data_paths: meta,
                binary_data_paths: binary,
            }),
            _ => None,
        }
    }
    fn find_file_paths_by_ext(
        paths: &[std::path::PathBuf],
        ext: &str,
    ) -> Option<Vec<std::path::PathBuf>> {
        let fpaths: Vec<std::path::PathBuf> = paths
            .iter()
            .filter(|path| -> bool {
                match path.extension() {
                    Some(p) => p == ext,
                    None => false,
                }
            })
            .cloned()
            .collect();
        if fpaths.is_empty() {
            None
        } else {
            Some(fpaths)
        }
    }
    fn get_meta_data(paths: &[std::path::PathBuf]) -> Result<Vec<String>, std::io::Error> {
        use std::io::Read;
        use self::itertools::Itertools;
        paths
            .iter()
            .map(|meta_path| -> Result<String, std::io::Error> {
                std::fs::File::open(meta_path).and_then(|mut file| {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).map(|_| contents)
                })
            })
            .map(|lines| -> Result<Vec<String>, std::io::Error> {
                lines.map(|l| -> Vec<String> {
                    l.lines()
                        .filter(|x| !x.is_empty())
                        .map(|x| x.into())
                        .collect_vec()
                })
            })
            .collect::<Result<Vec<Vec<String>>, std::io::Error>>()
            .map(|v| v.concat())
    }
    fn get_byte_datas(paths: &[std::path::PathBuf]) -> Result<Vec<Vec<u8>>, std::io::Error> {
        use std::io::{BufReader, Read};
        use self::itertools::Itertools;
        paths
            .iter()
            .map(|file_path| -> Result<Vec<u8>, std::io::Error> {
                std::fs::File::open(file_path).and_then(|file| {
                    let mut byte_data: Vec<u8> = Vec::new();
                    BufReader::new(file)
                        .read_to_end(&mut byte_data)
                        .map(|_| byte_data)
                })
            })
            .map(|byte_data| -> Result<Vec<Vec<u8>>, std::io::Error> {
                byte_data.map(|b| -> Vec<Vec<u8>> {
                    b.chunks(3073)
                        .map(|byte_img| -> Vec<u8> { byte_img.to_vec() })
                        .collect_vec()
                })
            })
            .collect::<Result<Vec<Vec<Vec<u8>>>, std::io::Error>>()
            .map(|v| v.concat())
    }
    fn get_images(byte_datas: Vec<Vec<u8>>) -> Result<Vec<CifarImage>, String> {
        byte_datas
            .into_iter()
            .map(|byte_img| {
                std::thread::spawn(move || CifarImage::new(&byte_img))
            })
            .map(|img| -> Result<CifarImage, String> {
                img.join().map_err(|_| "thread panicked".to_string())
            })
            .collect::<Result<Vec<CifarImage>, String>>()
    }
    pub fn for_test_get_image_by_save(&self) -> Result<(), String> {
        use self::rand::{thread_rng, Rng};
        let fout = &mut std::fs::File::create(&std::path::Path::new("test.jpeg"))
            .map_err(|err| err.to_string())?;
        let nth: &usize = &thread_rng().gen_range(0, self.count);
        let data: &CifarImage = &self.dataset[*nth];
        data.image
            .resize(500, 500, image::FilterType::Lanczos3)
            .save(fout, image::JPEG)
            .map_err(|err| err.to_string())?;
        println!("No.{} {}", nth, self.labels[data.label as usize]);
        Ok(())
    }
}

impl CifarImage {
    fn new(bytes: &[u8]) -> Self {
        use std::io::Read;
        use std::mem;
        use self::image::GenericImage;
        use self::itertools::multizip;
        use self::itertools::Itertools;
        let bytes: &mut &[u8] = &mut bytes.as_ref();
        let label: u8 = unsafe {
            let label: &mut [u8; 1] = &mut mem::uninitialized();
            bytes.read_exact(label).expect("Can't Read Label!!");
            *label.get_unchecked(0)
        };
        let img = unsafe {
            let mut img = image::DynamicImage::new_rgb8(32, 32);
            let red: &mut [u8; 1024] = &mut mem::uninitialized();
            let green: &mut [u8; 1024] = &mut mem::uninitialized();
            let blue: &mut [u8; 1024] = &mut mem::uninitialized();
            bytes.read_exact(red).expect("Can't Read Red!!");
            bytes.read_exact(green).expect("Can't Read Green!!");
            bytes.read_exact(blue).expect("Can't Read Blue!!");
            multizip((
                (0..32).cartesian_product(0..32),
                red.iter(),
                green.iter(),
                blue.iter(),
            )).for_each(|((y, x), r, g, b)| {
                let mut pixel: image::Rgba<u8> = mem::uninitialized();
                pixel.data = [*r, *g, *b, 255];
                img.unsafe_put_pixel(x, y, pixel);
            });
            img
        };
        CifarImage {
            label: label,
            image: img,
        }
    }
}
