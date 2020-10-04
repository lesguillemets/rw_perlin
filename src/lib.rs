use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let mut data: Vec<u8> = Vec::new();
    for j in 0..height {
        for i in 0..width {
            data.push(((i * 256) / width) as u8);
            data.push(((j * 256) / height) as u8);
            data.push(0);
            data.push(255);
        }
    }
    let data = ImageData::new_with_u8_clamped_array(Clamped(&mut data), width)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}
