extern crate ndarray;
extern crate num;
extern crate rand;

type SomLayers = ndarray::Array3<f32>;


pub trait SomLayersTrait {
    fn layer(&self, idx: usize) -> self::ndarray::ArrayView2<f32>;
    fn layer_mut(&mut self, idx: usize) -> self::ndarray::ArrayViewMut2<f32>;
}

impl SomLayersTrait for SomLayers {
    fn layer(&self, idx: usize) -> self::ndarray::ArrayView2<f32> {
        use ndarray::Axis;
        self.subview(Axis(0), idx)
    }
    fn layer_mut(&mut self, idx: usize) -> self::ndarray::ArrayViewMut2<f32> {
        use ndarray::Axis;
        self.subview_mut(Axis(0), idx)
    }
}

pub struct Csom {
    pub som_layers: SomLayers,
    pub fully_connected_layers: Vec<ndarray::Array2<f32>>,
}

impl Csom {
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        use csom::rnd::{rand_0_1, rand_0_255};
        let som_layers = Csom::new_layer_rand((5, 256, 9), rng, rand_0_255).unwrap();
        let fully_connected_layers = vec![
            Csom::new_layer_rand((16, 16), rng, rand_0_1).unwrap(),
            Csom::new_layer_rand((16, 16), rng, rand_0_1).unwrap(),
            Csom::new_layer_rand((16, 10), rng, rand_0_1).unwrap(),
            Csom::new_layer_rand((10, 10), rng, rand_0_1).unwrap(),
        ];
        Csom {
            som_layers: som_layers,
            fully_connected_layers: fully_connected_layers,
        }
    }
    fn new_layer_rand<T, E>(
        shape: E,
        rng: &mut rand::ThreadRng,
        rand_func: fn(&mut rand::ThreadRng) -> T,
    ) -> Result<ndarray::Array<T, <E as ndarray::IntoDimension>::Dim>, self::ndarray::ShapeError>
    where
        T: self::num::cast::FromPrimitive,
        E: self::ndarray::IntoDimension,
    {
        use ndarray::{Array, Dimension};
        let shape = shape.into_dimension();
        let num: usize = shape.slice().iter().fold(1, |s, x| s * x);
        Array::from_iter((0..num).map(|_| rand_func(rng))).into_shape(shape)
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
