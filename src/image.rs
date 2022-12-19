use itertools::Itertools;
use wasm_bindgen::{prelude::*, Clamped, JsCast};

use crate::mandelbrot::FractalBuilder;

pub type RGBPixel = (u8, u8, u8);

pub const WHITE: RGBPixel = (255, 255, 255);
pub const BLACK: RGBPixel = (0, 0, 0);
// pub const BENJI_COL_2: RGBPixel = (218, 165, 32);
// pub const BENJI_COL_1: RGBPixel = (72, 61, 139);
// pub const BENJI_COL_3: RGBPixel = (75, 0, 130);

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

    pub fn fractal(width: usize, height: usize, real_range: Vec<f64>, im_range: Vec<f64>) -> Self {
        Self {
            width,
            height,
            pixels: FractalBuilder::new(width, height)
                .real_range(real_range[0]..real_range[1])
                .im_range(im_range[0]..im_range[1])
                .render(),
        }
    }

    pub fn rgba_array(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|&(r, g, b)| [r, g, b, 255])
            .collect_vec()
    }

    pub fn image_data(self) -> web_sys::ImageData {
        let data = self.rgba_array();
        web_sys::ImageData::new_with_u8_clamped_array(Clamped(&data), self.width as u32).unwrap()
    }

    /// Render the image to a canvas
    pub fn render_to_canvas(self, canvas_id: &str) {
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
        ctx.put_image_data(&self.image_data(), 0.0, 0.0).unwrap();
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
