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

use std::f32;

fn color(ray: &Ray, world: &HitableList) -> Vec3f {
    let mut rec = HitRecord {
      t: 0.0,
      p: Default::default(),
      normal: Default::default()
    };
    if world.hit(ray, 0.0, f32::MAX, &mut rec) {
        return 0.5 * Vec3f {
            e: [
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            ],
        };
    } else {
        let unit_direction = unit_vector(ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3f::new(1.0, 1.0, 1.0) + t * Vec3f::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx: i32 = 400;
    let ny: i32 = 300;
    print!("P3\n{} {}\n255\n", nx, ny);
    let lower_left_corner = Vec3f {
        e: [-2.0, -1.5, -1.0],
    };
    let horiztonal = Vec3f::new(4.0, 0.0, 0.0);
    let vertical = Vec3f::new(0.0, 3.0, 0.0);
    let origin : Vec3f = Default::default();

    let mut world = HitableList { list: Vec::new() };

    world.list.push(Box::new(Sphere::new(&Vec3f::new(0.0, 0.0, -1.0), 0.5)));

    world.list.push(Box::new(Sphere::from((0.0, -100.5, -1.0, 100.0))));

    for j in (0..ny).rev() {
        for i in (0..nx) {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray {
                a: origin,
                b: lower_left_corner + u * horiztonal + v * vertical,
            };
            let col = color(&r, &world);
            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
