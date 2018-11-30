pub mod vec3;
use vec3::unit_vector;
use vec3::Vec3f;

pub mod ray;
use ray::Ray;

fn color(ray: &Ray) -> Vec3f {
    let unit_direction = unit_vector(&ray.b);
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
                a : origin,
                b : lower_left_corner + horiztonal * u + vertical * v
            };
            let col = color(&r);
            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
