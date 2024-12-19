use crate::math::vec::Vec3;

type Angle = f32;

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov: Angle,
}