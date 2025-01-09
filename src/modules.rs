mod interface;
pub mod renderer;
mod shaders;
pub mod window;

mod math {
    pub mod cg;
    pub mod mat;
    pub mod quaternion;
    pub mod vec;
    pub mod angle;
}

pub mod logic {
    pub mod camera;
    mod chunk;
    mod chunk_mesher;
    mod chunk_render;
    pub mod controller;
    mod light;
    mod render_controller;
    mod scene;
    mod voxel;
    mod key_input;
}

mod utility {
    pub mod benchmark;
    pub mod for_multi;
    pub mod framerate;
}
