use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::num::*;

use rand::Rng;

fn random_in_unit_disk<T: Rng>(rng: &mut T) -> Vec3 {
    let mut p = Vec3::from_scalar(1.0);
    while p.dot(p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<Num>() - 0.5, rng.gen::<Num>() - 0.5, 0.0);
    }
    p
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: Num,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: Num,
        aspect: Num,
        aperture: Num,
        focus_dist: Num,
    ) -> Self {
        let lens_radius = 0.5 * aperture;

        let theta = vfov * PI / 180.0;
        let height = 2.0 * (theta / 2.0).tan();
        let width = aspect * height;

        let origin = lookfrom;

        let w = (lookfrom - lookat).unit();
        let u = (vup.cross(w)).unit();
        let v = w.cross(u);

        let lower_left_corner =
            origin - 0.5 * focus_dist * (width * u + height * v) - focus_dist * w;
        let horizontal = width * focus_dist * u;
        let vertical = height * focus_dist * v;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray<T: Rng>(&self, s: Num, t: Num, rng: &mut T) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
