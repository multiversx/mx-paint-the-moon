#![allow(unused)]

use std::f32::consts::FRAC_PI_2;
use std::io::Write;
use std::{f32::consts::PI, fs::File};

use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageReader, Pixel, Rgba};

// fn moon_source() -> anyhow::Result<DynamicImage> {
//     let source = ImageReader::open("lroc_color_poles_1k.jpg")?.decode()?;
//     // let source = ImageReader::open("lroc_color_poles_2k.tif")?.decode()?;
//     Ok(source)
// }

// fn render_moon(size: u32) -> anyhow::Result<()> {
//     let source = moon_source()?;
//     // let source = checker(1000, 500)?;
//     sphere(size, 3.0, &source)?;
//     Ok(())
// }

fn latitude(y: u32, globe_r: f32) -> f32 {
    let yd = y as f32 - globe_r;
    let sin_phi = yd / globe_r;
    sin_phi.asin()
}

fn longitude(x: u32, small_r: f32, globe_r: f32) -> Option<f32> {
    let xd = x as f32 - globe_r;
    if xd >= -small_r && xd <= small_r {
        Some((xd / small_r).asin())
    } else {
        None
    }
}

fn wrap_width(x: i32, width: u32) -> u32 {
    x.rem_euclid(width as i32) as u32
}

pub fn sphere<F>(
    size: u32,
    long0: f32,
    source: &DynamicImage,
    mut put_pixel: F,
) -> anyhow::Result<()>
where
    F: FnMut(u32, u32, u8, u8, u8),
{
    let globe_r = (size / 2) as f32;

    println!("{}:{}", source.width(), source.height());
    let source_mid_x = (source.width() / 2) as f32;
    let source_mid_y = (source.height() / 2) as f32;

    let source_r = source_mid_y;

    let mut rendered = DynamicImage::new(size, size, ColorType::Rgb8);

    for y in 0..=size {
        let latitude = latitude(y, globe_r);
        let source_yd = (latitude / FRAC_PI_2) * source_r;
        let source_y = (source_mid_y + source_yd) as u32;

        let small_r = globe_r * latitude.cos();

        for x in 0..size {
            if let Some(longitude) = longitude(x, small_r, globe_r) {
                let abs_long = longitude + long0;
                let source_xd = (abs_long / FRAC_PI_2) * source_r;
                let source_x = (source_mid_x + source_xd) as i32;
                let source_x_wrap = wrap_width(source_x, source.width());

                if source_y < source.height() {
                    let source_pixel = source.get_pixel(source_x_wrap, source_y);
                    let channels = source_pixel.channels();
                    put_pixel(x, y, channels[0], channels[1], channels[2]);
                }
            }
        }
    }

    Ok(())
}
