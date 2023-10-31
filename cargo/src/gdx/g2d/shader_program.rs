use std::rc::Rc;

use const_format::formatcp;
use glow::*;

const MVP_MATRIX: &'static str = "u_projTrans";
const POSITION: &'static str = "a_position";
const COLOR: &'static str = "a_color";
const COLOR2: &'static str = "a_color2";
const TEXCOORDS: &'static str = "a_texCoord0";
const SAMPLER: &'static str = "u_texture";
const NORMAL: &'static str = "a_normal";
const TANGENT: &'static str = "a_tangent";
const BINORMAL: &'static str = "a_binormal";
const BONE_WEIGHT: &'static str = "a_boneWeight";

pub struct ShaderProgram {
  pub gl: Rc<Context>,
  pub vs_source: &'static str,
  pub vs: Shader,
  pub fs_source: &'static str,
  pub fs: Shader,
  pub program: Program,
}

pub const WHITE_VS: &str = formatcp!(
  r#"#version 100
attribute vec4 {};

void main() {{
  gl_Position = {};
}}"#,
  ShaderProgram::POSITION,
  ShaderProgram::POSITION
);

pub const WHITE_FRAG: &str = formatcp!(
  r#"#version 100
#ifdef GL_ES
  #define LOWP lowp
  precision mediump float;
#else
  #define LOWP
#endif

void main () {{
  gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}}"#
);

pub const COLOR_VS: &str = formatcp!(
  r#"#version 100
attribute vec4 {POSITION};
attribute vec4 {COLOR};

varying vec4 v_color;

void main() {{
  v_color = {COLOR};
  gl_Position = {POSITION};
}}"#
);

pub const COLOR_FRAG: &str = formatcp!(
  r#"#version 100
#ifdef GL_ES
  #define LOWP lowp
  precision mediump float;
#else
  #define LOWP
#endif

varying LOWP vec4 v_color;

void main () {{
  gl_FragColor = v_color;
}}"#
);

pub const TEXTURED_VS: &str = formatcp!(
  r#"#version 100
attribute vec4 {POSITION};
attribute vec2 {TEXCOORDS};
uniform mat4 {MVP_MATRIX};

varying vec2 v_texCoords;

void main() {{
  v_texCoords = {TEXCOORDS};
  gl_Position = {MVP_MATRIX} * {POSITION};
}}
"#
);

pub const TEXTURED_FS: &str = formatcp!(
  r#"#version 100
#ifdef GL_ES
  #define LOWP lowp
  precision mediump float;
#else
  #define LOWP
#endif
varying vec2 v_texCoords;
uniform sampler2D u_texture;

void main () {{
  gl_FragColor = texture2D(u_texture, v_texCoords);
}}
"#
);

pub const COLORED_TEXTURED_VS: &str = formatcp!(
  r#"#version 100
attribute vec4 {POSITION};
attribute vec4 {COLOR};
attribute vec2 {TEXCOORDS};
uniform mat4 {MVP_MATRIX};

varying vec4 v_color;
varying vec2 v_texCoords;

void main() {{
  v_color = {COLOR};
  v_color.a = v_color.a * (256.0/255.0);
  v_texCoords = {TEXCOORDS};
  gl_Position = {MVP_MATRIX} * {POSITION};
}}
"#
);

pub const COLORED_TEXTURED_FS: &str = formatcp!(
  r#"#version 100
#ifdef GL_ES
  #define LOWP lowp
  precision mediump float;
#else
  #define LOWP
#endif
varying LOWP vec4 v_color;
varying vec2 v_texCoords;
uniform sampler2D u_texture;

void main () {{
  gl_FragColor = v_color * texture2D(u_texture, v_texCoords);
}}
"#
);

impl ShaderProgram {
  pub const MVP_MATRIX: &'static str = MVP_MATRIX;
  pub const POSITION: &'static str = POSITION;
  pub const COLOR: &'static str = COLOR;
  pub const COLOR2: &'static str = COLOR2;
  pub const TEXCOORDS: &'static str = TEXCOORDS;
  pub const SAMPLER: &'static str = SAMPLER;
  pub const NORMAL: &'static str = NORMAL;
  pub const TANGENT: &'static str = TANGENT;
  pub const BINORMAL: &'static str = BINORMAL;
  pub const BONE_WEIGHT: &'static str = BONE_WEIGHT;

  pub fn compile_shader(
    gl: &Context,
    source: &'static str,
    shader_type: u32,
  ) -> Result<Shader, String> {
    unsafe {
      let shader = gl.create_shader(shader_type).unwrap();
      gl.shader_source(shader, source);
      gl.compile_shader(shader);
      if !gl.get_shader_compile_status(shader) {
        let info_log = gl.get_shader_info_log(shader);
        return Err(info_log);
      }
      Ok(shader)
    }
  }

