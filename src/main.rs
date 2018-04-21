#![feature(core_intrinsics)]

mod som;
pub mod generic_array_math;
#[macro_use]
pub mod type_compute;

extern crate num_traits;
extern crate rand;
#[macro_use]
extern crate typenum;
#[macro_use]
extern crate generic_array;


fn main() {
    use typenum::{U0, U1, U2, U3, U5, U6};
    use type_compute::{Shape,Nill,ShapeTrait};
    let p: i32 = num_traits::one();
    let p: i32 = num_traits::one::<i32>() + num_traits::one::<i32>();
    println!("{}", p);
    println!("Hello, World!!");
    let mut vec1 = vec![0, 0, 0, 0, 0];
    let mut vec2 = vec![0, 1, 2, 3, 4];
    let vec3 = vec1.iter().zip(&mut vec2)
        .map(|(a, b)| { *a + *b }).collect::<Vec<_>>();
    let array1 = [[0; 5]; 3];
    let array2 = [0, 1, 2, 3, 4];
    let mut rng = rand::thread_rng();
    let input2 = [0.1, 1.0, 2.0];
    let input = arr![f32; 1, 2, 3];
    let som1d = som::Som1d::<f64, U3, U5>::new(&mut rng);
    use som::Som;
    use rand::distributions::Normal;
    let normal = Normal::new(0.0, 1.0);
    let input: som::Array1D<U3, f64> = som::Array1D::new(&normal, &mut rng);
    som1d.train(&input);
    println!("{:?}", vec3);
    use typenum::Unsigned;
    type r = typenum::Quot<U6, U3>;
    println!("{}", r::to_usize());
    println!("{}", input2.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap());
    pub type ShapeTest = shp![U6,U5,U3];
    let test = &generic_array_math::MathArrayBase::<f32, ShapeTest>::new_rnd(&normal, &mut rng);
    let test2 = &generic_array_math::MathArrayBase::<f32, ShapeTest>::new_rnd(&normal, &mut rng);
    let test3 = test + test2;
    println!("{:?},\n{:?},\n{:?}\n", test, test2, test3);;
    let s = <ShapeTest as ShapeTrait>::shape_to_array();
    let d = <ShapeTest as ShapeTrait>::dim_to_usize();
    let t = <ShapeTest as ShapeTrait>::total_to_usize();
    //let a = arr![];
    //type tst = shp![];
    println!("{:?}\n{:?}\n{:?}", s, d, t);
    println!("{:?}", unsafe { std::intrinsics::type_name::<ShapeTest>() });
}