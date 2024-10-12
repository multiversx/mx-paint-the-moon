use crate::{sphere::sphere, DrawResult};
use image::{ColorType, DynamicImage, GenericImageView, ImageReader, Pixel};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use std::io::Cursor;
use web_sys::HtmlCanvasElement;

const MOON_SOURCE_EMBEDDED: &[u8] = include_bytes!("../lroc_color_poles_1k.jpg");

pub fn moon_source() -> anyhow::Result<DynamicImage> {
    let source =
        ImageReader::with_format(Cursor::new(MOON_SOURCE_EMBEDDED), image::ImageFormat::Jpeg)
            .decode()?;

    Ok(source)
}

pub fn draw(canvas: HtmlCanvasElement, pitch: f64, yaw: f64) -> DrawResult<()> {
    let area = CanvasBackend::with_canvas_object(canvas)
        .unwrap()
        .into_drawing_area();
    let (width, height) = area.dim_in_pixel();

    area.fill(&BLACK)?;

    let source = moon_source()?;

    // for x in 0..width {
    //     for y in 0..height {
    //         let pixel = source.get_pixel(x, y);
    //         let channels = pixel.channels();
    //         area.draw_pixel(
    //             (x as i32, y as i32),
    //             &RGBColor(channels[0], channels[1], channels[2]),
    //         )?;
    //     }
    // }

    sphere(height, -pitch as f32, &source, |x, y, r, g, b| {
        area.draw_pixel((x as i32, y as i32), &RGBColor(r, g, b))
            .unwrap();
    })?;

    Ok(())
}
