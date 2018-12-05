pub mod vec3;
use vec3::dot;
use vec3::unit_vector;
use vec3::Vec3f;

pub mod ray;
use ray::Ray;

pub mod hit;
pub mod sphere;
use hit::HitRecord;
use hit::Hitable;
use hit::HitableList;
use sphere::Sphere;

pub mod camera;
use camera::Camera;

extern crate rand;
use rand::{thread_rng, Rng};

use std::f32;

fn color(ray: &Ray, world: &HitableList) -> Vec3f {
    let mut rec = HitRecord {
      t: 0.0,
      p: Default::default(),
      normal: Default::default()
    };
    if world.hit(ray, 0.0, f32::MAX, &mut rec) {
        return 0.5 * Vec3f::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0);
    } else {
        let unit_direction = unit_vector(ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3f::new(1.0, 1.0, 1.0) + t * Vec3f::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx: i32 = 400;
    let ny: i32 = 300;
    let ns: i32 = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    

    let mut world = HitableList { list: Vec::new() };

    world.list.push(Box::new(Sphere::new(&Vec3f::new(0.0, 0.0, -1.0), 0.5)));
    world.list.push(Box::new(Sphere::from((0.0, -100.5, -1.0, 100.0))));

    let cam : Camera = Default::default();

    let mut rng = thread_rng();

    for j in (0..ny).rev() {
        for i in (0..nx) {
            let mut col = Vec3f::default();
            for s in (0..ns) {
                let randu : f32 = rng.gen_range(0.0, 1.0);
                let randv : f32 = rng.gen_range(0.0, 1.0);
                let u = (i as f32 + randu) / nx as f32;
                let v = (j as f32 + randv) / ny as f32;
                let r = cam.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col = col + color(&r, &world);
            }
            col = col / (ns as f32);
            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
