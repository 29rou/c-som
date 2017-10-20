extern crate num;
extern crate rand;
/*#[macro_export]

macro_rules! rand_0_255{
    ($rng:ident) =>
    {
        {
        use rand::distributions::{Normal, IndependentSample};
        Normal::new(113.0, 26.5).ind_sample($rng)
        }
    }
}

macro_rules! rand_0_1{
    ($rng:ident ) =>
    {
        {
        use rand::distributions::{Normal, IndependentSample};
        Normal::new(0.5, 0.109).ind_sample($rng)
        }
    }
}*/

pub(in csom) fn rand_0_255<T>(rng: &mut rand::ThreadRng) -> T
where
    T: self::num::cast::FromPrimitive,
{
    use self::rand::distributions::{IndependentSample, Normal};
    num::cast::FromPrimitive::from_f64(Normal::new(113.0, 26.5).ind_sample(rng)).unwrap()
}

pub(in csom) fn rand_0_1<T>(rng: &mut rand::ThreadRng) -> T
where
    T: self::num::cast::FromPrimitive,
{
    use self::rand::distributions::{IndependentSample, Normal};
    num::cast::FromPrimitive::from_f64(Normal::new(0.5, 0.109).ind_sample(rng)).unwrap()
}
