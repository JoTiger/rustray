use vec3::Vec3f;
use sphere::Sphere;

extern crate rand;
use rand::{thread_rng, Rng };

pub fn random_in_unit_sphere() -> Vec3f {
  let mut rng = thread_rng();
  let mut p : Vec3f;
  loop 
  {
    p = 2.0 * Vec3f::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)) - Vec3f::new(1.0, 1.0, 1.0);
    if (p.length() < 1.0)
      {
        break;
      }
  }
  p
}