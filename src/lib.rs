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

#[allow(dead_code)]
impl<T> TwoDArray<T> {
    fn at(&self, x: u32, y: u32) -> Option<&T> {
        self.f.get(x as usize + y as usize * self.w as usize)
    }
    fn at_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
        self.f.get_mut(x as usize + y as usize * self.w as usize)
    }
    fn at_unchecked(&self, x: u32, y: u32) -> &T {
        &self.f[x as usize + y as usize * self.w as usize]
    }
    fn at_unchecked_mut(&mut self, x: u32, y: u32) -> &T {
        &mut self.f[x as usize + y as usize * self.w as usize]
    }
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
            f.push(random() * 2.0 - 1.0);
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

struct Perlin {
    grid_size: u32,
    z: TwoDArray<f64>,
    grads: TwoDArray<(f64, f64)>,
}

impl Perlin {
    fn initialize(grid_size: u32) -> Self {
        let vertex_count = grid_size + 1;
        let size = (vertex_count as usize) * (vertex_count as usize);
        let mut z = Vec::with_capacity(size);
        let mut grads = Vec::with_capacity(size);
        for _ in 0..size {
            // z ∈ [0,1)
            z.push(random());
            let r = 2.0 * std::f64::consts::PI * random();
            // grads has gradient vector, normalized.
            grads.push((r.cos(), r.sin()));
        }
        // make it wrap
        for i in 0..(vertex_count as usize) {
            z[vertex_count as usize * (i + 1) - 1] = z[vertex_count as usize * i];
            z[i + vertex_count as usize * (vertex_count as usize - 1)] = z[i];
        }
        Perlin {
            grid_size,
            z: TwoDArray {
                f: z,
                w: grid_size + 1,
                h: grid_size + 1,
            },
            grads: TwoDArray {
                f: grads,
                w: grid_size + 1,
                h: grid_size + 1,
            },
        }
    }
}

fn random() -> f64 {
    Math::random()
}

fn fade_psi(t: f64) -> f64 {
    6.0 * t.powi(5) - 15.0 * t.powi(4) + 10.0 * t.powi(3)
}
