// src/layers/vector.rs
//
// A vector layer that stores shapes and paths. Over time, this might reference
// vector primitives, SVG paths, or be manipulated with vector tools.

use super::{Layer, LayerProperties};

#[derive(Clone, Debug)]
pub enum VectorShape {
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke_width: f32,
        color: [u8; 4],
    },
    Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        stroke_width: f32,
        fill_color: [u8; 4],
        stroke_color: [u8; 4],
    },
    Circle {
        x: f32,
        y: f32,
        radius: f32,
        stroke_width: f32,
        fill_color: [u8; 4],
        stroke_color: [u8; 4],
    },
}

pub struct VectorLayer {
    properties: LayerProperties,
    shapes: Vec<VectorShape>,
}

impl Layer for VectorLayer {
    fn layer_type(&self) -> &'static str {
        "vector"
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

impl VectorLayer {
    pub fn new() -> Self {
        Self {
            properties: LayerProperties {
                name: "New Vector Layer".to_string(),
                visible: true,
                opacity: 1.0,
            },
            shapes: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: VectorShape) {
        self.shapes.push(shape);
    }

    pub fn remove_shape(&mut self, index: usize) -> Option<VectorShape> {
        if index < self.shapes.len() {
            Some(self.shapes.remove(index))
        } else {
            None
        }
    }

    pub fn get_shapes(&self) -> &[VectorShape] {
        &self.shapes
    }

    pub fn get_shape_mut(&mut self, index: usize) -> Option<&mut VectorShape> {
        self.shapes.get_mut(index)
    }
}
