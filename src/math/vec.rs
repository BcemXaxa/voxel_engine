pub type VecN<const N: usize> = [f32; N];
pub type Vec2 = VecN<2>;
pub type Vec3 = VecN<3>;
pub type Vec4 = VecN<4>;

trait DotProd {
    fn dot(&self, other: &Self) -> f32;
}

impl DotProd for [f32] {
    fn dot(&self, other: &Self) -> f32 {
        self.into_iter()
            .enumerate()
            .map(|(i, this)| this * other[i])
            .sum()
    }
}

trait CrossProd {
    fn cross(&self, other: &Self) -> Self;
}

impl CrossProd for Vec3 {
    fn cross(&self, other: &Self) -> Self {
        [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ]
    }
}

trait Norm<const N: usize>: Sized + Clone {
    fn norm(&mut self);
    fn normed(&self) -> Self {
        let mut new = self.clone();
        new.norm();
        new
    }
}

impl<const N: usize> Norm<N> for [f32; N] {
    fn norm(&mut self) {
        let v_len = self.v_len();
        if v_len.is_normal() {
            self.into_iter().for_each(|comp| *comp /= v_len);
        }
    }
}

trait VecLen {
    fn v_len(&self) -> f32 {
        self.v_len2().sqrt()
    }
    // len squared
    fn v_len2(&self) -> f32;
}

impl VecLen for [f32] {
    fn v_len2(&self) -> f32 {
        self.into_iter().map(|comp| comp * comp).sum()
    }
}

mod vec_tests {
    use std::f32::EPSILON;

    use crate::math::vec::{DotProd, Norm, Vec3, Vec4, VecLen};

    use super::{CrossProd, Vec2};

    #[test]
    fn test_norm_len() {
        let pos: Vec2 = [10.0, 0.0];
        assert!((pos.normed().v_len() - 1.0).abs() <= EPSILON);

        let pos: Vec2 = [789.178, 999999.1];
        assert!((pos.normed().v_len() - 1.0).abs() <= EPSILON);

        let pos: Vec2 = [-141.26, -1512.15625];
        assert!((pos.normed().v_len() - 1.0).abs() <= EPSILON);

        let pos: Vec2 = [0.0, 0.0];
        assert!((pos.normed().v_len() - 0.0).abs() <= EPSILON);

        let pos: Vec3 = [-141.26, -1512.15625, 5250.2523];
        assert!((pos.normed().v_len() - 1.0).abs() <= EPSILON);

        let pos: Vec4 = [-141.26, -1512.15625, 5250.2523, -2345.4];
        assert!((pos.normed().v_len() - 1.0).abs() <= EPSILON);
    }

    #[test]
    fn test_norm() {
        let mut pos1: Vec2 = [54.0, -74.0];
        let pos2: Vec2 = pos1.normed();
        assert_eq!(pos1, [54.0, -74.0]);
        pos1.norm();
        assert_eq!(pos1, pos2);
        let pos3: Vec2 = pos1.clone();
        assert_eq!(pos2, pos3);
    }

    #[test]
    fn test_dot_prod() {
        let vec1: Vec3 = [10.0, -67.0, 23.0];
        let vec2: Vec3 = [8.0, -1.5, -5.0];

        let dot = 10.0 * 8.0 + 1.5 * 67.0 - 5.0 * 23.0;
        assert_eq!(dot, vec1.dot(&vec2));
    }

    #[test]
    fn test_cross_prod() {
        let vec1: Vec3 = [8.0, 0.0, 0.0];
        let vec2: Vec3 = [0.0, 0.0, -1.5];
        let cross = vec1.cross(&vec2).normed();
        assert_eq!([0.0, 1.0, 0.0], cross);
    }
}
