use ray::Ray;
use sample::random_in_unit_disk;
use vec3::cross;
use vec3::unit_vector;
use vec3::Vec3f;

pub struct Camera {
    origin: Vec3f,
    lower_left_corner: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
    u: Vec3f,
    v: Vec3f,
    w: Vec3f,
    lens_radius: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            origin: Default::default(),
            lower_left_corner: Vec3f::new(-2.0, -1.5, -1.0),
            horizontal: Vec3f::new(4.0, 0.0, 0.0),
            vertical: Vec3f::new(0.0, 3.0, 0.0),
            w: Vec3f::new(0.0, 0.0, 1.0),
            u: Vec3f::new(1.0, 0.0, 0.0),
            v: Vec3f::new(0.0, 1.0, 0.0),
            lens_radius: 1.0,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            a: self.origin + offset,
            b: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }

    pub fn new(
        lookfrom: Vec3f,
        lookat: Vec3f,
        vup: Vec3f,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
        }
    }
}
