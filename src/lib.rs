pub mod bounding_box;
pub mod camera;
pub mod collections;
pub mod hitable;
pub mod material;
pub mod num;
pub mod ray;
pub mod sphere;
pub mod stl;
pub mod triangle;
pub mod vec3;

use crate::camera::Camera;
use crate::hitable::{HitRecord, Hitable};
use crate::num::{Int, Num, MAX_NUM};
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::fs::File;
use std::io::{BufWriter, Write};

use rand::prelude::*;
use rayon::prelude::*;

fn color(mut r: Ray, world: &dyn Hitable, rng: &mut ThreadRng) -> Vec3 {
    let mut rec = HitRecord::default();
    let mut color = Vec3::from_scalar(1.0);
    let mut depth = 0;

    loop {
        if world.hit(&r, 0.001, MAX_NUM, &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            if depth < 50
                && rec
                    .material
                    .scatter(&r, &rec, &mut attenuation, &mut scattered, rng)
            {
                color *= attenuation;
                r = scattered;
                depth += 1;
                rec = HitRecord::default();
            } else {
                color = Vec3::default();
                break;
            }
        } else {
            color *= background_color(&r);
            break;
        }
    }
    color
}

fn background_color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::from_scalar(1.0 - t) + t * Vec3::new(0.5, 0.7, 1.0)
}

pub fn render<T: Hitable>(
    path: &str,
    world: T,
    camera: Camera,
    nx: Int,
    ny: Int,
    ns: Int,
) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);

    write!(&mut file, "P3\n{} {}\n255\n", nx, ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let mut rng = rand::thread_rng();
                let u = (i as Num + rng.gen::<Num>()) / (nx as Num);
                let v = (j as Num + rng.gen::<Num>()) / (ny as Num);

                let r = camera.get_ray(u, v, &mut rng);

                col += color(r, &world, &mut rng)
            }
            col /= ns as Num;

            let i = 255.99 * col.sqrt();

            writeln!(
                &mut file,
                "{} {} {}",
                i.r() as Int,
                i.g() as Int,
                i.b() as Int
            )?;
        }
    }

    Ok(())
}
