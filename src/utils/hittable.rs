use crate::utils::ray;

use cgmath::InnerSpace;

pub struct HitRecord {
    pub p: cgmath::Point3<f64>,
    pub normal: cgmath::Vector3<f64>,
    pub t: f64,
    pub front_face: bool, // NOTE: this might not be necessary 
}

impl HitRecord {
    pub fn new(p: cgmath::Point3<f64>, normal: cgmath::Vector3<f64>, t: f64) -> HitRecord {
        HitRecord { p, normal, t, front_face: false }
    }

    pub fn clone(&self) -> HitRecord {
        HitRecord { p: self.p, normal: self.normal, t: self.t, front_face: self.front_face }
    }

    pub fn set_face_normal(&mut self, r: &ray::Ray, normal: cgmath::Vector3<f64>) {
        let dot_product: f64 = normal.dot(r.get_direction());
        if dot_product > 0.0 {
            self.front_face = false;
            self.normal = -normal;
        } else {
            self.front_face = true;
            self.normal = normal;
        }
    } 
}


pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}


pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = HitRecord::new(cgmath::point3(0.0, 0.0, 0.0), cgmath::vec3(0.0, 0.0, 0.0), 0.0);

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone(); 
            }
        }

        hit_anything
    }
}


