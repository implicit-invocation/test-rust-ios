use super::{mat4::*, vector2::Vec2, vector3::Vec3};

pub const MAT3_M00: usize = 0;
pub const MAT3_M01: usize = 3;
pub const MAT3_M02: usize = 6;
pub const MAT3_M10: usize = 1;
pub const MAT3_M11: usize = 4;
pub const MAT3_M12: usize = 7;
pub const MAT3_M20: usize = 2;
pub const MAT3_M21: usize = 5;
pub const MAT3_M22: usize = 8;

pub struct Mat3 {
  pub values: [f32; 9],
  tmp: [f32; 9],
}

impl Mat3 {
  pub fn new() -> Self {
    let mut mat3 = Self {
      values: [0.0; 9],
      tmp: [0.0; 9],
    };
    mat3.idt();
    mat3
  }

  pub fn idt(&mut self) -> &mut Self {
    self.values[MAT3_M00] = 1.0;
    self.values[MAT3_M10] = 0.0;
    self.values[MAT3_M20] = 0.0;
    self.values[MAT3_M01] = 0.0;
    self.values[MAT3_M11] = 1.0;
    self.values[MAT3_M21] = 0.0;
    self.values[MAT3_M02] = 0.0;
    self.values[MAT3_M12] = 0.0;
    self.values[MAT3_M22] = 1.0;

    self
  }

  pub fn mul_with_mat3_values(&mut self, val: [f32; 9]) -> &mut Self {
    let v00 = self.values[MAT3_M00] * val[MAT3_M00]
      + self.values[MAT3_M01] * val[MAT3_M10]
      + self.values[MAT3_M02] * val[MAT3_M20];
    let v01 = self.values[MAT3_M00] * val[MAT3_M01]
      + self.values[MAT3_M01] * val[MAT3_M11]
      + self.values[MAT3_M02] * val[MAT3_M21];
    let v02 = self.values[MAT3_M00] * val[MAT3_M02]
      + self.values[MAT3_M01] * val[MAT3_M12]
      + self.values[MAT3_M02] * val[MAT3_M22];
    let v10 = self.values[MAT3_M10] * val[MAT3_M00]
      + self.values[MAT3_M11] * val[MAT3_M10]
      + self.values[MAT3_M12] * val[MAT3_M20];
    let v11 = self.values[MAT3_M10] * val[MAT3_M01]
      + self.values[MAT3_M11] * val[MAT3_M11]
      + self.values[MAT3_M12] * val[MAT3_M21];
    let v12 = self.values[MAT3_M10] * val[MAT3_M02]
      + self.values[MAT3_M11] * val[MAT3_M12]
      + self.values[MAT3_M12] * val[MAT3_M22];
    let v20 = self.values[MAT3_M20] * val[MAT3_M00]
      + self.values[MAT3_M21] * val[MAT3_M10]
      + self.values[MAT3_M22] * val[MAT3_M20];
    let v21 = self.values[MAT3_M20] * val[MAT3_M01]
      + self.values[MAT3_M21] * val[MAT3_M11]
      + self.values[MAT3_M22] * val[MAT3_M21];
    let v22 = self.values[MAT3_M20] * val[MAT3_M02]
      + self.values[MAT3_M21] * val[MAT3_M12]
      + self.values[MAT3_M22] * val[MAT3_M22];

    self.values[MAT3_M00] = v00;
    self.values[MAT3_M10] = v10;
    self.values[MAT3_M20] = v20;
    self.values[MAT3_M01] = v01;
    self.values[MAT3_M11] = v11;
    self.values[MAT3_M21] = v21;
    self.values[MAT3_M02] = v02;
    self.values[MAT3_M12] = v12;
    self.values[MAT3_M22] = v22;

    self
  }

  pub fn mul_with_mat3(&mut self, mat: &Self) -> &mut Self {
    self.mul_with_mat3_values(mat.values)
  }

