extern crate ndarray;
extern crate num;
extern crate rand;

pub(in csom) struct SomLayers<T, D>(ndarray::Array<Cell<T>, D>);

#[derive(Debug)]
pub(in csom) struct Cell<T> {
    label: u8,
    data: ndarray::Array1<T>,
}

impl<T, D> SomLayers<T, D>
where
    T: self::num::cast::FromPrimitive,
    D: self::ndarray::Dimension,
{
    pub fn new<E>(
        shape: E,
        rng: &mut rand::ThreadRng,
        rand_func: fn(&mut rand::ThreadRng) -> T,
    ) -> Self
    where
        E: self::ndarray::IntoDimension,
        D:
    {
        use self::ndarray::{Array, Dimension};
        let shape = shape.into_dimension();
        let num: usize = shape.slice().iter().fold(1, |s, x| s * x);
        let init_iter = (0..num).map(|_| -> Cell<T> { Cell::new(rng, rand_func) });
        SomLayers(Array::from_iter(init_iter).into_shape(shape).unwrap())
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
