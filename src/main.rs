use crate::hitable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

use std::fs::File;
use std::io::{BufWriter, Write};

mod hitable;
mod ray;
mod sphere;
mod vec3;

type Num = f32;
type Int = i32;

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let mut rec = HitRecord::zero();
    if world.hit(r, 0.0, std::f32::MAX, &mut rec) {
        0.5 * Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        )
    } else {
        let unit_direction = r.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create("hello.ppm")?);

    let nx = 896;
    let ny = 504;

    write!(&mut file, "P3\n{} {}\n255\n", nx, ny)?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut world = HitableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as Num) / (nx as Num);
            let v = (j as Num) / (ny as Num);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let p = r.point_at(2.0);
            let col = color(&r, &mut world);

            let ir = (255.99 * col.r()) as Int;
            let ig = (255.99 * col.g()) as Int;
            let ib = (255.99 * col.b()) as Int;

            write!(&mut file, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    Ok(())
}
