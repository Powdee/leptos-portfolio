pub mod app;
pub mod components;
pub mod pages;
pub mod sections;
pub mod utils;

pub mod types {
    pub mod project;
}

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
    // let program = init_shaders(&gl);
    // gl.use_program(Some(&program));
    gl.clear_color(247.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    draw_dots(&gl, canvas_width, canvas_height)?;

    Ok(gl)
}

fn init_shaders(gl: &WebGlRenderingContext) -> Result<WebGlProgram, String> {
    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec2 a_position;
        void main() {
            gl_Position = vec4(a_position, 0.0, 1.0);
            gl_PointSize = 10.0;
        }
        "#,
    )?;

    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;
        void main() {
            gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0); // Red color
        }
        "#,
    )?;

    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    Ok(program)
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    gl: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader program"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program")))
    }
}

fn draw_dots(
    gl: &WebGlRenderingContext,
    canvas_width: f32,
    canvas_height: f32,
) -> Result<(), JsValue> {
    let mut rng = rand::thread_rng();

    // Generate random positions for dots
    let mut positions = Vec::new();
    for _ in 0..10 {
        let x = rng.gen_range(0.0..canvas_width);
        let y = rng.gen_range(0.0..canvas_height);
        positions.push(x);
        positions.push(y);
    }

    // Create and bind buffer
    let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Transfer the positions data to the buffer
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&positions);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    // Configure and enable vertex attribute for position
    gl.vertex_attrib_pointer_with_i32(
        0,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(0);

    let program = init_shaders(&gl)?;
    gl.use_program(Some(&program));

    // Draw the dots
    gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 10);

    Ok(())
}

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
    // let _ = initialize_webgl_context();
}

