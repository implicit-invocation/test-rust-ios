use std::rc::Rc;

use glow::*;

use super::shader_program::ShaderProgram;

pub struct VertexAttribute {
  pub name: &'static str,
  pub vertex_type: u32,
  pub num_elements: usize,
}

impl VertexAttribute {
  pub fn new(name: &'static str, vertex_type: u32, num_elements: usize) -> Self {
    Self {
      name,
      vertex_type,
      num_elements,
    }
  }
  pub fn position2() -> Self {
    VertexAttribute::new(ShaderProgram::POSITION, FLOAT, 2)
  }
  pub fn position3() -> Self {
    VertexAttribute::new(ShaderProgram::POSITION, FLOAT, 3)
  }
  pub fn texcoords() -> Self {
    VertexAttribute::new(ShaderProgram::TEXCOORDS, FLOAT, 2)
  }
  pub fn color() -> Self {
    // TODO: use u8 for this
    VertexAttribute::new(ShaderProgram::COLOR, FLOAT, 4)
  }
  pub fn color2() -> Self {
    VertexAttribute::new(ShaderProgram::COLOR2, FLOAT, 4)
  }
}

pub struct Mesh {
  pub context: Rc<Context>,
  pub vertices: Vec<f32>,
  pub vertices_buffer: Option<Buffer>,
  pub vertices_length: usize,
  pub dirty_vertices: bool,
  pub indices: Vec<u16>,
  pub indices_buffer: Option<Buffer>,
  pub indices_length: usize,
  pub dirty_indices: bool,
  pub elements_per_vertex: usize,
  pub attributes: Vec<VertexAttribute>,
  pub vao: Option<VertexArray>,
}

impl Mesh {
  pub fn new(
    context: &Rc<Context>,
    attributes: Vec<VertexAttribute>,
    max_vertices: usize,
    max_indices: usize,
  ) -> Self {
    let mut elements_per_vertex = 0;
    for attribute in &attributes {
      elements_per_vertex += attribute.num_elements;
    }
    let vertices = vec![0.0; max_vertices * elements_per_vertex];

    Self {
      context: Rc::clone(context),
      vertices,
      vertices_buffer: None,
      vertices_length: 0,
      dirty_vertices: false,
      indices: vec![0; max_indices],
      indices_buffer: None,
      indices_length: 0,
      dirty_indices: false,
      elements_per_vertex,
      attributes,
      vao: None,
    }
  }

  pub fn set_vertices_length(&mut self, length: usize) {
    self.vertices_length = length;
    self.dirty_vertices = true;
  }

  pub fn set_indices_length(&mut self, length: usize) {
    self.indices_length = length;
    self.dirty_indices = true;
  }

  pub fn set_vertices(&mut self, vertices: &[f32]) {
    self.vertices_length = vertices.len();
    for i in 0..vertices.len() {
      self.vertices[i] = vertices[i];
    }
    self.dirty_vertices = true;
  }

  pub fn set_indices(&mut self, indices: &[u16]) {
    self.indices_length = indices.len();
    for i in 0..indices.len() {
      self.indices[i] = indices[i];
    }
    self.dirty_indices = true;
  }

  pub fn draw(&mut self, shader: &ShaderProgram, primitive_type: u32) -> &Self {
    self.draw_with_offset(
      shader,
      primitive_type,
      0,
      if self.indices_length > 0 {
        self.indices_length
      } else {
        self.vertices_length / self.elements_per_vertex
      },
    );
    self
  }

  pub fn num_vertices(&self) -> usize {
    self.vertices_length / self.elements_per_vertex
  }

  pub fn draw_with_offset(
    &mut self,
    shader: &ShaderProgram,
    primitive_type: u32,
    offset: usize,
    count: usize,
  ) -> &Self {
    if self.dirty_indices || self.dirty_vertices {
      self.update();
    }
    self.bind(shader);
    let gl = &self.context;
    unsafe {
      if self.indices_length > 0 {
        gl.draw_elements(primitive_type, count as i32, UNSIGNED_SHORT, offset as i32);
      } else {
        gl.draw_arrays(primitive_type, offset as i32, count as i32);
      }
    }
    self
  }

  pub fn update(&mut self) {
    let gl = &self.context;
    if self.dirty_vertices {
      if self.vertices_buffer.is_none() {
        unsafe {
          self.vertices_buffer = Some(gl.create_buffer().unwrap());
        }
      }
      unsafe {
        let vertices_u8: &[u8] = core::slice::from_raw_parts(
          self.vertices.as_ptr() as *const u8,
          self.vertices_length * core::mem::size_of::<f32>(),
        );
        gl.bind_buffer(ARRAY_BUFFER, self.vertices_buffer);
        gl.buffer_data_u8_slice(ARRAY_BUFFER, vertices_u8, STATIC_DRAW);
        self.dirty_vertices = false;
      };
    }

    if self.dirty_indices {
      if self.indices_buffer.is_none() {
        unsafe {
          self.indices_buffer = Some(gl.create_buffer().unwrap());
        }
      }
      unsafe {
        let indices_u8: &[u8] = core::slice::from_raw_parts(
          self.indices.as_ptr() as *const u8,
          self.indices_length * core::mem::size_of::<u16>(),
        );
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, self.indices_buffer);
        gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, indices_u8, STATIC_DRAW);
        self.dirty_indices = false;
      };
    }
  }

  pub fn bind(&mut self, shader: &ShaderProgram) {
    let gl = &self.context;
    unsafe {
      gl.bind_buffer(ARRAY_BUFFER, self.vertices_buffer);
      if self.vao.is_none() {
        self.vao = Some(gl.create_vertex_array().unwrap());
        let mut offset = 0;
        let vao = self.vao.unwrap();
        gl.bind_vertex_array(Some(vao));
        for attribute in &self.attributes {
          let location = shader.get_attribute_location(attribute.name).unwrap();
          gl.enable_vertex_attrib_array(location);
          gl.vertex_attrib_pointer_f32(
            location,
            attribute.num_elements as i32,
            attribute.vertex_type,
            false,
            self.elements_per_vertex as i32 * core::mem::size_of::<f32>() as i32,
            offset as i32 * core::mem::size_of::<f32>() as i32,
          );
          offset += attribute.num_elements;
        }
      } else {
        gl.bind_vertex_array(Some(self.vao.unwrap()));
      }

      if self.indices_length > 0 {
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, self.indices_buffer);
      }
    }
  }

  pub fn unbind(&self, shader: &ShaderProgram) {
    let gl = &self.context;
    unsafe {
      for attribute in &self.attributes {
        let location = shader.get_attribute_location(&attribute.name).unwrap();
        gl.disable_vertex_attrib_array(location);
      }
      gl.bind_buffer(ARRAY_BUFFER, None);
      if self.indices_length > 0 {
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, None);
      }
    }
  }

  pub fn dispose(&mut self) {
    unsafe {
      match self.vertices_buffer {
        Some(buffer) => self.context.delete_buffer(buffer),
        None => (),
      }
      match self.indices_buffer {
        Some(buffer) => self.context.delete_buffer(buffer),
        None => (),
      }
      self.vertices_buffer = None;
      self.indices_buffer = None;
    }
  }
}

impl Drop for Mesh {
  fn drop(&mut self) {
    self.dispose();
  }
}
