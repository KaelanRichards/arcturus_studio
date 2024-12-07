// src/layers/mod.rs
//
// Manages different layer types. Layers form the core of Arcturus Studio's
// editing paradigm. Initially, we have raster, vector, and a 3D scene layer.

use std::any::Any;

pub mod raster;
pub mod vector;
pub mod scene3d;

pub use raster::RasterLayer;
pub use vector::VectorLayer;
pub use scene3d::Scene3DLayer;

pub trait Layer: Any {
    fn layer_type(&self) -> &'static str;
    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);
    fn visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn opacity(&self) -> f32;
    fn set_opacity(&mut self, opacity: f32);
    fn as_any(&self) -> &dyn Any;
}

#[derive(Default)]
pub struct LayerProperties {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
}
