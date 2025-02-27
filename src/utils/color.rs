use cgmath;
use std::{fs::File, io::Write, format};

pub fn write_pixel(rgb: cgmath::Vector3<u32>, file: &mut File) {
    file.write_all((format!("{} {} {}\n", rgb.x, rgb.y, rgb.z)).as_bytes()).unwrap();
}

