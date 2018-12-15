use hit::HitRecord;
use hit::Hitable;
use material::Material;
use ray::Ray;
use vec3::dot;
use vec3::Vec3f;

#[derive(Default)]
pub struct Sphere<'a> {
    pub center: Vec3f,
    pub radius: f32,
    pub mat: Option<&'a Material>,
}

impl<'a> Sphere<'a> {
    pub fn new(c: &Vec3f, r: f32, m : Option<&'a Material>) -> Sphere<'a> {
        Sphere {
            center: *c,
            radius: r,
            mat: m,
        }
    }
}

impl<'a> Hitable<'a> for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let oc = ray.origin() - &self.center;
        let a = dot(ray.direction(), ray.direction());
        let b = dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
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

    fn material(&self) -> Option<&'a Material> {
        return self.mat;
    }
}
