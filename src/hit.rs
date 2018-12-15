use material::Material;
use ray::Ray;
use vec3::Vec3f;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3f,
    pub normal: Vec3f,
    pub mat: Option<&'a Material>,
}

impl<'a> Clone for HitRecord<'a> {
    fn clone(&self) -> HitRecord<'a> {
        HitRecord {
            t: self.t,
            p: self.p,
            normal: self.normal,
            mat: self.mat,
        }
    }
}

pub trait Hitable<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool;
    fn material(&self) -> Option<&'a Material>;
}

pub struct HitableList<'a> {
    pub list: Vec<Box<Hitable<'a> + 'a>>,
}

impl<'a> Hitable<'a> for HitableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let mut temp_rec = HitRecord {
            t: 0.0,
            p: Vec3f { e: [0.0, 0.0, 0.0] },
            normal: Vec3f { e: [0.0, 0.0, 0.0] },
            mat: None,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable in self.list.iter() {
            if hitable.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
                rec.mat = hitable.material();
            }
        }
        return hit_anything;
    }

    fn material(&self) -> Option<&'a Material> {
        None
    }
}
