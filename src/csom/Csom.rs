extern crate rand;

pub struct Csom {
    pub som_layers: ::csom::som::SomLayers,
    pub fully_connected_layers: ::csom::fc::FullyConnectedLayers,
}

impl Csom {
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        use csom::som::SomLayersTrait;
        use csom::fc::FullyConnectedLayers;
        Csom {
            som_layers: SomLayersTrait::new(rng),
            fully_connected_layers: FullyConnectedLayers::new(rng),
        }
    }
    pub fn train(
        &self,
        cifar_dataset: &::cifar::dataset::CifarDataset,
        rng: &mut rand::ThreadRng,
    ) -> &Self {
        self
    }
    pub fn test(
        &self,
        cifar_test_dataset: &::cifar::dataset::CifarDataset,
        rng: &mut rand::ThreadRng,
    ) -> &Self {
        self
    }
    pub fn save(&self) -> Result<&Self, String> {
        Ok(self)
    }
}
