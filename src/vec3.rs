use std::f32;
use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone)]
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
    return (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt();
  }
}

pub fn unit_vector(v: &Vec3f) -> Vec3f {
  let vl = v.length();
  Vec3f {
    e: [v.x() / vl, v.y() / vl, v.z() / vl],
  }
}

impl Add<Vec3f> for Vec3f {
  type Output = Vec3f;

  fn add(self, other: Vec3f) -> Vec3f {
    Vec3f {
      e: [
        self.e[0] + other.e[0],
        self.e[1] + other.e[1],
        self.e[2] + other.e[2],
      ],
    }
  }
}

impl<'a, 'b> Add<&'a Vec3f> for &'b Vec3f {
  type Output = Vec3f;

  fn add(self, other: &'a Vec3f) -> Vec3f {
    Vec3f {
      e: [
        self.e[0] + other.e[0],
        self.e[1] + other.e[1],
        self.e[2] + other.e[2],
      ],
    }
  }
}

impl Mul<f32> for Vec3f {
  type Output = Vec3f;

  fn mul(self, other: f32) -> Vec3f {
    Vec3f {
      e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
    }
  }
}

trait MulVec3f {
  fn mul(self, lhs: &Vec3f) -> Vec3f;
}

impl MulVec3f for f32 {

  fn mul(self, other: &Vec3f) -> Vec3f {
    Vec3f {
      e: [self * other.e[0], self * other.e[1], self * other.e[2]]
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
