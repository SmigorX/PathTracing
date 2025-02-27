use crate::utils::{ray, hittable};

use cgmath::InnerSpace;
use hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: cgmath::Point3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: cgmath::Point3<f64>, radius: f64) -> Sphere {
        if radius <= 0.0 {
            panic!("Radius must be greater than 0");
        }
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let line: cgmath::Vector3<f64> = self.center - r.get_origin();
        let a = r.get_direction().magnitude2();
        let h = r.get_direction().dot(line);
        let c = line.magnitude2() - self.radius * self.radius;
        let discriminant = h * h - a * c;


        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (h + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, (rec.p - self.center) / self.radius);

        return true;
    }
}
