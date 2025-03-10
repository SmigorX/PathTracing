use crate::utils::hittable::Hittable;
use crate::utils::hittable;

use cgmath::Vector3;
use cgmath::InnerSpace;

pub struct Ray {
    origin: cgmath::Point3<f64>,
    direction: cgmath::Vector3<f64>,
}

impl Ray {
    /// Creates a new Ray object
    ///
    /// # Arguments
    /// * `origin` - The center of the camera
    /// * `direction` - The direction of the ray
    ///
    /// # Returns
    /// * A Ray object
    pub fn new(origin: cgmath::Point3<f64>, direction: cgmath::Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn get_direction(&self) -> cgmath::Vector3<f64> {
        self.direction.normalize()
    }

    pub fn get_origin(&self) -> cgmath::Point3<f64> {
        self.origin
    }

    /// Returns the point at a given time
    ///
    /// # Arguments
    /// * `t` - The time
    ///
    /// # Returns
    /// * The point at time `t`
    pub fn at(&self, t: f64) -> cgmath::Point3<f64> {
        self.origin + self.direction * t
    }

    /// Returns the color the ray hits
    ///
    /// # Returns
    /// * RGB vector of the color
    pub fn color(&self, world: &hittable::HittableList) -> cgmath::Vector3<u32> {
        let mut hit_record = hittable::HitRecord::new(cgmath::point3(0.0, 0.0, 0.0), cgmath::vec3(0.0, 0.0, 0.0), 0.0);
        if world.hit(self, 0.0, f64::INFINITY, &mut hit_record) {
            return (0.5 * (hit_record.normal + cgmath::vec3(1.0, 1.0, 1.0))).cast().unwrap().map(|x: f64| (x * 255.999) as u32);
        }

        let unit_direction: cgmath::Vector3<f64> = self.direction.normalize();
        let t: f64 = 0.5 * (unit_direction.y + 1.0);
        
        return (cgmath::vec3(1.0, 1.0, 1.0).map(|x: f64| (x * (1.0 - t))) + cgmath::vec3(0.5, 0.7, 1.0).map(|x: f64| (x * t))).map(|x: f64| (x * 255.999) as u32);    
    }
}
