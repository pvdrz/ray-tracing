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
use crate::num::*;
use crate::bvh::*;

use std::fs::File;
use std::io::{BufWriter, Write};

use rand::prelude::*;
use rayon::prelude::*;

mod camera;
mod hitable;
mod bvh;
mod bounding_box;
mod material;
mod ray;
mod sphere;
mod stl;
mod triangle;
mod vec3;
mod num;

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

fn cover() -> HitableList {
    let mut rng = rand::thread_rng();

    let mut world = HitableList::new();

    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(0.5, 0.5, 0.5),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<Num>();
            let center = Vec3::new(
                a as Num + 0.9 * rng.gen::<Num>(),
                0.2,
                b as Num + 0.9 * rng.gen::<Num>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(
                            rng.gen::<Num>() * rng.gen::<Num>(),
                            rng.gen::<Num>() * rng.gen::<Num>(),
                            rng.gen::<Num>() * rng.gen::<Num>(),
                        ),
                    ));
                } else if choose_mat < 0.95 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            0.5 * (1.0 + rng.gen::<Num>()),
                            0.5 * (1.0 + rng.gen::<Num>()),
                            0.5 * (1.0 + rng.gen::<Num>()),
                            0.5 * rng.gen::<Num>(),
                        ),
                    ));
                } else {
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));

    world.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(0.4, 0.2, 0.1),
    ));
    // world.add(Sphere::new(
    //     Vec3::new(4.0, 1.0, 0.0),
    //     1.0,
    //     Metal::new(0.7, 0.6, 0.5, 0.0),
    // ));

    world
}

fn main() -> std::io::Result<()> {
    // let mut file = BufWriter::new(File::create("hello.ppm")?);
    let mut file = File::create("hello.ppm")?;

    // let nx = 896;
    // let ny = 504;
    let nx = 200;
    let ny = 100;
    let ns = 10;

    write!(&mut file, "P3\n{} {}\n255\n", nx, ny)?;
                    let mut rng = rand::thread_rng();
    // let world = cover();
    // let mut world = HitableList::new();
    //
    // world.add(Sphere::new(
    //     Vec3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     Material::lambertian(0.8, 0.8, 0.0),
    // ));

    let mut hitables = cover();

    let triangles = read_bin(
        "/home/christian/Downloads/shapes/doomguy.stl",
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.2, 0.0),
        Metal::new(0.8, 0.6, 0.2, 0.2),
    )?;

    for triangle in triangles {
        hitables.add(triangle);
    }

    let world = Node::new(hitables.inner, 0.0, MAX_NUM, &mut rng);

    // world.add(Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     Material::lambertian(0.1, 0.2, 0.5),
    // ));
    //
    // world.add(Sphere::new(
    //     Vec3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     Material::metal(0.8, 0.6, 0.2, 0.3),
    // ));
    //
    // world.add(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     Material::dielectric(1.5),
    // ));

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::zero();
    // let dist_to_focus = (lookfrom - lookat).len();
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (nx as Num) / (ny as Num),
        aperture,
        dist_to_focus,
    );

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
