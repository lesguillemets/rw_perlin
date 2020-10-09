mod array;
mod perlin;

use crate::array::TwoDArray;
use crate::perlin::Perlin;
use js_sys::{Date, Math};
use std::cmp::max;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let grids: u32 = 25;
    let d0 = Date::now();
    let perlin = Perlin::initialize(grids, None);
    let d1 = Date::now();
    console::log_1(&JsValue::from_str(&format!(
        "Took {:?} to initialize",
        d1 - d0
    )));
    let mut v: Vec<f64> = Vec::with_capacity((width * height) as usize);
    let scale = (grids as f64) / (max(width, height) as f64);
    for y in 0..height {
        for x in 0..width {
            // v.push(perlin.at(scale * x as f64, scale * y as f64).unwrap_or(0.0));
            let mut here = 0.0;
            for oct in 1..5 {
                let oct = oct as f64;
                here += oct
                    * perlin
                        .at(scale / oct * x as f64, scale / oct * y as f64)
                        .unwrap_or(0.0)
            }
            v.push(here);
        }
    }
    let d2 = Date::now();
    console::log_1(&JsValue::from_str(&format!("Took {:?} to calc", d2 - d1)));
    let f = Field {
        f: v,
        w: width,
        h: height,
    };
    let result = f.to_canvas(ctx);
    let d3 = Date::now();
    console::log_1(&JsValue::from_str(&format!("Took {:?} to render", d3 - d2)));
    result
}

// generics are (still) tricky to handle here
type It = f64; // [-1,1]
type Field = TwoDArray<It>;

impl Field {
    fn to_canvas(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let mut data: Vec<u8> = Vec::with_capacity(self.w as usize * self.h as usize);
        // normalise
        // f64 implements partialord only?
        let mut min = self.f[0];
        let mut max = self.f[0];
        for &value in &self.f {
            if min > value {
                min = value;
            }
            if max < value {
                max = value;
            }
        }
        for value in &self.f {
            add_to_colour_data((*value - min) / (max - min), &mut data);
        }
        let data = ImageData::new_with_u8_clamped_array(Clamped(&mut data), self.w)?;
        ctx.put_image_data(&data, 0.0, 0.0)
    }

    #[allow(dead_code)]
    fn new_white_noise(width: u32, height: u32) -> Self {
        let size = width as usize * height as usize;
        let mut f: Vec<f64> = Vec::with_capacity(size);
        for _ in 0..size {
            f.push(random() * 2.0 - 1.0);
        }
        TwoDArray {
            f,
            w: width,
            h: height,
        }
    }
}

#[inline]
fn add_to_colour_data(value: It, data: &mut Vec<u8>) {
    for _ in &['r', 'g', 'b'] {
        data.push((value * 255.0).floor() as u8);
    }
    data.push(255);
}

fn random() -> f64 {
    Math::random()
}
