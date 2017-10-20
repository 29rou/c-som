//#[macro_use]
mod rnd;
mod somlayers;
mod c_som;
pub use self::c_som::Csom;
use self::somlayers::{SomLayers, SomLayersTrait};
use self::rnd::{rand_0_1, rand_0_255};
