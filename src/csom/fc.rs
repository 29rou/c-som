extern crate rand;

pub struct fully_connected_layers {
    layer1: [[f32; 16]; 16],
    layer2: [[f32; 16]; 16],
    layer3: [[f32; 10]; 16],
    layer4: [[f32; 10]; 10],
}

impl fully_connected_layers {
    fn new(rng: &mut rand::ThreadRng) -> Self {
        unsafe {
            let fully_connected_layers: fully_connected_layers = ::std::mem::uninitialized();
            fully_connected_layers
        }
    }
}
