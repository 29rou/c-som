extern crate rand;

pub type SomLayer = [[f32; 9]; 256];

pub type SomLayers = [SomLayer; 5];

trait SomLayerTrait {
    fn new(rng: &mut rand::ThreadRng) -> Self;
}

trait SomLayersTrait {
    fn new(rng: &mut rand::ThreadRng) -> Self;
}
