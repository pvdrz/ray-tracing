#![allow(dead_code)]
#![allow(unused_imports)]

use crate::camera::*;
use crate::hitable::*;
use crate::material::*;
use crate::ray::*;
use crate::sphere::*;
use crate::stl::*;
use crate::triangle::*;
use crate::vec3::*;

use std::fs::File;
use std::io::{BufWriter, Write};

use rand::prelude::*;
use rayon::prelude::*;

mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod stl;
mod triangle;
mod vec3;

type Num = f64;
type Int = i64;

const MAX_NUM: Num = std::f64::MAX;
const PI: Num = std::f64::consts::PI;

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
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create("hello.ppm")?);

    // let nx = 896;
    // let ny = 504;
    let nx = 800;
    let ny = 400;
    let ns = 100;

    write!(&mut file, "P3\n{} {}\n255\n", nx, ny)?;

    let mut world = HitableList::new();

    // let triangles = read_bin(
    //     "/home/christian/Downloads/shapes/doomguy.stl",
    //     Vec3::new(1.0, 1.0, 1.0) / 50.0,
    //     Vec3::new(-2.0, -1.7, -1.5),
    //     Material::dielectric(1.5),
    // )?;
    // for triangle in triangles {
    //     world.add(triangle);
    // }

    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::lambertian(0.1, 0.2, 0.5),
    ));

    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::lambertian(0.8, 0.8, 0.0),
    ));

    world.add(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::metal(0.8, 0.6, 0.2, 0.3),
    ));

    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::dielectric(1.5),
    ));

    // let r = (PI / 4.0).cos();
    //
    // world.add(Sphere::new(
    //     Vec3::new(-r, 0.0, -1.0),
    //     r,
    //     Material::lambertian(0.0, 0.0, 1.0),
    // ));
    //
    // world.add(Sphere::new(
    //     Vec3::new(r, 0.0, -1.0),
    //     r,
    //     Material::lambertian(1.0, 0.0, 0.0),
    // ));

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (nx as Num) / (ny as Num),
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = (0..ns)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u = (i as Num + rng.gen::<Num>()) / (nx as Num);
                    let v = (j as Num + rng.gen::<Num>()) / (ny as Num);

                    let r = camera.get_ray(u, v);

                    // let p = r.point_at(2.0);
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
