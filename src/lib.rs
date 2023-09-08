// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod func_plot;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b;
}





#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}


#[wasm_bindgen]
pub fn power(canvas_id: &str, power: i32) -> Result<Chart, JsValue> {
    let map_coord = func_plot::draw(canvas_id, power).map_err(|err| err.to_string())?;
    Ok(Chart {
        convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
    })
}


#[wasm_bindgen]
impl Chart {
    
}