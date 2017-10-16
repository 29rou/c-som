extern crate rand;

pub struct Csom {
    som_layers: ::csom::som::SomLayers,
    fully_connected_layers: ::csom::fc::fully_connected_layers,
}

impl Csom {
    pub fn new(rng: &mut rand::ThreadRng) -> Self {
        let csom: Csom = unsafe {
            let csom: Csom = ::std::mem::uninitialized();
            csom
        };
        csom
    }
}
