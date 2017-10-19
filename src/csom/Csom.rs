extern crate ndarray;
extern crate num;
extern crate rand;

pub struct Csom {
    pub som_layers: ndarray::Array3<f32>,
    pub fully_connected_layers: Vec<ndarray::Array2<f32>>,
}

impl Csom {
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        use ndarray::Array;
        use csom::rnd::rand_0_255;
        let som_layers = Array::from_iter((0..(5 * 256 * 9)).map(|_| rand_0_255(rng)))
            .into_shape((5, 256, 9))
            .unwrap();
        let fully_connected_layers = vec![
            Csom::new_layer(16, 16, rng),
            Csom::new_layer(16, 16, rng),
            Csom::new_layer(16, 10, rng),
            Csom::new_layer(10, 10, rng),
        ];
        Csom {
            som_layers: som_layers,
            fully_connected_layers: fully_connected_layers,
        }
    }
    fn new_layer<T>(r: usize, c: usize, rng: &mut rand::ThreadRng) -> ndarray::Array2<T>
    where
        T: self::num::cast::FromPrimitive,
    {
        let randoms: Vec<T> = (0..(r * c)).map(|_| ::csom::rnd::rand_0_255(rng)).collect();
        self::ndarray::Array2::from_shape_vec((r, c), randoms).unwrap()
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
