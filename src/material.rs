use hit::HitRecord;
use ray::Ray;
use vec3::Vec3f;

pub trait Material {
  fn scatter(
    &self,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3f,
    scattered: &mut Ray,
  ) -> bool;
}

pub struct Lambert {}

// impl Material for Lambert {

// }
