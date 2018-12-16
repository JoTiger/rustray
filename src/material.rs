use hit::HitRecord;
use math::schlick;
use ray::Ray;
use sample::random_in_unit_sphere;
use vec3::dot;
use vec3::reflect;
use vec3::refract;
use vec3::unit_vector;
use vec3::Vec3f;

extern crate rand;
use rand::{thread_rng, Rng};

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
    pub fuzz: f32,
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
            a: rec.p,
            b: reflected + self.fuzz * random_in_unit_sphere(),
        };
        *attenuation = self.albedo;
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3f,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal;
        let reflected = reflect(r_in.direction(), &rec.normal);
        let ni_over_nt: f32;
        *attenuation = Vec3f { e: [1.0, 1.0, 1.0] };
        let mut refracted = Vec3f::default();
        let reflect_prob: f32;
        let cosine: f32;
        if dot(r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot(r_in.direction(), &rec.normal) / r_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(r_in.direction(), &rec.normal) / r_in.direction().length();
        }
        if refract(
            r_in.direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            *scattered = Ray {
                a: rec.p,
                b: reflected,
            };
            reflect_prob = 1.0
        }
        if thread_rng().gen_range(0.0, 1.0) < reflect_prob {
            *scattered = Ray {
                a: rec.p,
                b: reflected,
            };
        } else {
            *scattered = Ray {
                a: rec.p,
                b: refracted,
            };
        }
        return true;
    }
}
