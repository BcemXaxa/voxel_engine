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
mod logic {
    mod scene;
    mod camera;
    mod chunk;
    mod light;
}
mod utility {
    pub mod benchmark;
    pub mod for_multi;
}