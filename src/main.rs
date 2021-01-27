#![allow(unused)]

use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, Pixel, Rgba};

use std::fs::OpenOptions;

mod ray;
mod vec;
use ray::Ray;
use vec::Vec3;

// Width of the screen
const WX: u32 = 200;

// Height of the screen
const HY: u32 = 100;

const RGB: f32 = 255.99;

fn main() {
    blue_fade("./test2.png")
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
