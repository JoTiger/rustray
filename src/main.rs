pub mod vec3;
use vec3::dot;
use vec3::unit_vector;
use vec3::Vec3f;

pub mod ray;
use ray::Ray;

fn hit_sphere(center: &Vec3f, radius: f32, ray: &Ray) -> f32 {
    let oc: Vec3f = ray.origin() - center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(&oc, ray.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    }
    else
    {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(ray: &Ray) -> Vec3f {
    let sphereCenter = Vec3f {
        e: [0.0, 0.0, -1.0]
    };
    let t = hit_sphere(&sphereCenter, 0.5, ray);
    if t > 0.0 {
        let dN = ray.point_at_parameter(t) - sphereCenter;
        let N = unit_vector(&dN);
        return 0.5 * Vec3f { e : [
            N.x() + 1.0, N.y() + 1.0, N.z() + 1.0]
        }
    }

    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    let result = Vec3f { e: [1.0, 1.0, 1.0] } * (1.0 - t) + Vec3f { e: [0.5, 0.7, 1.0] } * t;
    result
}

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    let lower_left_corner = Vec3f {
        e: [-2.0, -1.0, -1.0],
    };
    let horiztonal = Vec3f { e: [4.0, 0.0, 0.0] };
    let vertical = Vec3f { e: [0.0, 2.0, 0.0] };
    let origin = Vec3f { e: [0.0, 0.0, 0.0] };
    for j in (0..ny).rev() {
        for i in (0..nx) {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray {
                a: origin,
                b: lower_left_corner + u * horiztonal + v * vertical,
            };
            let col = color(&r);
            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
