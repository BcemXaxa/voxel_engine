use crate::modules::math::{
    cg::{Orientation, Translation},
    mat::{Mat4x4, MatMult},
    quaternion::Quaternion,
    vec::{CrossProd, Vec3, VecMult, VecNorm, VecSub},
};

pub trait Camera {
    fn view_matrix(&self) -> Mat4x4;
}

pub struct OrientedCamera {
    pub pos: Vec3,
    pub orientation: Quaternion,
}

impl Camera for OrientedCamera {
    fn view_matrix(&self) -> Mat4x4 {
        self.orientation
            .rotation_matrix()
            .mult(self.pos.mult(-1.0).translation_matrix())
    }
}

pub struct TrackingCamera {
    pub pos: Vec3,
    pub target: Vec3,
}

impl Camera for TrackingCamera {
    fn view_matrix(&self) -> Mat4x4 {
        const UP: Vec3 = [0.0, 0.0, 1.0];

        let y = self.target.sub(self.pos).norm();
        let x = y.cross(UP).norm();
        let z = x.cross(y);

        [
            [x[0], x[1], x[2], 0.0],
            [y[0], y[1], y[2], 0.0],
            [z[0], z[1], z[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ].mult(self.pos.mult(-1.0).translation_matrix())
    }
}
