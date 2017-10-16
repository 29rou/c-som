extern crate rand;
#[macro_export]
macro_rules! rand_0_255{
    ($x:expr) =>
        {{use rand::Rng;
        let rnd:f32 = $x.gen_range(0.0,255.0);
        rnd
        }}
}
