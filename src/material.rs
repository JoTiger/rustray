use hit::HitRecord;
use ray::Ray;
use sample::random_in_unit_sphere;
use vec3::Vec3f;
use vec3::reflect;
use vec3::dot;
use vec3::unit_vector;

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
            a: rec.p,
            b: target - rec.p,
        };
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    pub albedo: Vec3f,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3f,
        scattered: &mut Ray,
    ) -> bool {

        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        *scattered = Ray {
            a : rec.p,
            b : reflected
        };
        *attenuation = self.albedo;
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}
