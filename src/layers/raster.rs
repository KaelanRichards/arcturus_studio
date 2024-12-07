// src/layers/raster.rs
//
// Represents a raster layer storing pixel-based data. In the future, this might
// reference a texture uploaded to the GPU, or a CPU-side buffer of pixels.

use super::{Layer, LayerProperties};

pub struct RasterLayer {
    properties: LayerProperties,
    width: u32,
    height: u32,
    data: Vec<u8>, // RGBA data
}

impl Layer for RasterLayer {
    fn layer_type(&self) -> &'static str {
        "raster"
    }

    fn name(&self) -> &str {
        &self.properties.name
    }

    fn set_name(&mut self, name: String) {
        self.properties.name = name;
    }

    fn visible(&self) -> bool {
        self.properties.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.properties.visible = visible;
    }

    fn opacity(&self) -> f32 {
        self.properties.opacity
    }

    fn set_opacity(&mut self, opacity: f32) {
        self.properties.opacity = opacity.clamp(0.0, 1.0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl RasterLayer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            properties: LayerProperties {
                name: "New Raster Layer".to_string(),
                visible: true,
                opacity: 1.0,
            },
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            Some([
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
            ])
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            self.data[index..index + 4].copy_from_slice(&color);
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
