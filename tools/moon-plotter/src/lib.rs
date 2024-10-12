use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod checker3d;
mod moon3d;
mod sphere;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    pub fn moon3d(canvas: HtmlCanvasElement, long0: f64) -> Result<(), JsValue> {
        moon3d::draw(canvas, long0).map_err(|err| err.to_string())?;
        Ok(())
    }

    pub fn checker3d(canvas: HtmlCanvasElement, long0: f64) -> Result<(), JsValue> {
        checker3d::draw(canvas, long0).map_err(|err| err.to_string())?;
        Ok(())
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}
