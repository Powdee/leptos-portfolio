pub mod app;
pub mod components;
pub mod pages;

use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::*;

static mut CURRENT_TIME: f32 = 0.0;

fn initialize_webgl_context() -> Result<WebGlRenderingContext, JsValue> {
    let window =
        web_sys::window().expect("should have a window in this context");
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext =
        canvas.get_context("webgl")?.unwrap().dyn_into()?;

    let canvas_width = canvas.width() as f32;
    let canvas_height = canvas.height() as f32;

    gl.viewport(0, 0, canvas_width as i32, canvas_height as i32);

    // Set up the WebGL program
    let program = init_shaders(&gl);
    gl.use_program(Some(&program));

    draw_dots(&gl, &program, 10000, canvas_width, canvas_height);

    let gl_clone = gl.clone();
    // Set up resize event listener
    let closure = Closure::wrap(Box::new(move |_| {
        draw_dots(&gl_clone, &program, 10000, canvas_width, canvas_height);
    }) as Box<dyn FnMut(web_sys::Event)>);

    web_sys::window()
        .unwrap()
        .set_onresize(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    gl.clear_color(247.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(gl)
}

fn compile_shader(
    context: &WebGlRenderingContext,
    source: &str,
    shader_type: u32,
) -> WebGlShader {
    let shader = context.create_shader(shader_type).unwrap();
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        shader
    } else {
        panic!(
            "Shader compilation error: {}",
            context.get_shader_info_log(&shader).unwrap()
        );
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> WebGlProgram {
    let program = context.create_program().unwrap();
    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        program
    } else {
        panic!(
            "Program linking error: {}",
            context.get_program_info_log(&program).unwrap()
        );
    }
}

pub fn init_shaders(context: &WebGlRenderingContext) -> WebGlProgram {
    // Vertex shader source code
    let vs_source = r#"
        attribute vec4 a_position;
        void main(void) {
          gl_Position = a_position;
        }
    "#;

    // Fragment shader source code
    let fs_source = r#"
        precision mediump float;
        uniform vec4 u_color;
        uniform float u_time; 
        
        void main(void) {
        float shimmer = sin(u_time * 2.0); // Adjusted the shimmer factor
        gl_FragColor = vec4(u_color.rgb, u_color.a + shimmer);
        }
    "#;

    // Compile shaders
    let vertex_shader = compile_shader(
        context,
        vs_source,
        WebGlRenderingContext::VERTEX_SHADER,
    );
    let fragment_shader = compile_shader(
        context,
        fs_source,
        WebGlRenderingContext::FRAGMENT_SHADER,
    );

    // Link program
    link_program(context, &vertex_shader, &fragment_shader)
}

pub fn draw_dots(
    context: &WebGlRenderingContext,
    program: &WebGlProgram,
    num_dots: usize,
    canvas_width: f32,
    canvas_height: f32,
) {
    // Generate random positions for each dot
    let mut rng = rand::thread_rng();
    let mut vertices: Vec<f32> = Vec::with_capacity(num_dots * 3);

    web_sys::console::log_1(&format!("{}", canvas_width).into());
    web_sys::console::log_1(&format!("{}", canvas_height).into());
    for _ in 0..num_dots {
        // Random x, y coordinates within the canvas bounds
        let x = rng.gen_range(0.0..canvas_width);
        let y = rng.gen_range(0.0..canvas_height);

        // Normalize the coordinates to the range [0.0, 1.0]
        let normalized_x = x / canvas_width;
        let normalized_y = y / canvas_height;

        // Z-coordinate is set to 0.0 for 2D rendering
        vertices.push(normalized_x * 2.0 - 1.0); // Adjusted for full canvas width
        vertices.push(1.0 - normalized_y * 2.0); // Adjusted for full canvas height
        vertices.push(0.0);
    }

    web_sys::console::log_1(&format!("{:?}", vertices).into());

    // Create a buffer and put the vertices in it
    let buffer = context
        .create_buffer()
        .ok_or("failed to create buffer")
        .unwrap();
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let time_location =
        context.get_uniform_location(&program, "u_time").unwrap();
    unsafe {
        let vertices_array = js_sys::Float32Array::view(&vertices);
        context.uniform1f(Some(&time_location), CURRENT_TIME);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    // Get attribute and uniform locations
    let position_location =
        context.get_attrib_location(&program, "a_position") as u32;
    let color_location =
        context.get_uniform_location(&program, "u_color").unwrap();

    web_sys::console::log_1(&format!("{:?}", position_location).into());
    web_sys::console::log_1(&format!("{:?}", color_location).into());

    // Use the program
    context.use_program(Some(&program));

    // Bind the buffer
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Set up attribute pointer
    context.vertex_attrib_pointer_with_i32(
        position_location,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_location);

    let green_color: [f32; 4] = [0.0, 0.0, 0.0, 0.8];
    context.uniform4fv_with_f32_array(Some(&color_location), &green_color);

    // Draw the dots
    context.draw_arrays(WebGlRenderingContext::POINTS, 0, num_dots as i32);
}

#[wasm_bindgen]
pub fn hydrate() {
    use app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);

    // let _ = initialize_webgl_context();
}

