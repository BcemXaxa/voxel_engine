use std::ops::{Mul, MulAssign};

use super::{
    angle::Angle,
    cg::Orientation,
    mat::Mat4x4,
    vec::{Vec4, VecNorm},
};

pub type EulerAngles = [Angle; 3];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl Orientation for Quaternion {
    fn rotation_matrix(&self) -> Mat4x4 {
        let Quaternion { w, x, y, z } = self;
        [
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - z * w),
                2.0 * (x * z + y * w),
                0.0,
            ],
            [
                2.0 * (x * y + z * w),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - x * w),
                0.0,
            ],
            [
                2.0 * (x * z - y * w),
                2.0 * (y * z + x * w),
                1.0 - 2.0 * (x * x + y * y),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Self) -> Self {
        let Quaternion {
            w: w1,
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Quaternion {
            w: w2,
            x: x2,
            y: y2,
            z: z2,
        } = other;
        Quaternion {
            w: w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
            x: w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
            y: w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
            z: w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2,
        }
    }
}
impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Quaternion) {
        *self = *self * rhs
    }
}

impl From<EulerAngles> for Quaternion {
    fn from(euler: EulerAngles) -> Self {
        let [pitch, roll, yaw] = euler;
        let (sin_p, cos_p) = (pitch / 2.0).sin_cos();
        let (sin_r, cos_r) = (roll / 2.0).sin_cos();
        let (sin_y, cos_y) = (yaw / 2.0).sin_cos();
        Quaternion {
            w: cos_r * cos_p * cos_y + sin_r * sin_p * sin_y,
            x: sin_r * cos_p * cos_y - cos_r * sin_p * sin_y,
            y: cos_r * sin_p * cos_y + sin_r * cos_p * sin_y,
            z: cos_r * cos_p * sin_y - sin_r * sin_p * cos_y,
        }
    }
}

impl From<Vec4> for Quaternion {
    fn from(vec: Vec4) -> Self {
        let [w, x, y, z] = vec.norm();
        Quaternion { w, x, y, z }
    }
}

#[cfg(test)]
mod quaternion_tests {
    use crate::modules::math::{angle::Angle, cg::Orientation};

    use super::Quaternion;

    #[test]
    fn test_quat_mult() {
        let q1: Quaternion = [0.0, 0.0, 1.0, 0.0].into();
        let q2 = Quaternion::default();
        let q3 = q1 * q2;
        assert_eq!(q1, q3)
    }

    #[test]
    fn test_rotate() {
        println!("{:?}, {:?}", Angle::from_deg(-30.0), Angle::from_deg(330.0));
        let q1 = Quaternion::from([
            Angle::from_deg(30.0),
            Angle::from_deg(0.0),
            Angle::from_deg(30.0),
        ]);
        println!("{q1:?}")
    }
}
