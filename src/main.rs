#![feature(core_intrinsics)]

extern crate rand;
extern crate typenum;
#[macro_use]
extern crate c_som;

use self::typenum::{U0, U1, U2, U3, U5, U6};
use c_som::generic_array_math;
//use c_som::type_compute::{ShapeTrait};
use rand::distributions::Normal;

fn main() {
    let normal = Normal::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    pub type ShapeTest = shp![U6,U5,U3];
    let test = &generic_array_math::MathArrayBase::<f32, ShapeTest>::new_rnd(&normal, &mut rng);
    let test2 = &generic_array_math::MathArrayBase::<f32, ShapeTest>::new_rnd(&normal, &mut rng);
    let test3 = test + test2;
    println!("{:?},\n{:?},\n{:?}\n", test, test2, test3);
    let t = test3.as_slice();
    use c_som::shape::{ListToVecTrait, Len, Prod};
    use self::typenum::Unsigned;
    let s = ShapeTest::list_to_vec();
    let d = Len::<ShapeTest>::to_usize();
    let t = Prod::<ShapeTest>::to_usize();
    //let a = arr![];
    //type tst = shp![];
    println!("{:?}\n{:?}\n{:?}", s, d, t);
    use c_som::type_compute::{Succ,Zero};
    type Three = Succ<Succ<Succ<Zero>>>;
    type Two = Succ<Succ<Zero>>;
    type Six = <Two as c_som::type_compute::Mul<Three>>::Result;
    println!("{}" ,<Six as c_som::type_compute::ToUsize>::to_usize());
    println!("{:?}", unsafe { std::intrinsics::type_name::<Six>() });
    println!("{:?}", unsafe { std::intrinsics::type_name::<ShapeTest>() });
}