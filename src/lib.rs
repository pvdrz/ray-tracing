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
    let t = 0.5 * (r.direction().unit().y() + 1.0);
    t * Vec3::new(0.5, 0.7, 1.0) + (1.0 - t)
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

    let num_nx = nx as Num;
    let num_ny = ny as Num;

    let colors = (0..ny as usize)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let mut rng = rand::thread_rng();
            let j = j as Num;
            (0..nx)
                .map(|i| {
                    let i = i as Num;
                    let mut col = Vec3::default();
                    for _ in 0..ns {
                        let u = (i + rng.gen::<Num>()) / num_nx;
                        let v = (j + rng.gen::<Num>()) / num_ny;

                        let r = camera.get_ray(u, v, &mut rng);

                        col += color(r, &world, &mut rng)
                    }
                    col /= ns as Num;

                    255.99 * col.sqrt()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for i in colors {
        writeln!(
            &mut file,
            "{} {} {}",
            i.r() as Int,
            i.g() as Int,
            i.b() as Int
        )?;
    }
    Ok(())
}
