use crate::DrawResult;
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

    let source = sphere::generate_checker(1000, 500);

    sphere::render_sphere(size, -long0 as f32, &source, |x, y, r, g, b| {
        area.draw_pixel((x as i32, y as i32), &RGBColor(r, g, b))
    })?;

    Ok(())
}
