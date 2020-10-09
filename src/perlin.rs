use crate::TwoDArray;
use js_sys::Math;

pub struct Perlin {
    grid_size: u32,
    z: TwoDArray<f64>,
    grads: TwoDArray<(f64, f64)>,
}

impl Perlin {
    pub fn initialize(grid_size: u32) -> Self {
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

    pub fn at(&self, x: f64, y: f64) -> Option<f64> {
        if x < 0.0 || y < 0.0 || x > (self.grid_size as f64) || y > (self.grid_size as f64) {
            return None;
        }
        let dx = x - x.floor();
        let dy = y - y.floor();
        let x0: u32 = x.floor() as u32;
        let y0: u32 = y.floor() as u32;
        let mut dep: f64 = 0.0;
        for &(cx, cy) in &[(0u32, 0u32), (1, 0), (0, 1), (1, 1)] {
            let grad = *self.grads.at_unchecked(x0 + cx, y0 + cy);
            let v = (dx - cx as f64, dy - cy as f64);
            // add the random height
            let z: f64 = *self.z.at_unchecked(x0 + cx, y0 + cy);

            dep += fade_psi(((1 - cx) as f64 - dx).abs())
                * fade_psi(((1 - cy) as f64 - dy).abs())
                * (dot_prod(&v, &grad) + z * RANDOM_Z_WEIGHT);
        }
        Some(dep)
    }
}

fn dot_prod((x0, y0): &(f64, f64), (x1, y1): &(f64, f64)) -> f64 {
    x0 * x1 + y0 * y1
}

const RANDOM_Z_WEIGHT: f64 = 0.5;
fn fade_psi(t: f64) -> f64 {
    6.0 * t.powi(5) - 15.0 * t.powi(4) + 10.0 * t.powi(3)
}

fn random() -> f64 {
    Math::random()
}
