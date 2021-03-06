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

pub mod sample;
use sample::random_in_unit_sphere;

pub mod material;
use material::Dielectric;
use material::Lambert;
use material::Material;
use material::Metal;

pub mod math;

fn color(ray: &Ray, world: &HitableList, depth: i32) -> Vec3f {
    let mut rec = HitRecord {
        t: 0.0,
        p: Default::default(),
        normal: Default::default(),
        mat: None,
    };
    if world.hit(ray, 0.001, f32::MAX, &mut rec) {
        let mut scattered = Ray {
            a: Vec3f::default(),
            b: Vec3f::default(),
        };
        let mut attenuation = Vec3f::default();
        if (depth < 50)
            && rec.mat.is_some()
            && rec
                .mat
                .unwrap()
                .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            Vec3f::default()
        }
    } else {
        let unit_direction = unit_vector(ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3f::new(1.0, 1.0, 1.0) + t * Vec3f::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let mut rng = thread_rng();

    let nx: i32 = 1200;
    let ny: i32 = 800;
    let ns: i32 = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    let mut materials: Vec<Box<Material>> = Vec::new();

    let mat1 = Lambert {
        albedo: Vec3f::new(0.5, 0.5, 0.5),
    };

    let mat2 = Dielectric {
        ref_idx : 1.5
    };

    let mat3 = Lambert {
        albedo : Vec3f::new(0.4, 0.2, 0.1),
    };

    let mat4 = Metal {
        albedo : Vec3f::new(0.7, 0.6, 0.5),
        fuzz : 0.0
    };

    let mut world = HitableList { list: Vec::new() };
    
    for m in (0..500) {
        let choose_mat = rng.gen_range(0.0, 1.0);
        if choose_mat < 0.8 {
            materials.push(Box::new(Lambert {
                albedo: Vec3f::new(
                    rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                ),
            }));
        } else if choose_mat < 0.95 {
            materials.push(Box::new(Metal {
                albedo: Vec3f::new(
                    0.5 + 0.5 * rng.gen_range(0.0, 1.0),
                    0.5 + 0.5 * rng.gen_range(0.0, 1.0),
                    0.5 + 0.5 * rng.gen_range(0.0, 1.0)
                ),
                fuzz : 0.5 * rng.gen_range(0.0, 1.0)
            }));
        } else {
            materials.push(Box::new(Dielectric {
                ref_idx : 1.5
            }))
        }
    }

    let mut sidx = 0;
    for a in (-11..11) {
        for b in (-11..11) {
            let center = Vec3f {
                e: [
                    a as f32 + 0.9 * rng.gen_range(0.0, 1.0),
                    0.2,
                    b as f32 + 0.9 * rng.gen_range(0.0, 1.0),
                ],
            };
            if (center - Vec3f { e: [4.0, 0.2, 0.0] }).length() > 0.9 {
                let mat = Some(&*materials[sidx]);
                sidx += 1;
                world.list.push(Box::new(Sphere::new(&center, 0.2, mat)));
            }
        }
    }

    
    world.list.push(Box::new(Sphere::new(
        &Vec3f::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(&mat1))));

    world.list.push(Box::new(Sphere::new(
        &Vec3f::new(0.0, 1.0, 0.0),
        1.0,
        Some(&mat2))));

    world.list.push(Box::new(Sphere::new(
        &Vec3f::new(-4.0, 1.0, 0.0),
        1.0,
        Some(&mat3))));

    world.list.push(Box::new(Sphere::new(
        &Vec3f::new(4.0, 1.0, 0.0),
        1.0,
        Some(&mat4))));

    let lookfrom = Vec3f::new(13.0, 2.0, 3.0);
    let lookat = Vec3f::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3f::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );
    let R = (std::f32::consts::PI / 4.0).cos();

    for j in (0..ny).rev() {
        for i in (0..nx) {
            let mut col = Vec3f::default();
            for s in (0..ns) {
                let randu: f32 = rng.gen_range(0.0, 1.0);
                let randv: f32 = rng.gen_range(0.0, 1.0);
                let u = (i as f32 + randu) / nx as f32;
                let v = (j as f32 + randv) / ny as f32;
                let r = cam.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col = col + color(&r, &world, 0);
            }
            col = col / (ns as f32);
            col = Vec3f::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
