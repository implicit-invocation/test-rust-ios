use std::rc::Rc;

use glow::*;

pub struct ImageData<'a> {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) data: &'a [u8],
}

#[derive(Debug)]
pub struct Texture {
  pub gl: Rc<Context>,
  pub texture: NativeTexture,
  pub width: u32,
  pub height: u32,
}

impl Texture {
  // TODO: cover all options, make an option struct too
  pub fn new_white_texture(gl: &Rc<Context>) -> Rc<Self> {
    let data = ImageData {
      width: 1,
      height: 1,
      data: &[255, 255, 255, 255],
    };
    Self::new(gl, data)
  }
  pub fn new(gl: &Rc<Context>, data: ImageData) -> Rc<Self> {
    unsafe {
      let texture = gl.create_texture().unwrap();
      gl.bind_texture(TEXTURE_2D, Some(texture));
      gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
      gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
      gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
      gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
      gl.tex_image_2d(
        TEXTURE_2D,
        0,
        RGBA as i32,
        data.width as i32,
        data.height as i32,
        0,
        RGBA,
        UNSIGNED_BYTE,
        Some(&data.data),
      );
      gl.generate_mipmap(TEXTURE_2D);
      Rc::new(Self {
        gl: Rc::clone(gl),
        texture,
        width: data.width,
        height: data.height,
      })
    }
  }

  pub fn bind(&self) {
    self.bind_to(0);
  }

  pub fn bind_to(&self, unit: u32) {
    unsafe {
      self.gl.active_texture(TEXTURE0 + unit);
      self.gl.bind_texture(TEXTURE_2D, Some(self.texture));
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.bind_texture(TEXTURE_2D, None);
    }
  }

  pub fn dispose(&self) {
    unsafe {
      self.gl.delete_texture(self.texture);
    }
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    self.dispose();
  }
}

impl PartialEq for Texture {
  fn eq(&self, other: &Self) -> bool {
    self.texture == other.texture
  }
}
