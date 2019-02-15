use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Num;

use rand::prelude::{Rng, ThreadRng};

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    let mut result = Vec3::zero() + 1.0;
    while result.dot(result) >= 1.0 {
        result = Vec3::new(rng.gen(), rng.gen(), rng.gen());
    }
    result
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dummy,
}

impl Material {
    pub fn lambertian(albedo_x: Num, albedo_y: Num, albedo_z: Num) -> Self {
        Material::Lambertian(Lambertian {
            albedo: Vec3::new(albedo_x, albedo_y, albedo_z),
        })
    }
    pub fn metal(albedo_x: Num, albedo_y: Num, albedo_z: Num) -> Self {
        Material::Metal(Metal {
            albedo: Vec3::new(albedo_x, albedo_y, albedo_z),
        })
    }
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        match self {
            Material::Lambertian(m) => m.scatter(r_in, rec, attenuation, scattered, rng),
            Material::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
            Material::Dummy => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
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
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn reflect(&self, v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = self.reflect(r_in.direction().unit(), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal) > 0.0
    }
}