  pub fn mul_left(&mut self, mat: &Self) -> &mut Self {
    let v00 = mat.values[MAT3_M00] * self.values[MAT3_M00]
      + mat.values[MAT3_M01] * self.values[MAT3_M10]
      + mat.values[MAT3_M02] * self.values[MAT3_M20];
    let v01 = mat.values[MAT3_M00] * self.values[MAT3_M01]
      + mat.values[MAT3_M01] * self.values[MAT3_M11]
      + mat.values[MAT3_M02] * self.values[MAT3_M21];
    let v02 = mat.values[MAT3_M00] * self.values[MAT3_M02]
      + mat.values[MAT3_M01] * self.values[MAT3_M12]
      + mat.values[MAT3_M02] * self.values[MAT3_M22];
    let v10 = mat.values[MAT3_M10] * self.values[MAT3_M00]
      + mat.values[MAT3_M11] * self.values[MAT3_M10]
      + mat.values[MAT3_M12] * self.values[MAT3_M20];
    let v11 = mat.values[MAT3_M10] * self.values[MAT3_M01]
      + mat.values[MAT3_M11] * self.values[MAT3_M11]
      + mat.values[MAT3_M12] * self.values[MAT3_M21];
    let v12 = mat.values[MAT3_M10] * self.values[MAT3_M02]
      + mat.values[MAT3_M11] * self.values[MAT3_M12]
      + mat.values[MAT3_M12] * self.values[MAT3_M22];
    let v20 = mat.values[MAT3_M20] * self.values[MAT3_M00]
      + mat.values[MAT3_M21] * self.values[MAT3_M10]
      + mat.values[MAT3_M22] * self.values[MAT3_M20];
    let v21 = mat.values[MAT3_M20] * self.values[MAT3_M01]
      + mat.values[MAT3_M21] * self.values[MAT3_M11]
      + mat.values[MAT3_M22] * self.values[MAT3_M21];
    let v22 = mat.values[MAT3_M20] * self.values[MAT3_M02]
      + mat.values[MAT3_M21] * self.values[MAT3_M12]
      + mat.values[MAT3_M22] * self.values[MAT3_M22];

    self.values[MAT3_M00] = v00;
    self.values[MAT3_M10] = v10;
    self.values[MAT3_M20] = v20;
    self.values[MAT3_M01] = v01;
    self.values[MAT3_M11] = v11;
    self.values[MAT3_M21] = v21;
    self.values[MAT3_M02] = v02;
    self.values[MAT3_M12] = v12;
    self.values[MAT3_M22] = v22;

    self
  }

  pub fn set_to_rotation_rad(&mut self, radians: f32) -> &mut Self {
    let cos = radians.cos();
    let sin = radians.sin();

    self.values[MAT3_M00] = cos;
    self.values[MAT3_M10] = sin;
    self.values[MAT3_M20] = 0.0;

    self.values[MAT3_M01] = -sin;
    self.values[MAT3_M11] = cos;
    self.values[MAT3_M21] = 0.0;

    self.values[MAT3_M02] = 0.0;
    self.values[MAT3_M12] = 0.0;
    self.values[MAT3_M22] = 1.0;

    self
  }

  pub fn set_to_rotation_deg(&mut self, degrees: f32) -> &mut Self {
    self.set_to_rotation_rad(degrees.to_radians())
  }

  pub fn set_to_rotation_with_axis_rad(&mut self, axis: &Vec3, radians: f32) -> &mut Self {
    let cos = radians.cos();
    let sin = radians.sin();

    let tmp = 1.0 - cos;

    self.values[MAT3_M00] = tmp * axis.x * axis.x + cos;
    self.values[MAT3_M10] = tmp * axis.x * axis.y - sin * axis.z;
    self.values[MAT3_M20] = tmp * axis.x * axis.z + sin * axis.y;

    self.values[MAT3_M01] = tmp * axis.x * axis.y + sin * axis.z;
    self.values[MAT3_M11] = tmp * axis.y * axis.y + cos;
    self.values[MAT3_M21] = tmp * axis.y * axis.z - sin * axis.x;

    self.values[MAT3_M02] = tmp * axis.x * axis.z - sin * axis.y;
    self.values[MAT3_M12] = tmp * axis.y * axis.z + sin * axis.x;
    self.values[MAT3_M22] = tmp * axis.z * axis.z + cos;
    self
  }

