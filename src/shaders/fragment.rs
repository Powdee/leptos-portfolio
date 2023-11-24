pub fn get_frag_shader() {
    let frag_shader = compile_shader(
        &gl,
        GL::FRAGMENT_SHADER,
        r#"
  precision mediump float;
  
  void main(void) {
    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0); // Black color
  }
  "#,
    )?;

    return frag_shader;
}
