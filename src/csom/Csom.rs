extern crate ndarray;
extern crate num;
extern crate rand;

pub struct Csom {
    pub som_layers: ::std::sync::RwLock<ndarray::Array3<f32>>,
    pub fully_connected_layers: ::std::sync::RwLock<Vec<ndarray::Array2<f32>>>,
}

impl Csom {
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        let som_layers = Csom::new_layer_init_by_rand((5, 256, 9), rng).unwrap();
        let fully_connected_layers: Vec<ndarray::Array2<f32>> = vec![
            Csom::new_layer_init_by_rand((16, 16), rng).unwrap(),
            Csom::new_layer_init_by_rand((16, 16), rng).unwrap(),
            Csom::new_layer_init_by_rand((16, 10), rng).unwrap(),
            Csom::new_layer_init_by_rand((10, 10), rng).unwrap(),
        ];
        Csom {
            som_layers: ::std::sync::RwLock::new(som_layers),
            fully_connected_layers: ::std::sync::RwLock::new(fully_connected_layers),
        }
    }
    fn new_layer_init_by_rand<T, E>(
        shape: E,
        rng: &mut rand::ThreadRng,
    ) -> Result<ndarray::Array<T, <E as ndarray::IntoDimension>::Dim>, self::ndarray::ShapeError>
    where
        T: self::num::cast::FromPrimitive,
        E: self::ndarray::IntoDimension,
    {
        use ndarray::{Array, Dimension};
        use csom::rnd::rand_0_255;
        let shape = shape.into_dimension();
        let num: usize = shape.slice().iter().fold(1, |s, x| s * x);
        Array::from_iter((0..num).map(|_| rand_0_255(rng))).into_shape(shape)
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
