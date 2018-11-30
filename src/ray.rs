use vec3::Vec3f;

pub struct Ray
{
  pub a : Vec3f,
  pub b : Vec3f,
}

impl Ray {
  // pub fn direction(self) -> & Vec3f {
  //   &self.b
  // }
  // pub fn origin(self) -> &'a Vec3f {
  //   &self.a
  // }

  // pub fn direction(self) -> &'a Vec3f {
  //   &self.b
  // }

  pub fn point_at_parameter(self, t : f32) -> Vec3f {
    self.a + self.b * t
  }
}

#[test]
fn test_point_at_parameter() {
  let ray = Ray {
    a : Vec3f {
      e : [0.0, 1.0, 2.0]
    },

    b : Vec3f {
      e : [1.0, 2.0, 3.0]
    }
  };
  let p = ray.point_at_parameter(2.0);
  assert_eq!(p.x(), 2.0);
  assert_eq!(p.y(), 5.0);
  assert_eq!(p.z(), 8.0);
}
