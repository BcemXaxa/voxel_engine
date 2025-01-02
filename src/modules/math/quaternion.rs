use super::{cg::Orientation, mat::Mat4x4, vec::Vec4};

pub type Quaternion = Vec4;

impl Orientation for Quaternion {
    fn rotation_matrix(&self) -> Mat4x4 {
        let w = self[0];
        let x = self[1];
        let y = self[2];
        let z = self[3];
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