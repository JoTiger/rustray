use std::f32;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Copy, Clone, Default)]
pub struct Vec3f {
    pub e: [f32; 3],
}

impl Vec3f {
    pub fn x(&self) -> f32 {
        return self.e[0];
    }
    pub fn y(&self) -> f32 {
        return self.e[1];
    }
    pub fn z(&self) -> f32 {
        return self.e[2];
    }
    pub fn r(&self) -> f32 {
        return self.e[0];
    }
    pub fn g(&self) -> f32 {
        return self.e[1];
    }
    pub fn b(&self) -> f32 {
        return self.e[2];
    }

    pub fn length(&self) -> f32 {
        dot(self, self).sqrt()
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { e: [x, y, z] }
    }
}

pub fn dot(v1: &Vec3f, v2: &Vec3f) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub fn cross(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
    Vec3f {
        e: [
            v1.y() * v2.z() - v1.z() * v2.y(),
            v1.z() * v2.x() - v1.x() * v2.z(),
            v1.x() * v2.y() - v1.y() * v2.x(),
        ],
    }
}

pub fn unit_vector(v: &Vec3f) -> Vec3f {
    let vl = v.length();
    Vec3f {
        e: [v.x() / vl, v.y() / vl, v.z() / vl],
    }
}

pub fn refract(v: &Vec3f, n: &Vec3f, ni_over_nt: f32, refracted: &mut Vec3f) -> bool {
    let uv = unit_vector(v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n;
        true
    } else {
        false
    }
}

pub fn reflect(v: &Vec3f, n: &Vec3f) -> Vec3f {
    v - 2.0 * dot(v, n) * n
}

impl Add<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn add(self, other: Vec3f) -> Vec3f {
        &self + &other
    }
}

impl<'a, 'b> Add<&'a Vec3f> for &'b Vec3f {
    type Output = Vec3f;

    fn add(self, other: &'a Vec3f) -> Vec3f {
        Vec3f {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Sub<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        &self - &other
    }
}

impl Sub<Vec3f> for &Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        *self - other
    }
}

impl<'a, 'b> Sub<&'a Vec3f> for &'b Vec3f {
    type Output = Vec3f;

    fn sub(self, other: &'a Vec3f) -> Vec3f {
        Vec3f {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, other: f32) -> Vec3f {
        Vec3f {
            e: [self.x() * other, self.y() * other, self.z() * other],
        }
    }
}

impl Mul<&Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, other: &Vec3f) -> Vec3f {
        Vec3f {
            e: [self * other.x(), self * other.y(), self * other.z()],
        }
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, other: Vec3f) -> Vec3f {
        other * self
    }
}

impl Mul<Vec3f> for &Vec3f {
    type Output = Vec3f;

    fn mul(self, other: Vec3f) -> Vec3f {
        *self * other
    }
}

impl Mul<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f {
            e: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ],
        }
    }
}

impl Div<f32> for Vec3f {
    type Output = Vec3f;

    fn div(self, other: f32) -> Vec3f {
        let rcp = 1.0 / other;
        self * rcp
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Vec3f {
        Vec3f {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

#[test]
fn test_add() {
    let v1 = Vec3f { e: [0.0, 1.0, 2.0] };
    let v2 = Vec3f { e: [1.0, 2.0, 3.0] };
    let v3 = v1 + v2;
    assert_eq!(v3.x(), 1.0);
    assert_eq!(v3.y(), 3.0);
    assert_eq!(v3.z(), 5.0);

    let v4 = &v1 + &v2;
    assert_eq!(v4.x(), 1.0);
    assert_eq!(v4.y(), 3.0);
    assert_eq!(v4.z(), 5.0);
}

#[test]
fn test_length() {
    let v = Vec3f {
        e: [3.0, 4.0, 12.0],
    };
    assert_eq!(v.length(), 13.0);
}
