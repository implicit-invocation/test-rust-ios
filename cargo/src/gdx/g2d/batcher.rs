use std::rc::Rc;

use glow::*;

fn incr(i: &mut usize) -> usize {
  let tmp = *i;
  *i += 1;
  tmp
}

const QUAD_TRIANGLES: [u16; 6] = [0, 1, 2, 2, 3, 0];

use crate::gdx::misc::color::Color;

use super::{
  mesh::{Mesh, VertexAttribute},
  shader_program::ShaderProgram,
  texture::Texture,
};

pub struct PolygonBatch {
  pub context: Rc<Context>,
  pub is_drawing: bool,
  mesh: Mesh,
  shader: ShaderProgram,
  last_texture: Option<Rc<Texture>>,
  vertices_length: usize,
  indices_length: usize,

  y_down: bool,

  src_color_blend: u32,
  src_alpha_blend: u32,
  dst_color_blend: u32,
  dst_alpha_blend: u32,

  color: Color,

  projection_values: [f32; 16],

  vertices: [f32; 32],
  draw_calls: i32,
}

impl PolygonBatch {
  pub fn create(context: &Rc<Context>) -> Self {
    Self::new(context, 10920)
  }
  pub fn new(context: &Rc<Context>, max_vertices: usize) -> Self {
    // TODO: draw calls
    let shader = ShaderProgram::colored_textured(&context);
    let mesh = Mesh::new(
      &context,
      vec![
        VertexAttribute::position2(),
        VertexAttribute::color(),
        VertexAttribute::texcoords(),
      ],
      max_vertices,
      max_vertices * 3,
    );
    Self {
      context: Rc::clone(context),
      is_drawing: false,
      y_down: true,
      mesh,
      shader,
      last_texture: None,
      vertices_length: 0,
      indices_length: 0,
      projection_values: [0.0; 16],
      src_color_blend: SRC_ALPHA,
      src_alpha_blend: SRC_ALPHA,
      dst_color_blend: ONE_MINUS_SRC_ALPHA,
      dst_alpha_blend: ONE_MINUS_SRC_ALPHA,
      color: Color::WHITE,
      vertices: [0.0; 32],
      draw_calls: 0,
    }
  }

  pub fn set_color(&mut self, color: &Color) {
    self.color = color.clone();
  }

