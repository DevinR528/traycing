#![allow(unused)]

use rand::random;

use std::fs::OpenOptions;

mod camera;
mod material;
mod ray;
mod stuff;
mod vec;

use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use camera::Camera;
use material::{Lambertian, Reflective};
use ray::Ray;
use stuff::{Objects, Sphere};
use vec::Vec3;

// Width of the screen
const WX: u32 = 800;

// Height of the screen
const HY: u32 = 400;

const NS: u32 = 100;

const RGB: f32 = 255.99;

fn main() {
    hitable("./test8.png")
}

fn hitable(file: &str) {
    let camera = Camera::new();
    let small_plastic = Lambertian::new(Vec3::new(0.5, 0.9, 0.5));
    let ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let gold_metal = Reflective::new(Vec3::new(0.8, 0.6, 0.2));
    let metal = Reflective::new(Vec3::new(0.8, 0.8, 0.8));

    let objs = Objects::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &small_plastic),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &ground),
        Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &gold_metal),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &metal),
    ]);

    let camera_ref = &camera;
    let objs_ref = &objs;
    let pixels = (0..HY)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            (0..WX).into_par_iter().flat_map(move |x| {
                let mut color = Vec3::default();
                for s in 0..NS {
                    let u = (x as f32 + random::<f32>()) / WX as f32;
                    let v = (y as f32 + random::<f32>()) / HY as f32;

                    let ray = camera_ref.get_ray(u, v);

                    color += stuff::color(ray, objs_ref, 0);
                }
                color /= NS as f32;
                color *= RGB;
                vec![color.x() as u8, color.y() as u8, color.z() as u8]
            })
        })
        .collect::<Vec<u8>>();

    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)
        .unwrap();

    let mut enc = png::Encoder::new(image_file, WX, HY);
    enc.set_color(png::ColorType::RGB);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
}

/*
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

            let mut color = ray::color(ray);
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
*/
