use super::{mat::Mat4x4, vec::Vec3};


pub trait Orientation {
    fn rotation_matrix(&self) -> Mat4x4;
}

pub trait Translation {
    fn translation_matrix(&self) -> Mat4x4;
}

impl Translation for Vec3 {
    fn translation_matrix(&self) -> Mat4x4 {
        [
            [1.0, 0.0, 0.0, self[0]],
            [0.0, 1.0, 0.0, self[1]],
            [0.0, 0.0, 1.0, self[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}

pub trait Frustum {
    fn projection_matrix(&self) -> Mat4x4;
}

pub struct PerspectiveFrustum {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub ar: f32,
}

impl Frustum for PerspectiveFrustum {
    fn projection_matrix(&self) -> Mat4x4 {
        let far = self.far;
        let near = self.near;
        let ar = self.ar;
        let t = self.fov.tan();
        [
            [1.0 / (t * ar), 0.0, 0.0, 0.0],
            [0.0, 1.0 / t, 0.0, 0.0],
            [0.0, 0.0, far / (far - near), -near * far / (far - near)],
            [0.0, 0.0, 1.0, 0.0],
        ]
    }
}

pub struct OrthographicFrustum {
    width: f32,
    height: f32,
    near: f32,
    far: f32,
}

impl Frustum for OrthographicFrustum {
    fn projection_matrix(&self) -> Mat4x4 {
        todo!()
    }
}