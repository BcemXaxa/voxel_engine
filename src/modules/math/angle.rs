use std::{
    f32::consts::PI,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Angle(f32);

impl Angle {
    pub fn from_rad(rad: f32) -> Angle {
        Angle(rad.rem_euclid(2.0 * PI))
    }
    pub fn from_deg(deg: f32) -> Angle {
        Angle::from_rad(deg.to_radians())
    }

    pub fn rad(&self) -> f32 {
        self.0
    }
    pub fn deg(&self) -> f32 {
        self.0.to_degrees()
    }
}

impl From<f32> for Angle {
    fn from(rad: f32) -> Self {
        Angle::from_rad(rad)
    }
}
impl Into<f32> for Angle {
    fn into(self) -> f32 {
        self.rad()
    }
}

impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Angle::from_rad(self.rad() + rhs.rad())
    }
}
impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Angle::from_rad(self.rad() - rhs.rad())
    }
}
impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl Mul<f32> for Angle {
    type Output = Angle;
    fn mul(self, scalar: f32) -> Self {
        Angle::from_rad(self.rad() * scalar)
    }
}
impl MulAssign<f32> for Angle {
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar
    }
}
impl Div<f32> for Angle {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Angle::from_rad(self.rad() / scalar)
    }
}
impl DivAssign<f32> for Angle {
    fn div_assign(&mut self, scalar: f32) {
        *self = *self / scalar
    }
}
impl Neg for Angle {
    type Output = Self;
    fn neg(self) -> Self {
        Angle::from_rad(-self.rad())
    }
}
impl Angle {
    pub fn sin(&self) -> f32 {
        self.rad().sin()
    }
    pub fn cos(&self) -> f32 {
        self.rad().cos()
    }
    pub fn sin_cos(&self) -> (f32, f32) {
        self.rad().sin_cos()
    }
    pub fn tan(&self) -> f32 {
        self.rad().tan()
    }
    pub fn is_reflex(&self) -> bool {
        self.rad() > PI
    }
    pub fn is_sharp(&self) -> bool {
        self.rad() < PI / 2.0
    }
    pub fn reflect(self) -> Angle {
        -self
    }
}

#[cfg(test)]
mod angle_tests {
    use super::Angle;

    #[test]
    fn test_display() {
        let a = Angle::from_deg(90.0);
        println!("{a:?}")
    }

    #[test]
    fn test_add() {
        let a = Angle::from_deg(90.0);
        let b = Angle::from_deg(80.0);
        assert_eq!(Angle::from_deg(170.0), a + b);

        let a = Angle::from_deg(780.0);
        let b = Angle::from_deg(-140.0);
        assert_eq!(Angle::from_deg(280.0), a + b);
    }
}
