use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_added() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 0.0, y: 2.0, z: 1.0};
        assert_eq!(a + b, Vec3{x: 1.0, y: 4.0, z: 4.0});
    }

    #[test]
    fn can_be_subtracted() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 0.0, y: 2.0, z: 1.0};
        assert_eq!(a - b, Vec3{x: 1.0, y: 0.0, z: 2.0});
    }

    #[test]
    fn can_be_multiplied() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 0.0, y: 2.0, z: 1.0};
        assert_eq!(a * b, Vec3{x: 0.0, y: 4.0, z: 3.0});
    }

    #[test]
    fn can_be_multiplied_by_scalar() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = 5.0;
        assert_eq!(a * b, Vec3{x: 5.0, y: 10.0, z: 15.0});
    }

    #[test]
    fn can_be_divided() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 0.5, y: 2.0, z: 1.0};
        assert_eq!(a / b, Vec3{x: 2.0, y: 1.0, z: 3.0});
    }

    #[test]
    fn can_be_divided_by_scalar() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = 10.0;
        assert_eq!(a / b, Vec3{x: 0.1, y: 0.2, z: 0.3});
    }

    #[test]
    fn can_be_negated() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(-a, Vec3{x: -1.0, y: -2.0, z: -3.0});
    }
}
