use ray::Ray;
use vec3::Vec3f;
use material::Material;

pub struct HitRecord<'a> {
  pub t: f32,
  pub p: Vec3f,
  pub normal: Vec3f,
  pub material: Option<&'a Material>
}

impl<'a> Clone for HitRecord<'a> {
    fn clone(&self) -> HitRecord<'a> {
        HitRecord {
          t : self.t,
          p : self.p,
          normal : self.normal,
          material : self.material
        }
    }
}

pub trait Hitable {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
  pub list: Vec<Box<Hitable>>,
}

impl HitableList {
  pub fn size(&self) -> usize {
    self.list.len()
  }
}

impl Hitable for HitableList {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord {
      t: 0.0,
      p: Vec3f { e: [0.0, 0.0, 0.0] },
      normal: Vec3f { e: [0.0, 0.0, 0.0] },
      material : None
    };
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    for hitable in self.list.iter() {
      if hitable.hit(ray, t_min, closest_so_far, &mut temp_rec) {
         hit_anything = true;
        closest_so_far = temp_rec.t;
        *rec = temp_rec.clone();
      }
    }
    return hit_anything;
  }
}
