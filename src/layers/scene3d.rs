// src/layers/scene3d.rs
//
// A 3D scene layer containing meshes, lights, and a camera.
// Eventually, Arcturus Studio will allow placing 3D objects and rendering them
// with wgpu. For now, just a placeholder.

use super::{Layer, LayerProperties};

#[derive(Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

#[derive(Clone, Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    material: Material,
}

#[derive(Clone, Debug)]
pub struct Material {
    diffuse_color: [f32; 4],
    specular_color: [f32; 4],
    shininess: f32,
}

pub struct Scene3DLayer {
    properties: LayerProperties,
    meshes: Vec<Mesh>,
    camera_position: [f32; 3],
    camera_target: [f32; 3],
    camera_up: [f32; 3],
}

impl Layer for Scene3DLayer {
    fn layer_type(&self) -> &'static str {
        "scene3d"
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

impl Scene3DLayer {
    pub fn new() -> Self {
        Self {
            properties: LayerProperties {
                name: "New 3D Scene".to_string(),
                visible: true,
                opacity: 1.0,
            },
            meshes: Vec::new(),
            camera_position: [0.0, 0.0, 5.0],
            camera_target: [0.0, 0.0, 0.0],
            camera_up: [0.0, 1.0, 0.0],
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn remove_mesh(&mut self, index: usize) -> Option<Mesh> {
        if index < self.meshes.len() {
            Some(self.meshes.remove(index))
        } else {
            None
        }
    }

    pub fn get_meshes(&self) -> &[Mesh] {
        &self.meshes
    }

    pub fn get_mesh_mut(&mut self, index: usize) -> Option<&mut Mesh> {
        self.meshes.get_mut(index)
    }

    pub fn set_camera_position(&mut self, position: [f32; 3]) {
        self.camera_position = position;
    }

    pub fn set_camera_target(&mut self, target: [f32; 3]) {
        self.camera_target = target;
    }

    pub fn set_camera_up(&mut self, up: [f32; 3]) {
        self.camera_up = up;
    }

    pub fn get_camera_view(&self) -> ([f32; 3], [f32; 3], [f32; 3]) {
        (self.camera_position, self.camera_target, self.camera_up)
    }
}
