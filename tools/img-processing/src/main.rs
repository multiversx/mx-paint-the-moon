#![allow(unused)]

use std::convert::Infallible;
use std::f32::consts::FRAC_PI_2;
use std::io::{Cursor, Write};
use std::{f32::consts::PI, fs::File};

use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};

const MOON_SOURCE_EMBEDDED: &[u8] = include_bytes!("../lroc_color_poles_1k.jpg");

fn main() {
    // render_checker().unwrap();
    // process_moon().unwrap();
    render_moon(800, 1.5).unwrap();
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

fn render_checker() -> anyhow::Result<()> {
    sphere::generate_checker(1000, 500).save("checker.bmp")?;
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
    let source = sphere::generate_checker(1000, 500);
    sphere(size, long0, &source)?;

    Ok(())
}

fn sphere(size: u32, long0: f32, source: &DynamicImage) -> anyhow::Result<()> {
    let mut rendered = DynamicImage::new(size, size, ColorType::Rgb8);

    sphere::render_sphere(size, long0, source, |x, y, r, g, b| {
        rendered.put_pixel(x, y, [r, g, b, 255u8].into());
        Result::<(), Infallible>::Ok(())
    });

    rendered.save("rendered.bmp")?;

    Ok(())
}
