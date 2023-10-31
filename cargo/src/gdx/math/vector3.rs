use super::mat3::*;
use super::mat4::*;

pub const VEC3_X: Vec3 = Vec3 {
  x: 1.0,
  y: 0.0,
  z: 0.0,
};

pub const VEC3_Y: Vec3 = Vec3 {
  x: 0.0,
  y: 1.0,
  z: 0.0,
};

pub const VEC3_Z: Vec3 = Vec3 {
  x: 0.0,
  y: 0.0,
  z: 1.0,
};

pub const VEC3_ZERO: Vec3 = Vec3 {
  x: 0.0,
  y: 0.0,
  z: 0.0,
};

#[derive(Debug)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  pub fn len_of(x: f32, y: f32, z: f32) -> f32 {
    (x * x + y * y + z * z).sqrt()
  }

  pub fn zero() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }

  pub fn cpy(&self) -> Self {
    Self {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }

  pub fn is_zero(&self) -> bool {
    self.x == 0.0 && self.y == 0.0 && self.z == 0.0
  }

  pub fn get_x(&self) -> f32 {
    self.x
  }

  pub fn get_y(&self) -> f32 {
    self.y
  }

  pub fn get_z(&self) -> f32 {
    self.z
  }

  pub fn dst2(&self, vec3: &Vec3) -> f32 {
    let x_dif = self.x - vec3.x;
    let y_dif = self.y - vec3.y;
    let z_dif = self.z - vec3.z;
    x_dif * x_dif + y_dif * y_dif + z_dif * z_dif
  }

  pub fn dst(&self, vec3: &Vec3) -> f32 {
    self.dst2(vec3).sqrt()
  }

  pub fn set_from(&mut self, vec3: &Vec3) -> &mut Self {
    self.x = vec3.x;
    self.y = vec3.y;
    self.z = vec3.z;
    self
  }

  pub fn set(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
    self.x = x;
    self.y = y;
    self.z = z;
    self
  }

  pub fn add(&mut self, vec3: &Vec3) -> &mut Self {
    self.x += vec3.x;
    self.y += vec3.y;
    self.z += vec3.z;
    self
  }

  pub fn add_values(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
    self.x += x;
    self.y += y;
    self.z += z;
    self
  }

  pub fn sub(&mut self, vec3: &Vec3) -> &mut Self {
    self.x -= vec3.x;
    self.y -= vec3.y;
    self.z -= vec3.z;
    self
  }

  pub fn sub_values(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
    self.x -= x;
    self.y -= y;
    self.z -= z;
    self
  }

  pub fn scl(&mut self, scalar: f32) -> &mut Self {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
    self
  }

  pub fn scl_vec(&mut self, vec3: &Vec3) -> &mut Self {
    self.x *= vec3.x;
    self.y *= vec3.y;
    self.z *= vec3.z;
    self
  }

  pub fn scl_values(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
    self.x *= x;
    self.y *= y;
    self.z *= z;
    self
  }

  pub fn normalize(&mut self) -> &mut Self {
    let len2 = self.x * self.x + self.y * self.y + self.z * self.z;
    if len2 != 0.0 && len2 != 1.0 {
      let len = len2.sqrt();
      self.x /= len;
      self.y /= len;
      self.z /= len;
    }
    self
  }

  pub fn dot(&self, vec3: &Vec3) -> f32 {
    self.x * vec3.x + self.y * vec3.y + self.z * vec3.z
  }

  pub fn cross(&mut self, vec3: &Vec3) -> &mut Self {
    let x = self.y * vec3.z - self.z * vec3.y;
    let y = self.z * vec3.x - self.x * vec3.z;
    let z = self.x * vec3.y - self.y * vec3.x;
    self.x = x;
    self.y = y;
    self.z = z;
    self
  }

  pub fn len2(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn len(&self) -> f32 {
    self.len2().sqrt()
  }

  pub fn multiply_mat3(&mut self, mat3: &Mat3) -> &mut Self {
    let x = self.x * mat3.values[MAT3_M00]
      + self.y * mat3.values[MAT3_M10]
      + self.z * mat3.values[MAT3_M20];
    let y = self.x * mat3.values[MAT3_M01]
      + self.y * mat3.values[MAT3_M11]
      + self.z * mat3.values[MAT3_M21];
    let z = self.x * mat3.values[MAT3_M02]
      + self.y * mat3.values[MAT3_M12]
      + self.z * mat3.values[MAT3_M22];
    self.x = x;
    self.y = y;
    self.z = z;
    self
  }

  pub fn multiply_mat4(&mut self, mat4: &Mat4) -> &mut Self {
    let x = self.x * mat4.values[MAT4_M00]
      + self.y * mat4.values[MAT4_M10]
      + self.z * mat4.values[MAT4_M20]
      + mat4.values[MAT4_M30];
    let y = self.x * mat4.values[MAT4_M01]
      + self.y * mat4.values[MAT4_M11]
      + self.z * mat4.values[MAT4_M21]
      + mat4.values[MAT4_M31];
    let z = self.x * mat4.values[MAT4_M02]
      + self.y * mat4.values[MAT4_M12]
      + self.z * mat4.values[MAT4_M22]
      + mat4.values[MAT4_M32];
    self.x = x;
    self.y = y;
    self.z = z;
    self
  }
}
