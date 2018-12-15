use hit::HitRecord;
use ray::Ray;
use vec3::Vec3f;
use sample::random_in_unit_sphere;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3f,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambert {
    pub albedo: Vec3f,
}

impl Material for Lambert {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3f,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray {
          a : rec.p,
          b : target - rec.p
        };
        *attenuation = self.albedo;
        return true;
    }
}
