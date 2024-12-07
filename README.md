Arcturus Studio
Arcturus Studio is an evolving desktop application built with Rust. It aims to provide a flexible and extensible 2D/3D creative environment. The long-term vision is to integrate vector and raster graphics, 3D scenes, and AI-driven image generation—offering a unique blend of traditional content creation tools and cutting-edge generative capabilities, all within a unified interface.

Current Status
Alpha Prototype (Work in Progress)

UI Framework: Iced for a responsive, declarative, and cross-platform Rust GUI.
Basic Functionality:
Displays a main window with dynamic text and a simple counter.
Allows renaming the “project” on the fly, reflecting changes in the window title.
No Complex Features Yet:
No document model integrated.
No layer system.
No rendering of vector/raster or 3D content.
No AI integration yet.
This is a foundation on which to build. The current codebase demonstrates a running window, a reactive UI, and a basic application state management pattern.

Roadmap
The following steps outline the near-term and long-term goals:

Document Model:

Introduce a Document struct holding layers and metadata.
Implement actions to add/remove layers and modify their properties.
2D Layer Support:

Add basic raster and vector layers.
Integrate GPU rendering with wgpu to draw shapes or images in the main viewport.
Provide basic editing tools (selection, transformation, painting).
3D Scene Integration:

Introduce a 3D layer type that holds meshes, camera parameters, and lighting.
Render the 3D scene in real-time, possibly alongside 2D layers.
AI Integration:

Hook up to an AI image generation service (e.g., ComfyUI or Stable Diffusion).
Allow users to input prompts and receive generated images as new raster layers.
Undo/Redo & Command System:

Implement a command pattern to track changes, enabling undo/redo operations.
Integrate with the UI, so users can revert or reapply actions.
Persistence and Assets:

Implement saving/loading documents.
Manage external assets (images, 3D models, vector files).
Plugins & Extensibility:

Enable a plugin architecture so developers can add custom tools, nodes, or AI integrations.
Technical Overview
Language: Rust
UI: Iced (cross-platform, declarative, async-friendly GUI).
Rendering (planned): wgpu for GPU-accelerated 2D/3D compositing.
Integration (planned):
serde, serde_json for serialization.
reqwest for HTTP requests to AI endpoints.
image crate for image loading/decoding.
Potential use of egui or other frameworks if certain views or editor modes require immediate-mode interfaces.
Contributing
Contributions are welcome! Current focus areas include:

Designing and implementing the document model.
Integrating the rendering pipeline for at least basic 2D output.
Discussing and planning the architecture for layer management, undo/redo, and AI integration.
Before contributing, please:

Check open issues and discussions for the current direction.
Follow the project’s coding conventions and style guidelines.
Propose new features in issues or discussions to reach consensus before implementing major changes.
License
Arcturus Studio is released under an open-source license (e.g., MIT or Apache 2.0—add a LICENSE file to clarify). Check the repository’s LICENSE file for specific terms.

Contact and Community
For questions and discussions, open a GitHub issue or join any available community channels listed in the repository.
As the project matures, we may add Discord servers or forums to facilitate in-depth discussions and real-time collaboration.
