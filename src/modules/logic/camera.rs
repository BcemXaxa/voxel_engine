use crate::modules::math::{
    cg::{Orientation, Translation}, mat::{Mat4x4, MatMult}, quaternion::Quaternion, vec::{CrossProd, Vec3, Vec4, VecMult, VecNorm, VecSub}
};

pub trait Camera {
    fn view_matrix(&self) -> Mat4x4;
}

pub struct OrientedCamera {
    pos: Vec3,
    orientation: Quaternion,
}

impl Camera for OrientedCamera {
    fn view_matrix(&self) -> Mat4x4 {
        self.pos
            .mult(-1.0)
            .translation_matrix()
            .mult(self.orientation.rotation_matrix())
    }
}

pub struct TrackingCamera {
    pub pos: Vec3,
    pub target: Vec3,
}

impl Camera for TrackingCamera {
    fn view_matrix(&self) -> Mat4x4 {
        const UP: Vec3 = [0.0, 1.0, 0.0];

        let z = self.target.sub(self.pos).norm();
        let x = z.cross(UP).norm();
        let y = x.cross(z);

        [
            [x[0], x[1], x[2], 0.0],
            [y[0], y[1], y[2], 0.0],
            [z[0], z[1], z[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}