extern crate ndarray;

pub(in csom) type SomLayers = ndarray::Array3<f32>;


pub(in csom) trait SomLayersTrait {
    fn layer(&self, idx: usize) -> self::ndarray::ArrayView2<f32>;
    fn layer_mut(&mut self, idx: usize) -> self::ndarray::ArrayViewMut2<f32>;
}

impl SomLayersTrait for SomLayers {
    fn layer(&self, idx: usize) -> self::ndarray::ArrayView2<f32> {
        use ndarray::Axis;
        self.subview(Axis(0), idx)
    }
    fn layer_mut(&mut self, idx: usize) -> self::ndarray::ArrayViewMut2<f32> {
        use ndarray::Axis;
        self.subview_mut(Axis(0), idx)
    }
}
