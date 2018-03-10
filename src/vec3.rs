use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.x * rhs.y,
            y: -(self.x * rhs.z) - self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
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
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 1.0,
        };
        assert_eq!(
            a + b,
            Vec3 {
                x: 1.0,
                y: 4.0,
                z: 4.0,
            }
        );
    }

    #[test]
    fn can_be_subtracted() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 1.0,
        };
        assert_eq!(
            a - b,
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            }
        );
    }

    #[test]
    fn can_be_multiplied() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 1.0,
        };
        assert_eq!(
            a * b,
            Vec3 {
                x: 0.0,
                y: 4.0,
                z: 3.0,
            }
        );
    }

    #[test]
    fn can_be_multiplied_by_scalar() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = 5.0;
        assert_eq!(
            a * b,
            Vec3 {
                x: 5.0,
                y: 10.0,
                z: 15.0,
            }
        );
    }

    #[test]
    fn can_be_divided() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 0.5,
            y: 2.0,
            z: 1.0,
        };
        assert_eq!(
            a / b,
            Vec3 {
                x: 2.0,
                y: 1.0,
                z: 3.0,
            }
        );
    }

    #[test]
    fn can_be_divided_by_scalar() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = 10.0;
        assert_eq!(
            a / b,
            Vec3 {
                x: 0.1,
                y: 0.2,
                z: 0.3,
            }
        );
    }

    #[test]
    fn can_be_negated() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(
            -a,
            Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            }
        );
    }

    #[test]
    fn has_len_squared() {
        let v = Vec3 { x: 2.0, y: 3.0, z: 1.0 };
        assert_eq!(v.len_squared(), 14.0);
    }

    #[test]
    fn has_len() {
        let a = Vec3 { x: 0.5, y: 0.0, z: 0.0 };
        assert_eq!(a.len(), 0.5);
        let b = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!(b.len(), 3f64.sqrt());
    }

    #[test]
    fn can_be_normalized() {
        let mut v = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        v.normalize();
        assert_eq!(v.len(), 1.0);
        assert_eq!(v.x, v.y);
        assert_eq!(v.y, v.z);
        assert_eq!(v.x, v.z);
    }

    #[test]
    fn supports_dot_product() {
        let a = Vec3 { x: 2.0, y: 3.0, z: -2.0 };
        let b = Vec3 { x: -5.0, y: 0.5, z: 10.0 };
        let dot = a.dot(b);
        assert_eq!(dot, -28.5);
    }

    #[test]
    fn supports_cross_product() {
        let a = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        let b = Vec3 { x: -2.0, y: 3.0, z: 0.0 };
        let cross = a.cross(b);
        assert_eq!(cross, Vec3 {
            x: -3.0,
            y: 2.0,
            z: 5.0,
        });
    }
}
