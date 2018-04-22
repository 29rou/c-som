#![feature(core_intrinsics)]

extern crate rand;
extern crate typenum;
#[macro_use]
extern crate c_som;

use self::typenum::{U0, U1, U2, U3, U5, U6};
use c_som::generic_array_math;
use c_som::type_compute::{ListTrait};
use rand::distributions::Normal;

fn main() {
    let normal = Normal::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    pub type ShapeTest = shp![U6,U5,U3];
    let test = &generic_array_math::MathArrayBase::<f32, ShapeTest>::new_rnd(&normal, &mut rng);
    let test2 = &generic_array_math::MathArrayBase::<f32, ShapeTest>::new_rnd(&normal, &mut rng);
    let test3 = test + test2;
    println!("{:?},\n{:?},\n{:?}\n", test, test2, test3);;
    use self::ShapeTrait;
    let s = ShapeTest::shape_to_array();
    let d = ShapeTest::dim_to_usize();
    let t = ShapeTest::total_to_usize();
    //let a = arr![];
    //type tst = shp![];
    println!("{:?}\n{:?}\n{:?}", s, d, t);
    println!("{:?}", unsafe { std::intrinsics::type_name::<ShapeTest>() });
}