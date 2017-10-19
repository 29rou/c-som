extern crate itertools;
extern crate ndarray;
extern crate rand;
extern crate typenum;

pub type SomLayer = ndarray::Array2<f32>;
pub type SomLayers = [SomLayer; 5];

trait SomLayerTrait {
    fn new(rng: &mut rand::ThreadRng) -> Result<SomLayer, self::ndarray::ShapeError>;
}

impl SomLayerTrait for SomLayer {
    fn new(rng: &mut rand::ThreadRng) -> Result<SomLayer, self::ndarray::ShapeError> {
        let randoms: Vec<f32> = (0..256 * 9).map(|_| ::csom::rnd::rand_0_255(rng)).collect();
        self::ndarray::Array2::from_shape_vec((256, 9), randoms)
    }
}

pub trait SomLayersTrait {
    fn new(rng: &mut rand::ThreadRng) -> Self;
}

impl SomLayersTrait for SomLayers {
    fn new(rng: &mut rand::ThreadRng) -> Self {
        unsafe {
            let mut somlayers: SomLayers = ::std::mem::uninitialized();
            let t = (0..5)
                .map(|x| {
                    SomLayerTrait::new(rng)
                        .map(|r| somlayers[x] = r)
                        .map_err(|err| err.to_string())
                })
                .collect::<Result<Vec<()>, _>>();
            Ok(somlayers)
        }.unwrap()
    }
}
