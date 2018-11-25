pub mod vec3;
use vec3::Vec3f;

pub mod ray;
use ray::Ray;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in (0..nx) {
            let fv = Vec3f {
                e: [i as f32 / nx as f32, j as f32 / ny as f32, 0.2],
            };
            let ir: i32 = (255.99 * fv.r()) as i32;
            let ig: i32 = (255.99 * fv.g()) as i32;
            let ib: i32 = (255.99 * fv.b()) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
