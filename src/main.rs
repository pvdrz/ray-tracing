use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};

mod ray;
mod vec3;

type Num = f64;
type Int = i64;

fn hit_sphere(center: Vec3, radius: Num, ray: &Ray) -> bool {
    let oc = ray.origin() - center;

    let a = ray.direction().dot(ray.direction());
    let b = (oc.dot(ray.direction())) * 2.0;
    let c = oc.dot(oc) - radius * radius;

    b * b - 4.0 * a * c > 0.0
}

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3::from_point(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::from_point(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::from_point(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::from_point(0.5, 0.7, 1.0) * t
}

fn main() -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create("hello.ppm")?);

    let nx = 896;
    let ny = 504;

    write!(&mut file, "P3\n{} {}\n255\n", nx, ny)?;

    let lower_left_corner = Vec3::from_point(-2.0, -1.0, -1.0);
    let horizontal = Vec3::from_point(4.0, 0.0, 0.0);
    let vertical = Vec3::from_point(0.0, 2.0, 0.0);
    let origin = Vec3::from_point(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as Num) / (nx as Num);
            let v = (j as Num) / (ny as Num);

            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r);

            let ir = (255.99 * col.r()) as Int;
            let ig = (255.99 * col.g()) as Int;
            let ib = (255.99 * col.b()) as Int;

            write!(&mut file, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    Ok(())
}
