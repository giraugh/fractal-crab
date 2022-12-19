use std::ops::Range;

use derivative::Derivative;
use itertools::Itertools;
use num::{complex::ComplexFloat, Complex};
use wasm_bindgen::prelude::*;

use super::image::RGBPixel;
use crate::image::{lerp_pixels, BLACK, WHITE};

#[wasm_bindgen]
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct FractalBuilder {
    width: usize,
    height: usize,

    #[derivative(Default(value = "(-1.5)..(1.0)"))]
    real_range: Range<f64>,

    #[derivative(Default(value = "(-1.0..1.0)"))]
    im_range: Range<f64>,

    #[derivative(Default(value = "400"))]
    iteration_limit: usize,

    julia_constant: Option<Complex<f64>>,
}

impl FractalBuilder {
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

    pub fn julia_constant(mut self, julia_real: f64, julia_im: f64) -> Self {
        self.julia_constant = Some(Complex::new(julia_real, julia_im));
        self
    }

    /// Render the mandelbrot image to RGB pixels
    pub fn render(&self) -> Vec<RGBPixel> {
        let grad = colorgrad::sinebow();

        (0..self.width * self.height)
            .map(|i| {
                // Normalise coordinates
                let (x, y) = (i % self.width, i / self.width);
                let (u, v) = (
                    (x as f64) / (self.width as f64),
                    (y as f64) / (self.height as f64),
                );

                let mut z = Complex::new(
                    lerp_range(&self.real_range, u),
                    lerp_range(&self.im_range, v),
                );

                // Construct c value
                let c = self.julia_constant.unwrap_or(z);

                // Repeatedly perform z = z^2 + c
                let mut iteration_count = 0;
                while z.norm() < 2.0 {
                    // Staying bounded?
                    iteration_count += 1;
                    if iteration_count > self.iteration_limit {
                        // This value (pixel) is in the mandelbrot set, colour it black
                        return BLACK;
                    }

                    // Keep going...
                    z = (z * z) + c;
                }

                // This value (pixel) was not in the mandelbrot set, colour it appropriately
                let t = (iteration_count as f64) / (self.iteration_limit as f64);
                let [r, g, b, _] = grad.at(t).to_rgba8();
                (r, g, b)
            })
            .collect_vec()
    }
}

fn lerp_range(range: &Range<f64>, t: f64) -> f64 {
    assert!((0.0..=1.0).contains(&t));
    range.start + (range.end - range.start) * t
}
