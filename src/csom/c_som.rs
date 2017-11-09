extern crate cifar_10_loader;
extern crate ndarray;
extern crate num;
extern crate rand;


pub struct Csom<T> {
    som_layers: super::SomLayers<T, self::ndarray::Dim<[usize; 2]>>,
}

impl<T> Csom<T>
where
    T: self::num::cast::FromPrimitive,
{
    pub fn new(rng: &mut rand::ThreadRng) -> Result<Self, self::ndarray::ShapeError> {
        use super::rnd::rand_0_255;
        use super::SomLayersTrait;
        let som_layers =
            SomLayersTrait::new::<(usize, usize)>((5 as usize, 256 as usize), rng, rand_0_255);
        let csom = Csom {
            som_layers: som_layers,
        };
        Ok(csom)
    }
    pub fn train(
        &self,
        cifar_dataset: &cifar_10_loader::CifarDataset,
        rng: &mut rand::ThreadRng,
    ) -> &Self {
        self
    }
    pub fn test(
        &self,
        cifar_test_dataset: &cifar_10_loader::CifarDataset,
        rng: &mut rand::ThreadRng,
    ) -> &Self {
        self
    }
    pub fn save(&self) -> Result<&Self, String> {
        Ok(self)
    }
}
