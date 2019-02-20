pub mod camera;
pub mod hitable;
pub mod bvh;
pub mod bounding_box;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod stl;
pub mod triangle;
pub mod vec3;
pub mod num;

use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::vec3::Vec3;
use crate::num::{Int, Num, MAX_NUM};
use crate::camera::Camera;

use std::fs::File;
use std::io::{BufWriter, Write};

use rand::prelude::*;
use rayon::prelude::*;

fn color(r: &Ray, world: &Hitable, depth: Int, rng: &mut ThreadRng) -> Vec3 {
    let mut rec = HitRecord::zero();
    if world.hit(r, 0.001, MAX_NUM, &mut rec) {
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();

        if depth < 50
            && rec
                .material
                .scatter(r, &rec, &mut attenuation, &mut scattered, rng)
        {
            attenuation * color(&scattered, world, depth + 1, rng)
        } else {
            Vec3::zero()
        }
    } else {
        let unit_direction = r.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::from_scalar(1.0 - t) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

pub fn render<T: Hitable>(path: &str, world: T, camera: Camera, nx: Int, ny: Int, ns: Int) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);

    write!(&mut file, "P3\n{} {}\n255\n", nx, ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = (0..ns)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u = (i as Num + rng.gen::<Num>()) / (nx as Num);
                    let v = (j as Num + rng.gen::<Num>()) / (ny as Num);

                    let r = camera.get_ray(u, v, &mut rng);

                    color(&r, &world, 0, &mut rng)
                })
                .reduce(|| Vec3::zero(), |x, acc| x + acc)
                / ns as Num;

            let ir = (255.99 * col.r().sqrt()) as Int;
            let ig = (255.99 * col.g().sqrt()) as Int;
            let ib = (255.99 * col.b().sqrt()) as Int;

            write!(&mut file, "{} {} {}\n", ir, ig, ib)?;
        }
    }

    Ok(())
}
