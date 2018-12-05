use ray::Ray;
use vec3::Vec3f;

pub struct Camera {
  origin: Vec3f,
  lower_left_corner: Vec3f,
  horizontal: Vec3f,
  vertical: Vec3f,
}

impl Default for Camera {
  fn default() -> Camera {
    Camera {
      origin: Default::default(),
      lower_left_corner: Vec3f::new(-2.0, -1.5, -1.0),
      horizontal: Vec3f::new(4.0, 0.0, 0.0),
      vertical: Vec3f::new(0.0, 3.0, 0.0),
    }
  }
}

impl Camera {
  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    Ray {
      a: self.origin,
      b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
    }
  }
}