  pub fn set_to_translation(&mut self, x: f32, y: f32) -> &mut Self {
    self.values[MAT3_M00] = 1.0;
    self.values[MAT3_M10] = 0.0;
    self.values[MAT3_M20] = 0.0;

    self.values[MAT3_M01] = 0.0;
    self.values[MAT3_M11] = 1.0;
    self.values[MAT3_M21] = 0.0;

    self.values[MAT3_M02] = x;
    self.values[MAT3_M12] = y;
    self.values[MAT3_M22] = 1.0;

    self
  }

  pub fn set_to_scaling(&mut self, x: f32, y: f32) -> &mut Self {
    self.values[MAT3_M00] = x;
    self.values[MAT3_M10] = 0.0;
    self.values[MAT3_M20] = 0.0;

    self.values[MAT3_M01] = 0.0;
    self.values[MAT3_M11] = y;
    self.values[MAT3_M21] = 0.0;

    self.values[MAT3_M02] = 0.0;
    self.values[MAT3_M12] = 0.0;
    self.values[MAT3_M22] = 1.0;

    self
  }

  pub fn det(&self) -> f32 {
    return self.values[MAT3_M00]
      * (self.values[MAT3_M11] * self.values[MAT3_M22]
        - self.values[MAT3_M12] * self.values[MAT3_M21])
      - self.values[MAT3_M01]
        * (self.values[MAT3_M10] * self.values[MAT3_M22]
          - self.values[MAT3_M12] * self.values[MAT3_M20])
      + self.values[MAT3_M02]
        * (self.values[MAT3_M10] * self.values[MAT3_M21]
          - self.values[MAT3_M11] * self.values[MAT3_M20]);
  }

  pub fn inv(&mut self) -> &mut Self {
    let det = self.det();
    if det == 0.0 {
      panic!("Matrix is singular and cannot be inverted");
    }

    let inv_det = 1.0 / det;

    self.tmp[MAT3_M00] =
      self.values[MAT3_M11] * self.values[MAT3_M22] - self.values[MAT3_M12] * self.values[MAT3_M21];
    self.tmp[MAT3_M10] =
      self.values[MAT3_M02] * self.values[MAT3_M21] - self.values[MAT3_M01] * self.values[MAT3_M22];
    self.tmp[MAT3_M20] =
      self.values[MAT3_M01] * self.values[MAT3_M12] - self.values[MAT3_M02] * self.values[MAT3_M11];
    self.tmp[MAT3_M01] =
      self.values[MAT3_M12] * self.values[MAT3_M20] - self.values[MAT3_M10] * self.values[MAT3_M22];
    self.tmp[MAT3_M11] =
      self.values[MAT3_M00] * self.values[MAT3_M22] - self.values[MAT3_M02] * self.values[MAT3_M20];
    self.tmp[MAT3_M21] =
      self.values[MAT3_M02] * self.values[MAT3_M10] - self.values[MAT3_M00] * self.values[MAT3_M12];
    self.tmp[MAT3_M02] =
      self.values[MAT3_M10] * self.values[MAT3_M21] - self.values[MAT3_M11] * self.values[MAT3_M20];
    self.tmp[MAT3_M12] =
      self.values[MAT3_M01] * self.values[MAT3_M20] - self.values[MAT3_M00] * self.values[MAT3_M21];
    self.tmp[MAT3_M22] =
      self.values[MAT3_M00] * self.values[MAT3_M11] - self.values[MAT3_M01] * self.values[MAT3_M10];

    self.values[MAT3_M00] = inv_det * self.tmp[MAT3_M00];
    self.values[MAT3_M10] = inv_det * self.tmp[MAT3_M10];
    self.values[MAT3_M20] = inv_det * self.tmp[MAT3_M20];
    self.values[MAT3_M01] = inv_det * self.tmp[MAT3_M01];
    self.values[MAT3_M11] = inv_det * self.tmp[MAT3_M11];
    self.values[MAT3_M21] = inv_det * self.tmp[MAT3_M21];
    self.values[MAT3_M02] = inv_det * self.tmp[MAT3_M02];
    self.values[MAT3_M12] = inv_det * self.tmp[MAT3_M12];
    self.values[MAT3_M22] = inv_det * self.tmp[MAT3_M22];

    self
  }

