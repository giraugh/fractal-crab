use itertools::Itertools;
use wasm_bindgen::{prelude::*, JsCast};

use crate::mandelbrot::MandelbrotBuilder;

pub type RGBPixel = (u8, u8, u8);

pub const WHITE: RGBPixel = (255, 255, 255);
pub const BLACK: RGBPixel = (0, 0, 0);

#[wasm_bindgen]
pub struct Image {
    pixels: Vec<RGBPixel>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Image {
    /// Get pixel index of cartesian coord
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Generate a new image with random black/white pixels
    pub fn random(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![WHITE; width * height]
                .iter()
                .map(|_| if rand::random::<bool>() { WHITE } else { BLACK })
                .collect_vec(),
        }
    }

    pub fn mandelbrot(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: MandelbrotBuilder::new(width, height).render(),
        }
    }

    // TODO: better error handling
    /// Render the image to a canvas
    pub fn render_to_canvas(&self, canvas_id: &str) {
        // Get canvas
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        // Set canvas size
        canvas.set_width(self.width as _);
        canvas.set_height(self.height as _);

        // Get canvas context
        let ctx = canvas
            .get_context("2d")
            .unwrap() // can fail
            .unwrap() // may be null
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap(); // can fail

        // Render pixels
        for y in 0..self.height {
            for x in 0..self.width {
                let (r, g, b) = self.pixels[self.index(x, y)];
                let fill = format!("rgb({r}, {g}, {b})");
                ctx.set_fill_style(&JsValue::from_str(&fill));
                ctx.fill_rect(x as _, y as _, 1.0, 1.0);
            }
        }
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    assert!((0.0..=1.0).contains(&t));
    a + (b - a) * t
}

pub fn lerp_pixels(a: RGBPixel, b: RGBPixel, t: f64) -> RGBPixel {
    (
        lerp(a.0 as f64, b.0 as f64, t) as u8,
        lerp(a.1 as f64, b.1 as f64, t) as u8,
        lerp(a.2 as f64, b.2 as f64, t) as u8,
    )
}
