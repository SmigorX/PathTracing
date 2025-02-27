mod utils {
    pub mod ray;
    pub mod color;
    pub mod hittable;
    pub mod shapes;
}


use std::{fs::File, io::Write};
use math::round;
use crate::utils::ray;


fn open_file() -> File {
    let file = File::options().write(true).create(true).open("image.ppm");

    if file.is_err() {
        panic!("Error: {:?}", file.err());
    } else {
        return file.unwrap();
    }
}

fn objects() -> utils::hittable::HittableList {
    let mut world: utils::hittable::HittableList = utils::hittable::HittableList::new();
    let sphere: utils::shapes::Sphere = utils::shapes::Sphere::new(cgmath::point3(-1.0, 1.5, -2.5), 1.0);
    world.add(sphere);
    let sphere2: utils::shapes::Sphere = utils::shapes::Sphere::new(cgmath::point3(1.0, 1.5, -2.5), 1.0);
    world.add(sphere2);
    let sphere3: utils::shapes::Sphere = utils::shapes::Sphere::new(cgmath::point3(-1.0, 1.0, -2.0), 0.5);
    world.add(sphere3);
    let sphere4: utils::shapes::Sphere = utils::shapes::Sphere::new(cgmath::point3(1.0, 1.0, -2.0), 0.5);
    world.add(sphere4);
    return world
}

fn main() {
    let mut file = open_file();

    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width: f64 = 400.0;
    let image_height: f64 = round::ceil(image_width / aspect_ratio, 0);

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64); 
    let camera_center: cgmath::Point3<f64> = cgmath::point3(0.0, 0.0, 0.0);
    let focal_length: f64 = 1.0;

    let viewport_u: cgmath::Vector3<f64> = cgmath::vec3(viewport_height, 0.0, 0.0);
    let viewport_v: cgmath::Vector3<f64> = cgmath::vec3(0.0, -viewport_width, 0.0);

    let pixel_delta_u: cgmath::Vector3<f64> = viewport_u / image_height;
    let pixel_delta_v: cgmath::Vector3<f64> = viewport_v / image_width;

    let viewport_upper_left: cgmath::Point3<f64> = camera_center - viewport_u / 2.0 - viewport_v / 2.0 - cgmath::vec3(0.0, 0.0, focal_length);

    let pixel_origin: cgmath::Point3<f64> = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let world: utils::hittable::HittableList = objects();

    file.write_all(std::format!("P3\n{image_width} {image_height}\n255\n").as_bytes()).unwrap();

    for i in 0..image_height as u32 {
        for j in 0..image_width as u32 {
            let pixel_position: cgmath::Point3<f64> = pixel_origin + (i as f64 * pixel_delta_v) + (j as f64 * pixel_delta_u);
            let ray_direction: cgmath::Vector3<f64> = pixel_position - camera_center;
            let ray: ray::Ray = ray::Ray::new(camera_center, ray_direction);

            let color: cgmath::Vector3<u32> = ray.color(&world);
            utils::color::write_pixel(color, &mut file);
        }
        if i % (image_height as u32 / 10) == 0 {
            println!("Progress: {}%", i / (image_height as u32 / 10) * 10);
        }
    }
}