  pub fn compile_program(gl: &Context, vs: &Shader, fs: &Shader) -> Result<Program, String> {
    unsafe {
      let program = gl.create_program().unwrap();
      gl.attach_shader(program, *vs);
      gl.attach_shader(program, *fs);
      gl.link_program(program);
      if !gl.get_program_link_status(program) {
        let info_log = gl.get_program_info_log(program);
        return Err(info_log);
      }
      Ok(program)
    }
  }

  pub fn new(gl: &Rc<Context>, vs_source: &'static str, fs_source: &'static str) -> Self {
    let vs = Self::compile_shader(&gl, vs_source, VERTEX_SHADER).unwrap();
    let fs = Self::compile_shader(&gl, fs_source, FRAGMENT_SHADER).unwrap();
    let program = ShaderProgram::compile_program(&gl, &vs, &fs).unwrap();

    Self {
      gl: Rc::clone(gl),
      vs_source,
      vs,
      fs_source,
      fs,
      program,
    }
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.use_program(Some(self.program));
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.use_program(None);
    }
  }

  pub fn set_uniform_i(&self, name: &str, value: i32) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self.gl.uniform_1_i32(Some(&location), value);
    }
  }

  pub fn set_uniform_iv(&self, name: &str, values: &[i32]) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self.gl.uniform_1_i32_slice(Some(&location), values);
    }
  }

  pub fn set_uniform_f(&self, name: &str, value: f32) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self.gl.uniform_1_f32(Some(&location), value);
    }
  }

  pub fn set_uniform_2f(&self, name: &str, value1: f32, value2: f32) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self.gl.uniform_2_f32(Some(&location), value1, value2);
    }
  }

  pub fn set_uniform_3f(&self, name: &str, value1: f32, value2: f32, value3: f32) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self
        .gl
        .uniform_3_f32(Some(&location), value1, value2, value3);
    }
  }

  pub fn set_uniform_f_with_location(&self, location: &UniformLocation, value: f32) {
    unsafe {
      self.gl.uniform_1_f32(Some(location), value);
    }
  }

  pub fn set_uniform_3f_with_location(
    &self,
    location: &UniformLocation,
    value1: f32,
    value2: f32,
    value3: f32,
  ) {
    unsafe {
      self
        .gl
        .uniform_3_f32(Some(location), value1, value2, value3);
    }
  }

  pub fn set_uniform_4f(&self, name: &str, value1: f32, value2: f32, value3: f32, value4: f32) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self
        .gl
        .uniform_4_f32(Some(&location), value1, value2, value3, value4);
    }
  }

  pub fn set_uniform_2x2f(&self, name: &str, values: &[f32]) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self
        .gl
        .uniform_matrix_2_f32_slice(Some(&location), false, values);
    }
  }

  pub fn set_uniform_3x3f(&self, name: &str, values: &[f32]) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self
        .gl
        .uniform_matrix_3_f32_slice(Some(&location), false, values);
    }
  }

  pub fn set_uniform_4x4f(&self, name: &str, values: &[f32]) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self
        .gl
        .uniform_matrix_4_f32_slice(Some(&location), false, values);
    }
  }

  pub fn set_uniform_4x4f_with_location(&self, location: &UniformLocation, values: &[f32]) {
    unsafe {
      self
        .gl
        .uniform_matrix_4_f32_slice(Some(location), false, values);
    }
  }

  pub fn set_uniform_3fv(&self, name: &str, values: &[f32]) {
    unsafe {
      let location = self.gl.get_uniform_location(self.program, name).unwrap();
      self.gl.uniform_3_f32_slice(Some(&location), values);
    }
  }

  // TODO: pendatic stuff
  pub fn get_uniform_location(&self, name: &str) -> Option<UniformLocation> {
    unsafe { self.gl.get_uniform_location(self.program, name) }
  }

  pub fn get_attribute_location(&self, name: &str) -> Option<u32> {
    unsafe { self.gl.get_attrib_location(self.program, name) }
  }

  pub fn dispose(&self) {
    unsafe {
      self.gl.delete_shader(self.vs);
      self.gl.delete_shader(self.fs);
      self.gl.delete_program(self.program);
    }
  }

  pub fn colored_textured(gl: &Rc<Context>) -> Self {
    Self::new(gl, COLORED_TEXTURED_VS, COLORED_TEXTURED_FS)
  }

  pub fn textured(gl: &Rc<Context>) -> Self {
    Self::new(gl, TEXTURED_VS, TEXTURED_FS)
  }

  pub fn colored(gl: &Rc<Context>) -> Self {
    Self::new(gl, COLOR_VS, COLOR_FRAG)
  }

  pub fn white(gl: &Rc<Context>) -> Self {
    Self::new(gl, WHITE_VS, WHITE_FRAG)
  }
}

impl Drop for ShaderProgram {
  fn drop(&mut self) {
    self.dispose();
  }
}
