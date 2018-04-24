#![feature(core_intrinsics)]

#[macro_use]
extern crate c_som;
extern crate rand;
extern crate typenum;

fn main() {
    use self::typenum::{U0, U1, U2, U3, U4, U5, U6, U10, U16, U17, U20, U32, U100, U10000};
    use c_som::generic_array_math;
    use rand::distributions::Normal;
    use c_som::shape::{ListToVecTrait, Len, Prod};
    use self::typenum::Unsigned;
    let normal = Normal::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    type ShapeTest = shp![U10, U10, U10];
    println!("{:?}", unsafe { std::intrinsics::type_name::<ShapeTest>() });
    let s = ShapeTest::list_to_vec();
    let d = Len::<ShapeTest>::to_usize();
    let t = Prod::<ShapeTest>::to_usize();
    println!("{:?}\n{:?}\n{:?}\n", s, d, t);
    let test = &generic_array_math::MathArrayBase::<f32, ShapeTest>::new_rnd(&normal, &mut rng);
    println!("{:?}\n", test);
    let test2 = &generic_array_math::MathArrayBase::<f32, ShapeTest>::new_rnd(&normal, &mut rng);
    println!("{:?}\n", test2);
    let test3 = test + test2;
    println!("{:?}\n", test3);
}