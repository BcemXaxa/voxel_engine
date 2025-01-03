use crate::modules::math::vec::Vec3;

pub struct Light {
    pub direction: Vec3,
    pub color: [f32; 3],
    pub intensity: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            direction: [0.0, -1.0, 0.0],
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
        }
    }
}
