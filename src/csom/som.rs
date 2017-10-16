extern crate itertools;
extern crate rand;

pub type SomLayer = [[f32; 9]; 255];
pub type SomLayers = [SomLayer; 5];

trait SomLayerTrait {
    fn new(rng: &mut rand::ThreadRng) -> Self;
}

impl SomLayerTrait for SomLayer {
    fn new(rng: &mut rand::ThreadRng) -> Self {
        use self::itertools::Itertools;
        unsafe {
            let mut somlayer: SomLayer = ::std::mem::uninitialized();
            (0..somlayer.len())
                .cartesian_product(0..somlayer[0].len())
                .for_each(|(y, x)| somlayer[y][x] = rand_0_255!(rng));
            somlayer
        }
    }
}

pub trait SomLayersTrait {
    fn new(rng: &mut rand::ThreadRng) -> Self;
}

impl SomLayersTrait for SomLayers {
    fn new(rng: &mut rand::ThreadRng) -> Self {
        unsafe {
            let mut somlayers: SomLayers = ::std::mem::uninitialized();
            (0..5).for_each(|x| somlayers[x] = SomLayerTrait::new(rng));
            somlayers
        }
    }
}
