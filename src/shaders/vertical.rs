pub fn get_vert_shader() {
    let vert_shader = compile_shader(
        &gl,
        GL::VERTEX_SHADER,
        r#"
  attribute vec4 aVertexPosition;
  
  void main(void) {
    gl_Position = aVertexPosition;
  }
  "#,
    )?;
    return vert_shader;
}
