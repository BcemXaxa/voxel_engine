use crate::modules::math::{
    angle::Angle,
    cg::{Orientation, Translation},
    mat::{Mat4x4, MatMult},
    quaternion::Quaternion,
    vec::{CrossProd, Vec3, VecAdd, VecMult, VecNorm, VecSub},
};

pub trait Camera {
    fn view_matrix(&self) -> Mat4x4 {
        self.rotation_matrix().mult(self.translation_matrix())
    }
    fn translation_matrix(&self) -> Mat4x4;
    fn rotation_matrix(&self) -> Mat4x4;
}

pub struct OrientedCamera {
    pub pos: Vec3,
    pub orientation: Quaternion,
}

impl Camera for OrientedCamera {
    fn translation_matrix(&self) -> Mat4x4 {
        self.pos.mult(-1.0).translation_matrix()
    }
    fn rotation_matrix(&self) -> Mat4x4 {
        self.orientation.rotation_matrix()
    }
}

impl OrientedCamera {
    pub fn local_move(&mut self, vec: Vec3) {
        let vec = [vec[0], vec[1], vec[2], 1.0];
        let vec = self.rotation_matrix().mult(vec);
        let vec = [vec[0], vec[1], vec[2]];
        self.pos = self.pos.add(vec);
    }
    pub fn local_rotate(&mut self, delta: [f32; 2]) {
        let delta = delta.div(8.0);
        self.orientation = Quaternion::from([
            0.0.into(),
            Angle::from_deg(delta[1]),
            Angle::from_deg(delta[0]),
        ]) * self.orientation;
    }
    pub fn local_roll(&mut self, delta: f32) {
        self.orientation =
            Quaternion::from([Angle::from_deg(delta), 0.0.into(), 0.0.into()]) * self.orientation;
    }
}

pub struct TrackingCamera {
    pub pos: Vec3,
    pub target: Vec3,
}

impl Camera for TrackingCamera {
    fn translation_matrix(&self) -> Mat4x4 {
        self.pos.mult(-1.0).translation_matrix()
    }
    fn rotation_matrix(&self) -> Mat4x4 {
        const UP: Vec3 = [0.0, 0.0, 1.0];

        let y = self.target.sub(self.pos).norm();
        let x = y.cross(UP).norm();
        let z = x.cross(y);

        [
            [x[0], x[1], x[2], 0.0],
            [y[0], y[1], y[2], 0.0],
            [z[0], z[1], z[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}

impl TrackingCamera {
    pub fn local_move(&mut self, vec: Vec3) {
        let vec = [vec[0], vec[1], vec[2], 1.0];
        let vec = self.rotation_matrix().mult(vec);
        let vec = [vec[0], vec[1], vec[2]];
        self.pos = self.pos.add(vec);
    }
}
