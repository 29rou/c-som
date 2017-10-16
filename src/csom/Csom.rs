extern crate rand;

pub struct Csom {
    pub som_layers: ::csom::som::SomLayers,
    pub fully_connected_layers: ::csom::fc::FullyConnectedLayers,
}

impl Csom {
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        let som_layers = ::csom::som::SomLayersTrait::new(rng);
        let fully_connected_layers = ::csom::fc::FullyConnectedLayers::new(rng);
        Csom {
            som_layers: som_layers,
            fully_connected_layers: fully_connected_layers,
        }
    }
}
