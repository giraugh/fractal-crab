use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[wasm_bindgen]
pub struct FractalCanvas {
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext,
    program: WebGlProgram,
    tri_count: usize,
    view_centre: [f64; 2],
    view_size: f64,
    julia_const: Option<[f32; 2]>,
}

#[wasm_bindgen]
impl FractalCanvas {
    pub fn from_canvas(canvas: HtmlCanvasElement) -> Result<FractalCanvas, JsValue> {
        // Get canvas context
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        // Compile vertex shader
        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("./vertex.glsl"),
        )?;

        // Compile fragment shader
        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("./fragment.glsl"),
        )?;

        // Link program
        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        // Define vertices
        let vertices: [f32; 18] = [
            -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0,
            1.0, 0.0,
        ];
        let position_attribute_location = context.get_attrib_location(&program, "position");
        let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Create a typed array view into the buffer
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // Setup and bind vertex array
        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));

        // Setup the webgl attribute pointer for the vertices
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);
        context.bind_vertex_array(Some(&vao));

        Ok(Self {
            canvas,
            context,
            program,
            tri_count: vertices.len() / 3,
            view_centre: [-0.5, 0.0],
            view_size: 1.2,
            julia_const: None, // Some([0.4, 0.3]),
        })
    }

    pub fn get_view_centre(&self) -> Vec<f64> {
        Vec::from_iter(self.view_centre.iter().cloned())
    }

    pub fn get_julia_constant(&self) -> Option<Vec<f32>> {
        self.julia_const
            .map(|jc| Vec::from_iter(jc.iter().cloned()))
    }

    pub fn set_julia_constant_at(&mut self, julia_const: Option<Vec<f32>>) {
        self.julia_const = julia_const.map(|c| {
            [
                self.view_centre[0] as f32
                    + lerp(-self.view_size, self.view_size, c[0] as f64) as f32,
                self.view_centre[1] as f32
                    + lerp(self.view_size, -self.view_size, c[1] as f64) as f32,
            ]
        })
    }

    pub fn resize_viewport(&self, width: i32, height: i32) {
        self.context.viewport(0, 0, width, height);
    }

    pub fn move_view(&mut self, x_off: f64, y_off: f64) {
        self.view_centre = [
            self.view_centre[0] + self.view_size * x_off,
            self.view_centre[1] + self.view_size * y_off,
        ];
    }

    pub fn zoom_view(&mut self, factor: f64) {
        self.view_size += factor * self.view_size;
    }

    pub fn draw(&self) {
        // Get uniform locationuniforms
        let resolution_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uResolution")
            .unwrap();
        let view_centre_x_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uViewCentreX")
            .unwrap();
        let view_centre_y_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uViewCentreY")
            .unwrap();
        let view_size_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uViewSize")
            .unwrap();
        let use_julia_const_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uUseJuliaConstant")
            .unwrap();
        let julia_const_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uJuliaConstant")
            .unwrap();

        // Set uniforms
        self.context.uniform2fv_with_f32_array(
            Some(&resolution_uniform_location),
            &[self.canvas.width() as f32, self.canvas.height() as f32],
        );

        // Set view region uniforms
        self.context.uniform2fv_with_f32_array(
            Some(&view_centre_x_uniform_location),
            &split_double(self.view_centre[0]),
        );
        self.context.uniform2fv_with_f32_array(
            Some(&view_centre_y_uniform_location),
            &split_double(self.view_centre[1]),
        );
        self.context.uniform2fv_with_f32_array(
            Some(&view_size_uniform_location),
            &split_double(self.view_size),
        );

        self.context.uniform1i(
            Some(&use_julia_const_uniform_location),
            self.julia_const.is_some() as i32,
        );
        if let Some(julia_const) = self.julia_const {
            self.context
                .uniform2fv_with_f32_array(Some(&julia_const_uniform_location), &julia_const);
        }

        // Clear canvas
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // Draw quad
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.tri_count as i32);
    }
}

pub fn split_double(x: f64) -> [f32; 2] {
    let x1 = x as f32;
    let x2 = (x - (x1 as f64)) as f32;
    [x1, x2]
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
