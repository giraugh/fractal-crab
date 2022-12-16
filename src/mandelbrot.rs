use std::ops::Range;

use derivative::Derivative;
use itertools::Itertools;
use num::{complex::ComplexFloat, Complex, Zero};
use wasm_bindgen::prelude::*;

use super::image::RGBPixel;
use crate::image::{lerp_pixels, BLACK, WHITE};

#[wasm_bindgen]
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct MandelbrotBuilder {
    width: usize,
    height: usize,

    #[derivative(Default(value = "(-1.5)..(1.0)"))]
    real_range: Range<f64>,

    #[derivative(Default(value = "(-1.0..1.0)"))]
    im_range: Range<f64>,

    #[derivative(Default(value = "100"))]
    iteration_limit: usize,
}

impl MandelbrotBuilder {
    /// Create a new builder
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn real_range(mut self, real_range: Range<f64>) -> Self {
        self.real_range = real_range;
        self
    }

    pub fn im_range(mut self, im_range: Range<f64>) -> Self {
        self.im_range = im_range;
        self
    }

    /// Render the mandelbrot image to RGB pixels
    pub fn render(&self) -> Vec<RGBPixel> {
        (0..self.width * self.height)
            .map(|i| {
                // Normalise coordinates
                let (x, y) = (i % self.width, i / self.width);
                let (u, v) = (
                    (x as f64) / (self.width as f64),
                    (y as f64) / (self.height as f64),
                );

                // Construct c value
                let c = Complex::new(
                    lerp_range(&self.real_range, u),
                    lerp_range(&self.im_range, v),
                );

                // Repeatedly perform z = z^2 + c
                let mut z = Complex::<f64>::zero();
                let mut iteration_count = 0;
                while z.abs() < 2.0 {
                    // Staying bounded?
                    iteration_count += 1;
                    if iteration_count > self.iteration_limit {
                        // This value (pixel) is in the mandelbrot set, colour it black
                        return BLACK;
                    }

                    // Keep going...
                    z = z.powi(2) + c;
                }

                // This value (pixel) was not in the mandelbrot set, colour it appropriately
                let t = (iteration_count as f64) / (self.iteration_limit as f64);
                lerp_pixels(BLACK, WHITE, t)
            })
            .collect_vec()
    }
}

// pub struct Complex {
//     real: f64,
//     im: f64,
// }

// impl Complex {
//     const ZERO: Complex = Complex { real: 0.0, im: 0.0 };

//     pub fn new(real: f64, im: f64) -> Self {
//         Self { im, real }
//     }

//     pub fn real(&self) -> f64 {
//         self.real
//     }

//     pub fn im(&self) -> f64 {
//         self.im
//     }

//     pub fn mag(&self) -> f64 {
//         ((self.real).powi(2) + self.im.powi(2)).sqrt()
//     }
// }

fn lerp_range(range: &Range<f64>, t: f64) -> f64 {
    assert!((0.0..=1.0).contains(&t));
    range.start + (range.end - range.start) * t
}