  pub fn set_by_mat3(&mut self, mat: &Self) -> &mut Self {
    for i in 0..9 {
      self.values[i] = mat.values[i];
    }

    self
  }

  pub fn set_by_mat4(&mut self, mat: &Mat4) -> &mut Self {
    self.values[MAT3_M00] = mat.values[MAT4_M00];
    self.values[MAT3_M10] = mat.values[MAT4_M10];
    self.values[MAT3_M20] = mat.values[MAT4_M20];
    self.values[MAT3_M01] = mat.values[MAT4_M01];
    self.values[MAT3_M11] = mat.values[MAT4_M11];
    self.values[MAT3_M21] = mat.values[MAT4_M21];
    self.values[MAT3_M02] = mat.values[MAT4_M02];
    self.values[MAT3_M12] = mat.values[MAT4_M12];
    self.values[MAT3_M22] = mat.values[MAT4_M22];

    self
  }

  pub fn trn_by_vec2(&mut self, vec: &Vec2) -> &mut Self {
    self.values[MAT3_M02] += vec.x;
    self.values[MAT3_M12] += vec.y;

    self
  }

  pub fn trn_by_vec3(&mut self, vec: &Vec3) -> &mut Self {
    self.values[MAT3_M02] += vec.x;
    self.values[MAT3_M12] += vec.y;

    self
  }

  pub fn translate(&mut self, x: f32, y: f32) -> &mut Self {
    self.tmp[MAT3_M00] = 1.0;
    self.tmp[MAT3_M10] = 0.0;
    self.tmp[MAT3_M20] = 0.0;

    self.tmp[MAT3_M01] = 0.0;
    self.tmp[MAT3_M11] = 1.0;
    self.tmp[MAT3_M21] = 0.0;

    self.tmp[MAT3_M02] = x;
    self.tmp[MAT3_M12] = y;
    self.tmp[MAT3_M22] = 1.0;

    self.mul_with_mat3_values(self.tmp);

    self
  }

  pub fn rotate_rad(&mut self, radians: f32) -> &mut Self {
    if radians == 0.0 {
      return self;
    }

    let cos = radians.cos();
    let sin = radians.sin();

    self.tmp[MAT3_M00] = cos;
    self.tmp[MAT3_M10] = sin;
    self.tmp[MAT3_M20] = 0.0;

    self.tmp[MAT3_M01] = -sin;
    self.tmp[MAT3_M11] = cos;
    self.tmp[MAT3_M21] = 0.0;

    self.tmp[MAT3_M02] = 0.0;
    self.tmp[MAT3_M12] = 0.0;
    self.tmp[MAT3_M22] = 1.0;

    self.mul_with_mat3_values(self.tmp);

    self
  }

  pub fn rotate_deg(&mut self, degrees: f32) -> &mut Self {
    self.rotate_rad(degrees.to_radians())
  }

  pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
    self.tmp[MAT3_M00] = x;
    self.tmp[MAT3_M10] = 0.0;
    self.tmp[MAT3_M20] = 0.0;

    self.tmp[MAT3_M01] = 0.0;
    self.tmp[MAT3_M11] = y;
    self.tmp[MAT3_M21] = 0.0;

    self.tmp[MAT3_M02] = 0.0;
    self.tmp[MAT3_M12] = 0.0;
    self.tmp[MAT3_M22] = 1.0;

    self.mul_with_mat3_values(self.tmp);

