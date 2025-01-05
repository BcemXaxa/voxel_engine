//##########################//
//###### DECLARATION #######//
//##########################//

pub type VecN<const N: usize> = [f32; N];
pub type Vec2 = VecN<2>;
pub type Vec3 = VecN<3>;
pub type Vec4 = VecN<4>;

//##########################//
//######### TRAITS #########//
//##########################//

pub trait VecDefault {
    fn zero() -> Self;
    fn basis() -> Self;
    fn fill(val: f32) -> Self;
}
pub trait VecAdd {
    fn add(self, other: Self) -> Self;
}

pub trait VecSub {
    fn sub(self, other: Self) -> Self;
}

pub trait VecMult: Sized {
    fn mult(self, k: f32) -> Self;
    fn div(self, k: f32) -> Self {
        if k.is_normal() {
            self.mult(1.0 / k)
        } else {
            self
        }
    }
}
pub trait VecLen {
    fn len(&self) -> f32 {
        self.len2().sqrt()
    }
    // len squared
    fn len2(&self) -> f32;
}
pub trait VecNorm: VecLen + VecMult {
    fn norm(self) -> Self {
        let len = self.len();
        self.div(len)
    }
}

pub trait DotProd {
    fn dot(self, other: Self) -> f32;
}

pub trait CrossProd {
    fn cross(self, other: Self) -> Self;
}

//##########################//
//##### IMPLEMENTATION #####//
//##########################//

impl<const N: usize> VecDefault for VecN<N> {
    fn zero() -> Self {
        [0.0; N]
    }

    fn basis() -> Self {
        [1.0; N]
    }

    fn fill(val: f32) -> Self {
        [val; N]
    }
}

impl<const N: usize> VecAdd for VecN<N> {
    fn add(self, other: Self) -> Self {
        let mut out = VecN::zero();
        self.into_iter()
            .zip(other)
            .enumerate()
            .for_each(|(i, (left, right))| out[i] = left + right);
        out
    }
}

impl<const N: usize> VecSub for VecN<N> {
    fn sub(self, other: Self) -> Self {
        let mut out = VecN::zero();
        self.into_iter()
            .zip(other)
            .enumerate()
            .for_each(|(i, (left, right))| out[i] = left - right);
        out
    }
}

impl<const N: usize> VecMult for VecN<N> {
    fn mult(mut self, k: f32) -> Self {
        self.iter_mut().for_each(|comp| *comp *= k);
        self
    }
}

impl<const N: usize> VecLen for VecN<N> {
    fn len2(&self) -> f32 {
        self.into_iter().map(|comp| comp * comp).sum()
    }
}

impl<const N: usize> VecNorm for VecN<N> {}

impl CrossProd for Vec3 {
    fn cross(self, other: Self) -> Self {
        [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ]
    }
}

impl<const N: usize> DotProd for VecN<N> {
    fn dot(self, other: Self) -> f32 {
        self.into_iter()
            .zip(other)
            .map(|(left, right)| left * right)
            .sum()
    }
}

//##########################//
//######### TESTS ##########//
//##########################//

#[cfg(test)]
mod vec_tests {
    use std::f32::EPSILON;
    use crate::modules::math::vec::*;

    #[test]
    fn test_norm_len() {
        let pos: Vec2 = [10.0, 0.0];
        assert!((pos.norm().len() - 1.0).abs() <= EPSILON);

        let pos: Vec2 = [789.178, 999999.1];
        assert!((pos.norm().len() - 1.0).abs() <= EPSILON);

        let pos: Vec2 = [-141.26, -1512.15625];
        assert!((pos.norm().len() - 1.0).abs() <= EPSILON);

        let pos: Vec2 = [0.0, 0.0];
        assert!((pos.norm().len() - 0.0).abs() <= EPSILON);

        let pos: Vec3 = [-141.26, -1512.15625, 5250.2523];
        assert!((pos.norm().len() - 1.0).abs() <= EPSILON);

        let pos: Vec4 = [-141.26, -1512.15625, 5250.2523, -2345.4];
        assert!((pos.norm().len() - 1.0).abs() <= EPSILON);
    }

    #[test]
    fn test_norm() {
        let mut pos1: Vec2 = [54.0, -74.0];
        let pos2: Vec2 = pos1.norm();
        assert_eq!(pos1, [54.0, -74.0]);
        pos1 = pos1.norm();
        assert_eq!(pos1, pos2);
        let pos3: Vec2 = pos1;
        assert_eq!(pos2, pos3);
    }

    #[test]
    fn test_dot_prod() {
        let vec1: Vec3 = [10.0, -67.0, 23.0];
        let vec2: Vec3 = [8.0, -1.5, -5.0];

        let dot = 10.0 * 8.0 + 1.5 * 67.0 - 5.0 * 23.0;
        assert_eq!(dot, vec1.dot(vec2));
    }

    #[test]
    fn test_cross_prod() {
        let vec1: Vec3 = [8.0, 0.0, 0.0];
        let vec2: Vec3 = [0.0, 0.0, -1.5];
        let cross = vec1.cross(vec2).norm();
        assert_eq!([0.0, 1.0, 0.0], cross);
    }

    #[test]
    fn test_add_sub() {
        let arr1 = Vec2::basis();
        let arr2 = Vec2::basis();

        assert_eq!(arr1.add(arr2), Vec2::basis().mult(2.0));
        assert_eq!(arr1.sub(arr2), Vec2::zero())
    }
}
