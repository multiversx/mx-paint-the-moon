#![allow(unused)]

use std::f32::consts::FRAC_PI_2;
use std::io::{Cursor, Write};
use std::{f32::consts::PI, fs::File};

use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};

const MOON_SOURCE_EMBEDDED: &[u8] = include_bytes!("../lroc_color_poles_1k.jpg");

fn main() {
    // render_checker().unwrap();
    // process_moon().unwrap();
    render_moon(800, 0.0).unwrap();

    // let source = ImageReader::open("lroc_color_poles_2k.tif")?.decode()?;
    // source.resize(nwidth, nheight, filter)
}

fn process_moon() -> anyhow::Result<()> {
    let img = ImageReader::open("lroc_color_poles_2k.tif")?.decode()?;
    println!("{}:{}", img.width(), img.height());
    img.save("lroc_color_poles_2k.bmp")?;

    let bytes = img.into_bytes();

    let mut file = File::create("src/embedded_moon.rs")?;
    writeln!(file, "pub const MOON_BYTES: &[u8] = &{bytes:?};")?;

    Ok(())
}

fn xor(a: bool, b: bool) -> bool {
    (a && !b) || (!a && b)
}

fn checker(width: u32, height: u32) -> anyhow::Result<DynamicImage> {
    let mut checker = DynamicImage::new(width, height, ColorType::Rgb8);
    let checker_size = 25;
    let half_width = width / 2;
    let half_height = height / 2;
    for x in 0..width {
        for y in 0..height {
            if xor(x / checker_size % 2 == 0, y / checker_size % 2 == 0) {
                let pixel = if xor(x < half_width, y < half_height) {
                    let c = (x * 256 / width) as u8;
                    [c, 0, 0, 255u8].into()
                } else {
                    let c = (x * 256 / width) as u8;
                    [0, c, 0, 255u8].into()
                };
                checker.put_pixel(x, y, pixel);
            }
        }
    }

    checker.save("checker.bmp")?;

    Ok(checker)
}

fn render_checker() -> anyhow::Result<()> {
    let _ = checker(1000, 500)?;
    Ok(())
}

fn moon_source() -> anyhow::Result<DynamicImage> {
    let source =
        ImageReader::with_format(Cursor::new(MOON_SOURCE_EMBEDDED), image::ImageFormat::Jpeg)
            .decode()?;
    // let source = ImageReader::open("lroc_color_poles_1k.jpg")?.decode()?;
    // let source = ImageReader::open("lroc_color_poles_2k.tif")?.decode()?;

    Ok(source)
}

fn render_moon(size: u32, long0: f32) -> anyhow::Result<()> {
    // let source = moon_source()?;
    let source = checker(1000, 500)?;
    sphere(size, long0, &source)?;
    Ok(())
}

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

fn sphere(size: u32, long0: f32, source: &DynamicImage) -> anyhow::Result<()> {
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

        // println!("{sin_phi} ... {latitude} ... {source_y} ");

        for x in 0..size {
            if let Some(longitude) = longitude(x, small_r, globe_r) {
                let abs_long = longitude + long0;
                let source_xd = (abs_long / FRAC_PI_2) * source_r;
                let source_x = (source_mid_x + source_xd) as i32;
                let source_x_wrap = wrap_width(source_x, source.width());

                if source_y < source.height() {
                    let source_pixel = source.get_pixel(source_x_wrap, source_y);
                    rendered.put_pixel(x, y, source_pixel);
                } else {
                    println!("{source_y}");
                }
            }
        }

        // println!();
    }

    rendered.save("rendered.bmp")?;

    Ok(())
}