    self
  }

  pub fn get_values(&self) -> [f32; 9] {
    self.values
  }

  pub fn get_translation<'a>(&self, output: &'a mut Vec2) -> &'a mut Vec2 {
    output.x = self.values[MAT3_M02];
    output.y = self.values[MAT3_M12];

    output
  }

  pub fn get_scale<'a>(&self, output: &'a mut Vec2) -> &'a mut Vec2 {
    output.x = self.values[MAT3_M00];
    output.y = self.values[MAT3_M11];

    output
  }

  pub fn get_rotation_rad(&self) -> f32 {
    return self.values[MAT3_M10].atan2(self.values[MAT3_M00]);
  }

  pub fn get_rotation_deg(&self) -> f32 {
    return self.get_rotation_rad().to_degrees();
  }

  pub fn scl_by_number(&mut self, scalar: f32) -> &mut Self {
    self.values[MAT3_M00] *= scalar;
    self.values[MAT3_M11] *= scalar;

    self
  }

  pub fn scl_by_vec2(&mut self, scale: &Vec2) -> &mut Self {
    self.values[MAT3_M00] *= scale.x;
    self.values[MAT3_M11] *= scale.y;

    self
  }

  pub fn scl_by_vec3(&mut self, scale: &Vec3) -> &mut Self {
    self.values[MAT3_M00] *= scale.x;
    self.values[MAT3_M11] *= scale.y;

    self
  }

  pub fn transpose(&mut self) -> &mut Self {
    let v01 = self.values[MAT3_M10];
    let v02 = self.values[MAT3_M20];
    let v10 = self.values[MAT3_M01];
    let v12 = self.values[MAT3_M21];
    let v20 = self.values[MAT3_M02];
    let v21 = self.values[MAT3_M12];

    self.values[MAT3_M01] = v01;
    self.values[MAT3_M02] = v02;
    self.values[MAT3_M10] = v10;
    self.values[MAT3_M12] = v12;
    self.values[MAT3_M20] = v20;
    self.values[MAT3_M21] = v21;

    self
  }

  pub fn mul_values<'a>(mata: &'a mut [f32; 9], matb: &[f32; 9]) -> &'a mut [f32; 9] {
    let v00 = mata[MAT3_M00] * matb[MAT3_M00]
      + mata[MAT3_M01] * matb[MAT3_M10]
      + mata[MAT3_M02] * matb[MAT3_M20];
    let v01 = mata[MAT3_M00] * matb[MAT3_M01]
      + mata[MAT3_M01] * matb[MAT3_M11]
      + mata[MAT3_M02] * matb[MAT3_M21];
    let v02 = mata[MAT3_M00] * matb[MAT3_M02]
      + mata[MAT3_M01] * matb[MAT3_M12]
      + mata[MAT3_M02] * matb[MAT3_M22];

    let v10 = mata[MAT3_M10] * matb[MAT3_M00]
      + mata[MAT3_M11] * matb[MAT3_M10]
      + mata[MAT3_M12] * matb[MAT3_M20];
    let v11 = mata[MAT3_M10] * matb[MAT3_M01]
      + mata[MAT3_M11] * matb[MAT3_M11]
      + mata[MAT3_M12] * matb[MAT3_M21];
    let v12 = mata[MAT3_M10] * matb[MAT3_M02]
      + mata[MAT3_M11] * matb[MAT3_M12]
      + mata[MAT3_M12] * matb[MAT3_M22];

    let v20 = mata[MAT3_M20] * matb[MAT3_M00]
      + mata[MAT3_M21] * matb[MAT3_M10]
      + mata[MAT3_M22] * matb[MAT3_M20];
    let v21 = mata[MAT3_M20] * matb[MAT3_M01]
      + mata[MAT3_M21] * matb[MAT3_M11]
      + mata[MAT3_M22] * matb[MAT3_M21];
    let v22 = mata[MAT3_M20] * matb[MAT3_M02]
      + mata[MAT3_M21] * matb[MAT3_M12]
      + mata[MAT3_M22] * matb[MAT3_M22];

    mata[MAT3_M00] = v00;
    mata[MAT3_M10] = v10;
    mata[MAT3_M20] = v20;
    mata[MAT3_M01] = v01;
    mata[MAT3_M11] = v11;
    mata[MAT3_M21] = v21;
    mata[MAT3_M02] = v02;
    mata[MAT3_M12] = v12;
    mata[MAT3_M22] = v22;

    mata
  }
}
