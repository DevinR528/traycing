#![allow(unused)]

use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, Pixel, Rgba};
use rand::random;

use std::fs::OpenOptions;

mod camera;
mod ray;
mod stuff;
mod vec;

use camera::Camera;
use ray::Ray;
use stuff::{Objects, Sphere};
use vec::Vec3;

// Width of the screen
const WX: u32 = 200;

// Height of the screen
const HY: u32 = 100;

const NS: u32 = 100;

const RGB: f32 = 255.99;

fn main() {
    hitable("./test3.png")
}

fn hitable(file: &str) {
    let mut image = DynamicImage::new_rgb8(200, 100);

    let camera = Camera::new();

    let objs = Objects::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ]);

    for y in 0..HY {
        for x in 0..WX {
            let mut color = Vec3::default();
            for s in 0..NS {
                let u = (x as f32 + random::<f32>()) / WX as f32;
                let v = (y as f32 + random::<f32>()) / HY as f32;

                let ray = camera.get_ray(u, v);

                let p = ray.point_at_parameter(2.0);
                color += stuff::color(&ray, &objs);
            }
            color /= NS as f32;
            color *= RGB;
            image.put_pixel(
                x,
                99 - y,
                Rgba::from_channels(color.x() as u8, color.y() as u8, color.z() as u8, RGB as u8),
            );
        }
    }

    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)
        .unwrap();

    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}

fn blue_fade(file: &str) {
    let mut image = DynamicImage::new_rgb8(200, 100);

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizon = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for y in 0..HY {
        for x in 0..WX {
            let u = x as f32 / WX as f32;
            let v = y as f32 / HY as f32;

            let ray = Ray::new(origin, lower_left + u * horizon + v * vertical);

            let mut color = ray::color(&ray);
            color *= RGB;

            image.put_pixel(
                x,
                99 - y,
                Rgba::from_channels(color.x() as u8, color.y() as u8, color.z() as u8, RGB as u8),
            );
        }
    }

    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)
        .unwrap();

    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}

fn rainbow_gradient(file: &str) {
    let mut image = DynamicImage::new_rgb8(200, 100);

    for y in 0..HY {
        for x in 0..WX {
            let vec = Vec3::new(x as f32 / WX as f32, y as f32 / HY as f32, 0.2);

            image.put_pixel(
                x,
                (HY - 1) - y,
                Rgba::from_channels(vec.x() as u8, vec.y() as u8, vec.z() as u8, RGB as u8),
            );
        }
    }

    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)
        .unwrap();

    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}
