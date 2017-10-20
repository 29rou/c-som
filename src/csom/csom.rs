extern crate cifar_10_loader;
extern crate ndarray;
extern crate num;
extern crate rand;


pub struct Csom {
    som_layers: ::csom::somlayers::SomLayers,
    fully_connected_layers: Vec<ndarray::Array2<f32>>,
}

impl Csom {
    pub fn new(rng: &mut rand::ThreadRng) -> Result<Self, self::ndarray::ShapeError> {
        use csom::rnd::{rand_0_1, rand_0_255};
        let som_layers = Csom::new_layer_rand((5, 256, 9), rng, rand_0_255)?;
        let fully_connected_layers = vec![
            Csom::new_layer_rand((16, 16), rng, rand_0_1)?,
            Csom::new_layer_rand((16, 16), rng, rand_0_1)?,
            Csom::new_layer_rand((16, 10), rng, rand_0_1)?,
            Csom::new_layer_rand((10, 10), rng, rand_0_1)?,
        ];
        let csom = Csom {
            som_layers: som_layers,
            fully_connected_layers: fully_connected_layers,
        };
        Ok(csom)
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
    pub fn output(&self) -> Result<(), String> {
        use csom::somlayers::SomLayersTrait;
        use ndarray::Axis;
        println!("{:?}", self.som_layers);
        println!("{:?}", self.som_layers.subview(Axis(0), 0));
        println!("{:?}", self.som_layers.subview(Axis(0), 1));
        println!("{:?}", self.som_layers.subview(Axis(0), 2));
        println!("{:?}", self.som_layers.layer(0));
        println!("{:?}", self.som_layers.shape());
        Ok(())
    }
}
