use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[wasm_bindgen]
pub struct FractalCanvas {
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext,
    program: WebGlProgram,
    tri_count: usize,
    view_min: [f64; 2],
    view_max: [f64; 2],
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
            view_max: [-2.0, -1.3],
            view_min: [0.8, 1.3],
        })
    }

    pub fn move_view(&mut self, x_off: f64, y_off: f64) {
        let view_width = (self.view_max[0] - self.view_min[0]).abs();
        let view_height = (self.view_max[1] - self.view_min[1]).abs();
        let x_delta = x_off * view_width;
        let y_delta = y_off * view_height;
        self.view_min = [self.view_min[0] + x_delta, self.view_min[1] + y_delta];
        self.view_max = [self.view_max[0] + x_delta, self.view_max[1] + y_delta];
    }

    pub fn zoom_view(&mut self, factor: f64) {
        let mid_x = (self.view_min[0] + self.view_max[0]) / 2.0;
        let mid_y = (self.view_min[1] + self.view_max[1]) / 2.0;
        self.view_min = [
            lerp(self.view_min[0], mid_x, factor),
            lerp(self.view_min[1], mid_y, factor),
        ];
        self.view_max = [
            lerp(self.view_max[0], mid_x, factor),
            lerp(self.view_max[1], mid_y, factor),
        ];
    }

    pub fn draw(&self) {
        // Get uniform locationuniforms
        let resolution_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uResolution")
            .unwrap();
        let view_min_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uViewMin")
            .unwrap();
        let view_max_uniform_location = self
            .context
            .get_uniform_location(&self.program, "uViewMax")
            .unwrap();

        // Set uniforms
        self.context.uniform2fv_with_f32_array(
            Some(&resolution_uniform_location),
            &[self.canvas.width() as f32, self.canvas.height() as f32],
        );

        // Set view region uniforms
        let vmin = [self.view_min[0] as f32, self.view_min[1] as f32];
        let vmax = [self.view_max[0] as f32, self.view_max[1] as f32];
        self.context
            .uniform2fv_with_f32_array(Some(&view_min_uniform_location), &vmin);
        self.context
            .uniform2fv_with_f32_array(Some(&view_max_uniform_location), &vmax);

        // Clear canvas
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // Draw quad
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.tri_count as i32);
    }
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
