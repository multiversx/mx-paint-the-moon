use crate::{sphere::sphere, DrawResult};
use image::{DynamicImage, ImageReader};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use std::{cmp::min, io::Cursor};
use web_sys::HtmlCanvasElement;

const MOON_SOURCE_EMBEDDED: &[u8] = include_bytes!("../lroc_color_poles_1k.jpg");

pub fn moon_source() -> anyhow::Result<DynamicImage> {
    let source =
        ImageReader::with_format(Cursor::new(MOON_SOURCE_EMBEDDED), image::ImageFormat::Jpeg)
            .decode()?;

    Ok(source)
}

pub fn draw(canvas: HtmlCanvasElement, long0: f64) -> DrawResult<()> {
    let area = CanvasBackend::with_canvas_object(canvas)
        .unwrap()
        .into_drawing_area();
    let (width, height) = area.dim_in_pixel();
    let size = min(width, height);

    area.fill(&BLACK)?;

    let source = moon_source()?;

    sphere(size, -long0 as f32, &source, |x, y, r, g, b| {
        area.draw_pixel((x as i32, y as i32), &RGBColor(r, g, b))
    })?;

    Ok(())
}
