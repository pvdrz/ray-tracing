use crate::camera::*;
use crate::hitable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

use std::fs::File;
use std::io::{BufWriter, Write};

use rand::prelude::*;

mod camera;
mod hitable;
mod ray;
mod sphere;
mod vec3;

type Num = f64;
type Int = i64;

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    let mut result = Vec3::zero() + 1.0;
    while result.dot(result) >= 1.0 {
        result = Vec3::new(rng.gen(), rng.gen(), rng.gen());
    }
    result
}

fn color<T: Rng>(r: &Ray, world: &Hitable, rng: &mut T) -> Vec3 {
    let mut rec = HitRecord::zero();
    if world.hit(r, 0.0, std::f64::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere(rng);
        0.5 * color(&Ray::new(rec.p, target - rec.p), world, rng)
    } else {
        let unit_direction = r.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut file = BufWriter::new(File::create("hello.ppm")?);

    // let nx = 896;
    // let ny = 504;
    let nx = 800;
    let ny = 400;
    let ns = 100;

    write!(&mut file, "P3\n{} {}\n255\n", nx, ny)?;

    let mut world = HitableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (i as Num + rng.gen::<Num>()) / (nx as Num);
                let v = (j as Num + rng.gen::<Num>()) / (ny as Num);

                let r = camera.get_ray(u, v);

                let p = r.point_at(2.0);
                col += color(&r, &mut world, &mut rng);
            }
            col /= ns as Num;

            let ir = (255.99 * col.r()) as Int;
            let ig = (255.99 * col.g()) as Int;
            let ib = (255.99 * col.b()) as Int;

            write!(&mut file, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    Ok(())
}
