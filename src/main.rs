use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, Pixel, Rgba};

use std::fs::{File, OpenOptions};

mod vec;
use vec::{Axis::*, Vec3};

const WX: u32 = 200;
const HY: u32 = 100;

const RGB: f32 = 255.99;

fn main() {
    let mut image = DynamicImage::new_rgb8(200, 100);

    for y in 0..HY {
        for x in 0..WX {
            let mut vec = Vec3::new(x as f32 / WX as f32, y as f32 / HY as f32, 0.2);

            vec *= RGB;
            println!("{:?}", vec);
            image.put_pixel(
                x,
                y,
                Rgba::from_channels(vec[X] as u8, vec[Y] as u8, vec[Z] as u8, RGB as u8),
            );
        }
    }

    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./test.png")
        .unwrap();

    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}
