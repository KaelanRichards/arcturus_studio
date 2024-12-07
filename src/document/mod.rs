// src/document/mod.rs
//
// The Document represents the user's project state in Arcturus Studio.
// It holds layers (raster, vector, 3D), possibly metadata like dimensions,
// color space, and future settings.
//
// As the project evolves, this will:
// - Manage a list of layers.
// - Provide functions for adding/removing layers.
// - Handle saving/loading from disk.

use crate::layers::{Layer, RasterLayer, VectorLayer, Scene3DLayer};

pub struct Document {
    pub name: String,
    pub layers: Vec<Box<dyn Layer>>,
    pub width: u32,
    pub height: u32,
}

impl Document {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        Self {
            name: name.to_string(),
            layers: Vec::new(),
            width,
            height,
        }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn remove_layer(&mut self, index: usize) -> Option<Box<dyn Layer>> {
        if index < self.layers.len() {
            Some(self.layers.remove(index))
        } else {
            None
        }
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn get_layer(&self, index: usize) -> Option<&Box<dyn Layer>> {
        self.layers.get(index)
    }

    pub fn get_layer_mut(&mut self, index: usize) -> Option<&mut Box<dyn Layer>> {
        self.layers.get_mut(index)
    }
}
