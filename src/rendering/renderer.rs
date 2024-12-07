// src/rendering/renderer.rs
//
// The Renderer handles wgpu initialization and draws the scene.
// Future steps:
// - Initialize wgpu (device, queue, surface).
// - Provide methods to render the current Document's layers.

use crate::layers::{Layer, RasterLayer, VectorLayer, Scene3DLayer};
use crate::layers::vector::VectorShape;

pub struct Renderer {
    width: u32,
    height: u32,
    framebuffer: Vec<u8>, // RGBA
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            framebuffer: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        for pixel in self.framebuffer.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }

    pub fn render_layer(&mut self, layer: &dyn Layer) {
        if !layer.visible() {
            return;
        }

        match layer.layer_type() {
            "raster" => {
                if let Some(raster) = layer.as_any().downcast_ref::<RasterLayer>() {
                    self.render_raster_layer(raster);
                }
            }
            "vector" => {
                if let Some(vector) = layer.as_any().downcast_ref::<VectorLayer>() {
                    self.render_vector_layer(vector);
                }
            }
            "scene3d" => {
                if let Some(scene3d) = layer.as_any().downcast_ref::<Scene3DLayer>() {
                    self.render_3d_layer(scene3d);
                }
            }
            _ => {}
        }
    }

    fn render_raster_layer(&mut self, layer: &RasterLayer) {
        let (layer_width, layer_height) = layer.dimensions();
        let opacity = layer.opacity();

        for y in 0..self.height.min(layer_height) {
            for x in 0..self.width.min(layer_width) {
                if let Some(color) = layer.get_pixel(x, y) {
                    let dest_index = ((y * self.width + x) * 4) as usize;
                    let src_color = [
                        (color[0] as f32 * opacity) as u8,
                        (color[1] as f32 * opacity) as u8,
                        (color[2] as f32 * opacity) as u8,
                        (color[3] as f32 * opacity) as u8,
                    ];
                    self.blend_pixel(dest_index, src_color);
                }
            }
        }
    }

    fn render_vector_layer(&mut self, layer: &VectorLayer) {
        for shape in layer.get_shapes() {
            match shape {
                VectorShape::Rectangle { x, y, width, height, fill_color, .. } => {
                    let x_start = (*x as u32).min(self.width);
                    let y_start = (*y as u32).min(self.height);
                    let x_end = ((*x + *width) as u32).min(self.width);
                    let y_end = ((*y + *height) as u32).min(self.height);
                    
                    for py in y_start..y_end {
                        for px in x_start..x_end {
                            let index = ((py * self.width + px) * 4) as usize;
                            self.blend_pixel(index, *fill_color);
                        }
                    }
                }
                _ => {} // Other shapes not implemented yet
            }
        }
    }

    fn render_3d_layer(&mut self, _layer: &Scene3DLayer) {
        // 3D rendering would require a more sophisticated implementation
        // using a 3D graphics API like wgpu
    }

    fn blend_pixel(&mut self, dest_index: usize, src_color: [u8; 4]) {
        let dest = &mut self.framebuffer[dest_index..dest_index + 4];
        let src_alpha = src_color[3] as f32 / 255.0;
        let dest_alpha = dest[3] as f32 / 255.0;
        
        for i in 0..3 {
            let src = src_color[i] as f32 / 255.0;
            let dst = dest[i] as f32 / 255.0;
            let blended = src * src_alpha + dst * dest_alpha * (1.0 - src_alpha);
            dest[i] = (blended * 255.0) as u8;
        }
        
        let final_alpha = src_alpha + dest_alpha * (1.0 - src_alpha);
        dest[3] = (final_alpha * 255.0) as u8;
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }
}
