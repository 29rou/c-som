extern crate itertools;
extern crate num;
extern crate rand;

macro_rules!  for_fully_connected_layer_init{
    ($layer:ident : $T:ty ,$rng:ident) => {
        {
        use self::itertools::Itertools;
        (0..$layer.len())
                .cartesian_product(0..$layer[0].len())
               .for_each(|(y, x)| $layer[y][x] = rand_0_1!($rng) as $T);
        }
    };
}



pub struct FullyConnectedLayers {
    pub layer1: [[f32; 16]; 16],
    pub layer2: [[f32; 16]; 16],
    pub layer3: [[f32; 10]; 16],
    pub layer4: [[f32; 10]; 10],
}

impl FullyConnectedLayers {
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        use std::mem::uninitialized;
        let layer1 = unsafe {
            let mut layer: [[f32; 16]; 16] = uninitialized();
            for_fully_connected_layer_init!(layer: f32, rng);
            layer
        };
        let layer2 = unsafe {
            let mut layer: [[f32; 16]; 16] = uninitialized();
            for_fully_connected_layer_init!(layer: f32, rng);
            layer
        };
        let layer3 = unsafe {
            let mut layer: [[f32; 10]; 16] = uninitialized();
            for_fully_connected_layer_init!(layer: f32, rng);
            layer
        };
        let layer4 = unsafe {
            let mut layer: [[f32; 10]; 10] = uninitialized();
            for_fully_connected_layer_init!(layer: f32, rng);
            layer
        };
        FullyConnectedLayers {
            layer1: layer1,
            layer2: layer2,
            layer3: layer3,
            layer4: layer4,
        }
    }
}
