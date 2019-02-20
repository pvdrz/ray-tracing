extern crate ray_tracing;

use crate::ray_tracing::hitable::HitableList;
use crate::ray_tracing::vec3::Vec3;
use crate::ray_tracing::num::Num;
use crate::ray_tracing::camera::Camera;
use crate::ray_tracing::material::{Metal, Dielectric, Lambertian};
use crate::ray_tracing::sphere::Sphere;
use crate::ray_tracing::render;

use rand::prelude::*;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut world = HitableList::new();

    let nx = 800;
    let ny = 600;
    let ns = 100;

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::zero();
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

    world.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(0.7, 0.6, 0.5, 0.0),
    ));

    render("cover.ppm", world, camera, nx, ny, ns)
}
