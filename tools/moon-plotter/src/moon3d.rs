use crate::DrawResult;
use image::{DynamicImage, ImageReader};
use std::{io::Cursor, sync::OnceLock};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

const MOON_SOURCE_EMBEDDED: &[u8] = include_bytes!("../lroc_color_poles_1k.jpg");

static MOON_SOURCE: OnceLock<DynamicImage> = OnceLock::new();

fn moon_source() -> &'static DynamicImage {
    MOON_SOURCE.get_or_init(|| {
        ImageReader::with_format(Cursor::new(MOON_SOURCE_EMBEDDED), image::ImageFormat::Jpeg)
            .decode()
            .expect("Failed to decode embedded moon image")
    })
}

pub fn draw(canvas: HtmlCanvasElement, long0: f64) -> DrawResult<()> {
    let size = canvas.width().min(canvas.height());

    // Render all pixels into an RGBA buffer entirely within WASM — no per-pixel JS calls.
    let mut buf = vec![0u8; (size * size * 4) as usize];
    sphere::render_sphere(size, -long0 as f32, moon_source(), |x, y, r, g, b| {
        let idx = ((y * size + x) * 4) as usize;
        buf[idx]     = r;
        buf[idx + 1] = g;
        buf[idx + 2] = b;
        buf[idx + 3] = 255;
        Ok::<_, Box<dyn std::error::Error>>(())
    })?;

    // Push the whole buffer to the canvas in a single JS call.
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
