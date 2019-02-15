use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Num;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: Num, aspect: Num) -> Self {
        let theta = vfov * crate::PI / 180.0;
        let height = 2.0 * (theta / 2.0).tan();
        let width = aspect * height;
        let w = (lookfrom - lookat).unit();
        let u = (vup.cross(w)).unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let lower_left_corner = origin - 0.5 * (width * u + height * v) - w;
        let horizontal = width * u;
        let vertical = height * v;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: Num, v: Num) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
