// src/plugins/ai_integration.rs
//
// Example plugin to integrate AI image generation (via ComfyUI).
// Future steps: send prompts, receive images, add a raster layer with AI results.

use serde::Serialize;

#[derive(Serialize)]
struct AiRequest {
    prompt: String,
}

pub struct AiPlugin;

impl AiPlugin {
    pub fn generate_image(&self, _prompt: &str) {
        // TODO: Use reqwest to call ComfyUI API, decode image, insert into Document as a raster layer.
    }
}
