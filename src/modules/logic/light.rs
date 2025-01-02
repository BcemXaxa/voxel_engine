use crate::modules::math::vec::Vec3;

pub struct Light {
    direction: Vec3,
    color: [f32; 3],
    intensity: f32,
}