  pub fn set_color_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
    self.color.set(r, g, b, a);
  }

  pub fn set_y_down(&mut self, y_down: bool) {
    self.y_down = y_down;
  }

  pub fn set_blend_function(&mut self, src: u32, dst: u32) {
    self.src_color_blend = src;
    self.src_alpha_blend = src;
    self.dst_color_blend = dst;
    self.dst_alpha_blend = dst;
  }

  pub fn set_blend_function_separate(
    &mut self,
    src_color: u32,
    dst_color: u32,
    src_alpha: u32,
    dst_alpha: u32,
  ) {
    self.src_color_blend = src_color;
    self.src_alpha_blend = src_alpha;
    self.dst_color_blend = dst_color;
    self.dst_alpha_blend = dst_alpha;
  }

  pub fn set_projection(&mut self, projection: &[f32; 16]) {
    for i in 0..16 {
      self.projection_values[i] = projection[i];
    }
  }

  pub fn get_draw_calls(&self) -> i32 {
    self.draw_calls
  }

  pub fn begin(&mut self) {
    if self.is_drawing {
      panic!("PolygonBatch is already drawing");
    }
    self.last_texture = None;
    self.is_drawing = true;
    self.draw_calls = 0;

    self.vertices_length = 0;
    self.indices_length = 0;

    self.last_texture = None;

    self.shader.bind();
    self
      .shader
      .set_uniform_4x4f(ShaderProgram::MVP_MATRIX, &self.projection_values);
    self.shader.set_uniform_i("u_texture", 0);

    unsafe {
      self.context.enable(BLEND);
      self.context.blend_func_separate(
        self.src_color_blend,
        self.dst_color_blend,
        self.src_alpha_blend,
        self.dst_alpha_blend,
      );
    }
  }

  pub fn end(&mut self) {
    if !self.is_drawing {
      panic!("PolygonBatch is not drawing");
    }
    if self.vertices_length > 0 || self.indices_length > 0 {
      self.flush();
    }

    self.shader.unbind();
    self.last_texture = None;
    self.is_drawing = false;

    unsafe {
      self.context.disable(BLEND);
    }
  }

  pub fn flush(&mut self) {
    if self.vertices_length == 0 {
      return;
    }

    match &self.last_texture {
      Some(texture) => {
        if self.last_texture.is_none() || self.last_texture.as_deref().unwrap() != texture.as_ref()
        {
          texture.bind();
        }
        self.mesh.draw(&self.shader, TRIANGLES);
        self.draw_calls += 1;

        self.vertices_length = 0;
        self.indices_length = 0;
        self.mesh.set_indices_length(0);
        self.mesh.set_vertices_length(0);
      }
      None => {}
    }
  }

  pub fn dispose(&mut self) {
    self.mesh.dispose();
    self.shader.dispose();
  }

  pub fn draw_vertices_with_indices(
    &mut self,
    texture: &Rc<Texture>,
    vertices: &[f32],
    indices: &[u16],
  ) {
    if self.last_texture.is_none() || self.last_texture.as_deref().unwrap() != texture.as_ref() {
      self.flush();
      self.last_texture = Some(Rc::clone(texture));
    } else if self.vertices_length + vertices.len() > self.mesh.vertices.len()
      || self.indices_length + indices.len() > self.mesh.indices.len()
    {
      self.flush();
    }

    let index_start = self.mesh.num_vertices();
    for i in 0..vertices.len() {
      self.mesh.vertices[self.vertices_length + i] = vertices[i];
    }
    self.vertices_length += vertices.len();
    self.mesh.set_vertices_length(self.vertices_length);

    for i in 0..indices.len() {
      self.mesh.indices[self.indices_length + i] = indices[i] + index_start as u16;
    }
    self.indices_length += indices.len();
    self.mesh.set_indices_length(self.indices_length);
  }

  pub fn draw_vertices(&mut self, texture: &Rc<Texture>, vertices: &[f32]) {
    self.draw_vertices_with_indices(texture, vertices, &QUAD_TRIANGLES)
  }

  pub fn draw_own_vertices(&mut self, texture: &Rc<Texture>) {
    if self.last_texture.is_none() || self.last_texture.as_deref().unwrap() != texture.as_ref() {
      self.flush();
      self.last_texture = Some(Rc::clone(texture));
    } else if self.vertices_length + self.vertices.len() > self.mesh.vertices.len()
      || self.indices_length + QUAD_TRIANGLES.len() > self.mesh.indices.len()
    {
      self.flush();
    }

    let vertices = &self.vertices;
    let indices = &QUAD_TRIANGLES;
    let index_start = self.mesh.num_vertices();
    for i in 0..vertices.len() {
      self.mesh.vertices[self.vertices_length + i] = vertices[i];
    }
    self.vertices_length += vertices.len();
    self.mesh.set_vertices_length(self.vertices_length);

    for i in 0..indices.len() {
      self.mesh.indices[self.indices_length + i] = indices[i] + index_start as u16;
    }
    self.indices_length += indices.len();
    self.mesh.set_indices_length(self.indices_length);
  }

  // TODO: affine2 and draw_transformed

  pub fn draw(&mut self, texture: &Rc<Texture>, x: f32, y: f32, width: f32, height: f32) {
    self.draw_with_options(
      texture, x, y, width, height, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, false,
    )
  }

  pub fn draw_with_rot_and_scl(
    &mut self,
    texture: &Rc<Texture>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    origin_x: f32,
    origin_y: f32,
    rotation: f32,
    scale_x: f32,
    scale_y: f32,
  ) {
    self.draw_with_options(
      texture, x, y, width, height, origin_x, origin_y, rotation, scale_x, scale_y, 0.0, 1.0, 1.0,
      0.0, false,
    )
  }

  pub fn draw_with_options(
    &mut self,
    texture: &Rc<Texture>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    origin_x: f32,
    origin_y: f32,
    rotation: f32,
    scale_x: f32,
    scale_y: f32,
    ou1: f32,
    ov1: f32,
    ou2: f32,
    ov2: f32,
    rotate: bool,
  ) {
    let mut ov1 = ov1;
    let mut ov2 = ov2;
    if self.y_down {
      let tmp_v1 = ov1;
      ov1 = ov2;
      ov2 = tmp_v1;
    }
    // TODO: no width/height provided case
    let mut x1 = -origin_x;
    let mut x2 = width - origin_x;
    let mut x3 = width - origin_x;
    let mut x4 = -origin_x;

    let mut y1 = -origin_y;
    let mut y2 = -origin_y;
    let mut y3 = height - origin_y;
    let mut y4 = height - origin_y;

    if scale_x != 1. {
      x1 = x1 * scale_x;
      x2 = x2 * scale_x;
      x3 = x3 * scale_x;
      x4 = x4 * scale_x;
    }

    if scale_y != 1. {
      y1 = y1 * scale_y;
      y2 = y2 * scale_y;
      y3 = y3 * scale_y;
      y4 = y4 * scale_y;
    }

    if rotation != 0. {
      let cos = rotation.cos();
      let sin = rotation.sin();

      let rotated_x1 = cos * x1 - sin * y1;
      let rotated_y1 = sin * x1 + cos * y1;

      let rotated_x2 = cos * x2 - sin * y2;
      let rotated_y2 = sin * x2 + cos * y2;

      let rotated_x3 = cos * x3 - sin * y3;
      let rotated_y3 = sin * x3 + cos * y3;

      let rotated_x4 = rotated_x1 + (rotated_x3 - rotated_x2);
      let rotated_y4 = rotated_y3 - (rotated_y2 - rotated_y1);

      x1 = rotated_x1;
      x2 = rotated_x2;
      x3 = rotated_x3;
      x4 = rotated_x4;

      y1 = rotated_y1;
      y2 = rotated_y2;
      y3 = rotated_y3;
      y4 = rotated_y4;
    }

    x1 += x + origin_x;
    x2 += x + origin_x;
    x3 += x + origin_x;
    x4 += x + origin_x;

    y1 += y + origin_y;
    y2 += y + origin_y;
    y3 += y + origin_y;
    y4 += y + origin_y;

    let mut u1 = ou1;
    let mut v1 = ov1;
    let mut u2 = ou2;
    let mut v2 = ov2;
    let mut u3 = u2;
    let mut v3 = v1;
    let mut u4 = u1;
    let mut v4 = v2;

    if rotate {
      if self.y_down {
        u1 = ou1;
        v1 = ov2;
        u2 = ou2;
        v2 = ov1;
        u3 = ou1;
        v3 = ov1;
        u4 = ou2;
        v4 = ov2;
      } else {
        u1 = ou2;
        v1 = ov1;
        u2 = ou1;
        v2 = ov2;
        u3 = ou2;
        v3 = ov2;
        u4 = ou1;
        v4 = ov1;
      }
    }

    let quad = &mut self.vertices;
    let mut i: usize = 0;

    let color = &self.color;

    quad[incr(&mut i)] = x1;
    quad[incr(&mut i)] = y1;
    quad[incr(&mut i)] = color.r;
    quad[incr(&mut i)] = color.g;
    quad[incr(&mut i)] = color.b;
    quad[incr(&mut i)] = color.a;
    quad[incr(&mut i)] = u1;
    quad[incr(&mut i)] = v1;

    quad[incr(&mut i)] = x2;
    quad[incr(&mut i)] = y2;
    quad[incr(&mut i)] = color.r;
    quad[incr(&mut i)] = color.g;
    quad[incr(&mut i)] = color.b;
    quad[incr(&mut i)] = color.a;
    quad[incr(&mut i)] = u3;
    quad[incr(&mut i)] = v3;

    quad[incr(&mut i)] = x3;
    quad[incr(&mut i)] = y3;
    quad[incr(&mut i)] = color.r;
    quad[incr(&mut i)] = color.g;
    quad[incr(&mut i)] = color.b;
    quad[incr(&mut i)] = color.a;
    quad[incr(&mut i)] = u2;
    quad[incr(&mut i)] = v2;

    quad[incr(&mut i)] = x4;
    quad[incr(&mut i)] = y4;
    quad[incr(&mut i)] = color.r;
    quad[incr(&mut i)] = color.g;
    quad[incr(&mut i)] = color.b;
    quad[incr(&mut i)] = color.a;
    quad[incr(&mut i)] = u4;
    quad[incr(&mut i)] = v4;

    self.draw_own_vertices(texture);
  }
}

impl Drop for PolygonBatch {
  fn drop(&mut self) {
    self.dispose();
  }
}
