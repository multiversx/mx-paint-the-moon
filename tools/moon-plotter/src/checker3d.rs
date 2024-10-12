use crate::{sphere::sphere, DrawResult};
use image::{ColorType, DynamicImage, GenericImage};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use std::cmp::min;
use web_sys::HtmlCanvasElement;

pub fn draw(canvas: HtmlCanvasElement, long0: f64) -> DrawResult<()> {
    let area = CanvasBackend::with_canvas_object(canvas)
        .unwrap()
        .into_drawing_area();
    let (width, height) = area.dim_in_pixel();
    let size = min(width, height);

    area.fill(&BLACK)?;

    let source = checker(1000, 500)?;

    sphere(size, -long0 as f32, &source, |x, y, r, g, b| {
        area.draw_pixel((x as i32, y as i32), &RGBColor(r, g, b))
    })?;

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

    Ok(checker)
}
