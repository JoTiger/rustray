use hit::HitRecord;
use hit::Hitable;
use ray::Ray;
use vec3::dot;
use vec3::Vec3f;

#[derive(Default)]
pub struct Sphere {
  pub center: Vec3f,
  pub radius: f32,
}

impl Sphere {
  pub fn new(c : &Vec3f, r : f32) -> Sphere {
    Sphere{
      center : *c,
      radius : r
    }
  }
}

impl From<(f32, f32, f32, f32)> for Sphere {
  fn from((x, y, z, r) : (f32, f32, f32, f32)) -> Sphere {
    Sphere {
      center : Vec3f::new(x, y, z),
      radius : r
    }
  }
}

impl Hitable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
    let oc = ray.origin() - &self.center;
    let a = dot(ray.direction(), ray.direction());
    let b = dot(&oc, ray.direction());
    let c = dot(&oc, &oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      let mut temp = (-b - ( b * b - a *c).sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = ray.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
      temp = (-b + (b * b - a * c).sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = ray.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
    }
    false
  }
}
