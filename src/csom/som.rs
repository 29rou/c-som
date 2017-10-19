extern crate itertools;
extern crate ndarray;
extern crate rand;
extern crate typenum;

pub type SomLayer = ndarray::Array3<f32>;

pub trait SomLayerTrait {
    fn new(rng: &mut rand::ThreadRng) -> Result<SomLayer, self::ndarray::ShapeError>;
}

impl SomLayerTrait for SomLayer {
    fn new(rng: &mut rand::ThreadRng) -> Result<SomLayer, self::ndarray::ShapeError> {
        let randoms: Vec<f32> = (0..(256 * 9 * 5))
            .map(|_| ::csom::rnd::rand_0_255(rng))
            .collect();
        println!("test3");
        let t = self::ndarray::Array3::from_shape_vec((5, 256, 9), randoms);
        println!("{:?}", t.unwrap().dim());
        let randoms: Vec<f32> = (0..(256 * 9))
            .map(|_| ::csom::rnd::rand_0_255(rng))
            .collect();
        let t = self::ndarray::Array3::from_shape_vec((5, 256, 9), randoms);
        println!("4");
        t
    }
}

/*pub trait SomLayersTrait {
    fn new(rng: &mut rand::ThreadRng) -> Self;
}

impl SomLayersTrait for SomLayers {
    fn new(rng: &mut rand::ThreadRng) -> Self {
        unsafe {
            let mut somlayers: SomLayers = ::std::mem::uninitialized();
            (0..5).for_each(|x| {
                SomLayer::new(rng)
                    .map_err(|err| err.to_string())
                    .map(|r| somlayers[x] = r)
                    .unwrap();
            });
            somlayers
        }
    }
}*/
