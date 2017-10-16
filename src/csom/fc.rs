extern crate itertools;
extern crate rand;

pub struct FullyConnectedLayers {
    pub layer1: [[f32; 16]; 16],
    layer2: [[f32; 16]; 16],
    layer3: [[f32; 10]; 16],
    layer4: [[f32; 10]; 10],
}

impl FullyConnectedLayers {
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        use self::itertools::Itertools;
        let layer1: [[f32; 16]; 16] = unsafe {
            let mut layer: [[f32; 16]; 16] = ::std::mem::uninitialized();
            (0..layer.len())
                .cartesian_product(0..layer[0].len())
                .for_each(|(y, x)| layer[y][x] = rand_0_255!(rng));
            layer
        };
        let layer2: [[f32; 16]; 16] = unsafe {
            let mut layer: [[f32; 16]; 16] = ::std::mem::uninitialized();
            (0..layer.len())
                .cartesian_product(0..layer[0].len())
                .for_each(|(y, x)| layer[y][x] = rand_0_255!(rng));
            layer
        };
        let layer3: [[f32; 10]; 16] = unsafe {
            let mut layer: [[f32; 10]; 16] = ::std::mem::uninitialized();
            (0..layer.len())
                .cartesian_product(0..layer[0].len())
                .for_each(|(y, x)| layer[y][x] = rand_0_255!(rng));
            layer
        };
        let layer4: [[f32; 10]; 10] = unsafe {
            let mut layer: [[f32; 10]; 10] = ::std::mem::uninitialized();
            (0..layer.len())
                .cartesian_product(0..layer[0].len())
                .for_each(|(y, x)| layer[y][x] = rand_0_255!(rng));
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
