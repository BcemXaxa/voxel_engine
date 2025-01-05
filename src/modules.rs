pub mod renderer;
pub mod window;
mod interface;
mod shaders;

mod math {
    pub mod mat;
    pub mod vec;
    pub mod quaternion;
    pub mod cg;
}
pub mod logic {
    mod scene;
    pub mod camera;
    mod chunk;
    mod light;
    pub mod controller;
    // mod render_controller;
}
mod utility {
    pub mod benchmark;
    pub mod for_multi;
}