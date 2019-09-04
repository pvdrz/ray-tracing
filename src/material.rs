use crate::hitable::HitRecord;
use crate::num::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::prelude::{Rng, ThreadRng};

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    let mut result = Vec3::from_scalar(1.0);
    while result.dot(result) >= 1.0 {
        result = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - 1.0;
    }
    result
}

pub trait Material {
    fn scatter(
        &self,
        _: &Ray,
        _: &HitRecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut ThreadRng,
    ) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo_x: Num, albedo_y: Num, albedo_z: Num) -> Self {
        Lambertian {
            albedo: Vec3::new(albedo_x, albedo_y, albedo_z),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere(rng);
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: Num,
}

impl Metal {
    pub fn new(albedo_x: Num, albedo_y: Num, albedo_z: Num, f: Num) -> Self {
        Metal {
            albedo: Vec3::new(albedo_x, albedo_y, albedo_z),
            fuzz: if f > 1.0 { 1.0 } else { f },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let reflected = reflect(r_in.direction().unit(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(rng));
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal) > 0.0
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ref_idx: Num,
}

impl Dielectric {
    pub fn new(ref_idx: Num) -> Self {
        Dielectric { ref_idx }
    }
    fn schlick(&self, cos: Num) -> Num {
        let mut r0 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let outward_normal;
        let reflected = reflect(r_in.direction(), rec.normal);
        let ratio;
        *attenuation = Vec3::from_scalar(1.0);
        let mut refracted = Vec3::default();
        let mut cos = r_in.direction().dot(rec.normal) / r_in.direction().len();
        if r_in.direction().dot(rec.normal) > 0.0 {
            outward_normal = -1.0 * rec.normal;
            ratio = self.ref_idx;
            cos *= self.ref_idx;
        } else {
            outward_normal = rec.normal;
            ratio = 1.0 / self.ref_idx;
            cos *= -1.0;
        }
        if refract(r_in.direction(), outward_normal, ratio, &mut refracted)
            && rng.gen::<Num>() >= self.schlick(cos)
        {
            *scattered = Ray::new(rec.p, refracted);
        } else {
            *scattered = Ray::new(rec.p, reflected);
        }
        true
    }
}

#[derive(Clone)]
pub struct Dummy;

impl Material for Dummy {}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(v: Vec3, n: Vec3, ratio: Num, refracted: &mut Vec3) -> bool {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ratio * ratio * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ratio * (uv - n * dt) - n * discriminant.sqrt();
        true
    } else {
        false
    }
}
