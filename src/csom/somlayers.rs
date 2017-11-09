extern crate ndarray;
extern crate num;
extern crate rand;

pub(in csom) type SomLayers<T, E> = ndarray::Array<Cell<T>, <E as ndarray::IntoDimension>::Dim>;

#[derive(Debug)]
pub(in csom) struct Cell<T> {
    label: u8,
    data: ndarray::Array1<T>,
}

pub(in csom) trait SomLayersTrait<T, E>
where
    T: self::num::cast::FromPrimitive,
    E: self::ndarray::IntoDimension,
{
    fn new(
        shape: E,
        rng: &mut rand::ThreadRng,
        rand_func: fn(&mut rand::ThreadRng) -> T,
    ) -> ndarray::Array<Cell<T>, <E as ndarray::IntoDimension>::Dim>;
}

impl<T, E> SomLayersTrait<T, E> for SomLayers<T, E>
where
    T: self::num::cast::FromPrimitive,
    E: self::ndarray::IntoDimension,
{
    fn new(
        shape: E,
        rng: &mut rand::ThreadRng,
        rand_func: fn(&mut rand::ThreadRng) -> T,
    ) -> ndarray::Array<Cell<T>, <E as ndarray::IntoDimension>::Dim> {
        use self::ndarray::{Array, Dimension};
        let shape = shape.into_dimension();
        let num: usize = shape.slice().iter().fold(1, |s, x| s * x);
        let init_iter = (0..num).map(|_| -> Cell<T> { Cell::new(rng, rand_func) });
        Array::from_iter(init_iter).into_shape(shape).unwrap()
    }
}

impl<T> Cell<T> {
    fn new(rng: &mut ::rand::ThreadRng, rand_func: fn(&mut rand::ThreadRng) -> T) -> Self
    where
        T: self::num::cast::FromPrimitive,
    {
        use self::ndarray::Array;
        let init_iter = (0..9).map(|_| rand_func(rng));
        let data = Array::from_iter(init_iter);
        Cell {
            label: <u8>::max_value(),
            data: data,
        }
    }
}
