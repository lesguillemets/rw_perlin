use js_sys::Math;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let f = Field::new_white_noise(width, height);
    f.to_canvas(ctx)
}

struct TwoDArray<T> {
    f: Vec<T>,
    w: u32,
    h: u32,
}

// generics are (still) tricky to handle here
type It = f64; // [-1,1]
type Field = TwoDArray<It>;

impl Field {
    fn to_canvas(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let mut data: Vec<u8> = Vec::with_capacity(self.w as usize * self.h as usize);
        for value in &self.f {
            add_to_colour_data(*value, &mut data);
        }
        let data = ImageData::new_with_u8_clamped_array(Clamped(&mut data), self.w)?;
        ctx.put_image_data(&data, 0.0, 0.0)
    }

    fn new_white_noise(width: u32, height: u32) -> Self {
        let size = width as usize * height as usize;
        let mut f: Vec<f64> = Vec::with_capacity(size);
        for _ in 0..size {
            f.push(Math::random() * 2.0 - 1.0);
        }
        TwoDArray {
            f,
            w: width,
            h: height,
        }
    }
}

fn add_to_colour_data(value: It, data: &mut Vec<u8>) {
    for _ in &['r', 'g', 'b'] {
        data.push((value * 255.0).floor() as u8);
    }
    data.push(255);
}
