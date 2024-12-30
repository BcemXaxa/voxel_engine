use crate::math::{mat::Mat4x4, vec::Vec3};

pub trait Camera {
    fn view_matrix() -> Mat4x4;
}

type Angle = f32;

pub struct RotorCamera {
    pos: Vec3,

}

pub struct DirectionalCamera {
    pos: Vec3,
    target: Vec3,
}