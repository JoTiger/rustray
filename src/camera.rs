use ray::Ray;
use vec3::cross;
use vec3::unit_vector;
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

    pub fn new(lookfrom: Vec3f, lookat: Vec3f, vup: Vec3f, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }
}
