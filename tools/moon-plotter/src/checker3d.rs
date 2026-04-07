use crate::DrawResult;
use image::DynamicImage;
use std::sync::OnceLock;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

static CHECKER_SOURCE: OnceLock<DynamicImage> = OnceLock::new();

fn checker_source() -> &'static DynamicImage {
    CHECKER_SOURCE.get_or_init(|| sphere::generate_checker(1000, 500))
}

pub fn draw(canvas: HtmlCanvasElement, long0: f64) -> DrawResult<()> {
    let size = canvas.width().min(canvas.height());

    let mut buf = vec![0u8; (size * size * 4) as usize];
    sphere::render_sphere(size, -long0 as f32, checker_source(), |x, y, r, g, b| {
        let idx = ((y * size + x) * 4) as usize;
        buf[idx] = r;
        buf[idx + 1] = g;
        buf[idx + 2] = b;
        buf[idx + 3] = 255;
        Ok::<_, Box<dyn std::error::Error>>(())
    })?;

    let image_data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buf), size, size)
            .map_err(|e| format!("{e:?}"))?;
    let ctx = canvas
        .get_context("2d")
        .map_err(|e| format!("{e:?}"))?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| "expected CanvasRenderingContext2d")?;
    ctx.put_image_data(&image_data, 0.0, 0.0)
        .map_err(|e| format!("{e:?}"))?;

    Ok(())
}
