use super::{
  common::{is_equal, is_zero},
  quaternion::Quaternion,
  vector3::Vec3,
};

pub const MAT4_M00: usize = 0;
pub const MAT4_M01: usize = 4;
pub const MAT4_M02: usize = 8;
pub const MAT4_M03: usize = 12;
pub const MAT4_M10: usize = 1;
pub const MAT4_M11: usize = 5;
pub const MAT4_M12: usize = 9;
pub const MAT4_M13: usize = 13;
pub const MAT4_M20: usize = 2;
pub const MAT4_M21: usize = 6;
pub const MAT4_M22: usize = 10;
pub const MAT4_M23: usize = 14;
pub const MAT4_M30: usize = 3;
pub const MAT4_M31: usize = 7;
pub const MAT4_M32: usize = 11;
pub const MAT4_M33: usize = 15;

pub struct Mat4 {
  pub values: [f32; 16],
  temp: [f32; 16],
  x_axis: Vec3,
  y_axis: Vec3,
  z_axis: Vec3,
}

impl Mat4 {
  pub fn new() -> Self {
    let mut values = [0.0; 16];
    values[MAT4_M00] = 1.0;
    values[MAT4_M11] = 1.0;
    values[MAT4_M22] = 1.0;
    values[MAT4_M33] = 1.0;
    Self {
      values,
      temp: [0.0; 16],
      x_axis: Vec3::new(1.0, 0.0, 0.0),
      y_axis: Vec3::new(0.0, 1.0, 0.0),
      z_axis: Vec3::new(0.0, 0.0, 1.0),
    }
  }

  pub fn set(&mut self, values: &[f32; 16]) -> &mut Self {
    for i in 0..16 {
      self.values[i] = values[i];
    }
    self
  }

  pub fn set_from_quaternion(&mut self, quaternion: &Quaternion) -> &mut Self {
    let x = quaternion.x;
    let y = quaternion.y;
    let z = quaternion.z;
    let w = quaternion.w;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;

    self.values[MAT4_M00] = 1.0 - (yy + zz);
    self.values[MAT4_M01] = xy - wz;
    self.values[MAT4_M02] = xz + wy;
    self.values[MAT4_M03] = 0.;

    self.values[MAT4_M10] = xy + wz;
    self.values[MAT4_M11] = 1.0 - (xx + zz);
    self.values[MAT4_M12] = yz - wx;
    self.values[MAT4_M13] = 0.;

    self.values[MAT4_M20] = xz - wy;
    self.values[MAT4_M21] = yz + wx;
    self.values[MAT4_M22] = 1.0 - (xx + yy);
    self.values[MAT4_M23] = 0.;

    self.values[MAT4_M30] = 0.;
    self.values[MAT4_M31] = 0.;
    self.values[MAT4_M32] = 0.;
    self.values[MAT4_M33] = 1.;

    self
  }

  pub fn set_from_translation_rotation(
    &mut self,
    position: &Vec3,
    orientation: &Quaternion,
    scale: &Vec3,
  ) -> &mut Self {
    let x = orientation.x;
    let y = orientation.y;
    let z = orientation.z;
    let w = orientation.w;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;
    let sx = scale.x;
    let sy = scale.y;
    let sz = scale.z;

    self.values[MAT4_M00] = (1.0 - (yy + zz)) * sx;
    self.values[MAT4_M01] = (xy - wz) * sy;
    self.values[MAT4_M02] = (xz + wy) * sz;
    self.values[MAT4_M03] = position.x;

    self.values[MAT4_M10] = (xy + wz) * sx;
    self.values[MAT4_M11] = (1.0 - (xx + zz)) * sy;
    self.values[MAT4_M12] = (yz - wx) * sz;
    self.values[MAT4_M13] = position.y;

    self.values[MAT4_M20] = (xz - wy) * sx;
    self.values[MAT4_M21] = (yz + wx) * sy;
    self.values[MAT4_M22] = (1.0 - (xx + yy)) * sz;
    self.values[MAT4_M23] = position.z;

    self.values[MAT4_M30] = 0.0;
    self.values[MAT4_M31] = 0.0;
    self.values[MAT4_M32] = 0.0;
    self.values[MAT4_M33] = 1.0;

    self
  }

  pub fn transpose(&mut self) -> &mut Self {
    self.temp[MAT4_M00] = self.values[MAT4_M00];
    self.temp[MAT4_M01] = self.values[MAT4_M10];
    self.temp[MAT4_M02] = self.values[MAT4_M20];
    self.temp[MAT4_M03] = self.values[MAT4_M30];
    self.temp[MAT4_M10] = self.values[MAT4_M01];
    self.temp[MAT4_M11] = self.values[MAT4_M11];
    self.temp[MAT4_M12] = self.values[MAT4_M21];
    self.temp[MAT4_M13] = self.values[MAT4_M31];
    self.temp[MAT4_M20] = self.values[MAT4_M02];
    self.temp[MAT4_M21] = self.values[MAT4_M12];
    self.temp[MAT4_M22] = self.values[MAT4_M22];
    self.temp[MAT4_M23] = self.values[MAT4_M32];
    self.temp[MAT4_M30] = self.values[MAT4_M03];
    self.temp[MAT4_M31] = self.values[MAT4_M13];
    self.temp[MAT4_M32] = self.values[MAT4_M23];
    self.temp[MAT4_M33] = self.values[MAT4_M33];

    let temp = self.temp;
    self.set(&temp)
  }

  pub fn identity(&mut self) -> &mut Self {
    self.values[MAT4_M00] = 1.0;
    self.values[MAT4_M01] = 0.0;
    self.values[MAT4_M02] = 0.0;
    self.values[MAT4_M03] = 0.0;
    self.values[MAT4_M10] = 0.0;
    self.values[MAT4_M11] = 1.0;
    self.values[MAT4_M12] = 0.0;
    self.values[MAT4_M13] = 0.0;
    self.values[MAT4_M20] = 0.0;
    self.values[MAT4_M21] = 0.0;
    self.values[MAT4_M22] = 1.0;
    self.values[MAT4_M23] = 0.0;
    self.values[MAT4_M30] = 0.0;
    self.values[MAT4_M31] = 0.0;
    self.values[MAT4_M32] = 0.0;
    self.values[MAT4_M33] = 1.0;
    self
  }

  pub fn invert(&mut self) -> &mut Self {
    let v = &mut self.values;
    let t = &mut self.temp;

    let l_det = v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M12] * v[MAT4_M03]
      - v[MAT4_M20] * v[MAT4_M31] * v[MAT4_M12] * v[MAT4_M03]
      - v[MAT4_M30] * v[MAT4_M11] * v[MAT4_M22] * v[MAT4_M03]
      + v[MAT4_M10] * v[MAT4_M31] * v[MAT4_M22] * v[MAT4_M03]
      + v[MAT4_M20] * v[MAT4_M11] * v[MAT4_M32] * v[MAT4_M03]
      - v[MAT4_M10] * v[MAT4_M21] * v[MAT4_M32] * v[MAT4_M03]
      - v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M02] * v[MAT4_M13]
      + v[MAT4_M20] * v[MAT4_M31] * v[MAT4_M02] * v[MAT4_M13]
      + v[MAT4_M30] * v[MAT4_M01] * v[MAT4_M22] * v[MAT4_M13]
      - v[MAT4_M00] * v[MAT4_M31] * v[MAT4_M22] * v[MAT4_M13]
      - v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M32] * v[MAT4_M13]
      + v[MAT4_M00] * v[MAT4_M21] * v[MAT4_M32] * v[MAT4_M13]
      + v[MAT4_M30] * v[MAT4_M11] * v[MAT4_M02] * v[MAT4_M23]
      - v[MAT4_M10] * v[MAT4_M31] * v[MAT4_M02] * v[MAT4_M23]
      - v[MAT4_M30] * v[MAT4_M01] * v[MAT4_M12] * v[MAT4_M23]
      + v[MAT4_M00] * v[MAT4_M31] * v[MAT4_M12] * v[MAT4_M23]
      + v[MAT4_M10] * v[MAT4_M01] * v[MAT4_M32] * v[MAT4_M23]
      - v[MAT4_M00] * v[MAT4_M11] * v[MAT4_M32] * v[MAT4_M23]
      - v[MAT4_M20] * v[MAT4_M11] * v[MAT4_M02] * v[MAT4_M33]
      + v[MAT4_M10] * v[MAT4_M21] * v[MAT4_M02] * v[MAT4_M33]
      + v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M12] * v[MAT4_M33]
      - v[MAT4_M00] * v[MAT4_M21] * v[MAT4_M12] * v[MAT4_M33]
      - v[MAT4_M10] * v[MAT4_M01] * v[MAT4_M22] * v[MAT4_M33]
      + v[MAT4_M00] * v[MAT4_M11] * v[MAT4_M22] * v[MAT4_M33];

    if l_det == 0. {
      panic!("Cannot invert matrix, determinant is 0");
    }

    let inv_det = 1.0 / l_det;

    t[MAT4_M00] = inv_det
      * (v[MAT4_M11] * v[MAT4_M22] * v[MAT4_M33]
        + v[MAT4_M21] * v[MAT4_M32] * v[MAT4_M13]
        + v[MAT4_M31] * v[MAT4_M12] * v[MAT4_M23]
        - v[MAT4_M11] * v[MAT4_M32] * v[MAT4_M23]
        - v[MAT4_M21] * v[MAT4_M12] * v[MAT4_M33]
        - v[MAT4_M31] * v[MAT4_M22] * v[MAT4_M13]);

    t[MAT4_M01] = inv_det
      * (v[MAT4_M10] * v[MAT4_M32] * v[MAT4_M23]
        + v[MAT4_M20] * v[MAT4_M12] * v[MAT4_M33]
        + v[MAT4_M30] * v[MAT4_M22] * v[MAT4_M13]
        - v[MAT4_M10] * v[MAT4_M22] * v[MAT4_M33]
        - v[MAT4_M20] * v[MAT4_M32] * v[MAT4_M13]
        - v[MAT4_M30] * v[MAT4_M12] * v[MAT4_M23]);

    t[MAT4_M02] = inv_det
      * (v[MAT4_M10] * v[MAT4_M21] * v[MAT4_M33]
        + v[MAT4_M20] * v[MAT4_M31] * v[MAT4_M13]
        + v[MAT4_M30] * v[MAT4_M11] * v[MAT4_M23]
        - v[MAT4_M10] * v[MAT4_M31] * v[MAT4_M23]
        - v[MAT4_M20] * v[MAT4_M11] * v[MAT4_M33]
        - v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M13]);

    t[MAT4_M03] = inv_det
      * (v[MAT4_M10] * v[MAT4_M31] * v[MAT4_M22]
        + v[MAT4_M20] * v[MAT4_M11] * v[MAT4_M32]
        + v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M12]
        - v[MAT4_M10] * v[MAT4_M21] * v[MAT4_M32]
        - v[MAT4_M20] * v[MAT4_M31] * v[MAT4_M12]
        - v[MAT4_M30] * v[MAT4_M11] * v[MAT4_M22]);

    t[MAT4_M10] = inv_det
      * (v[MAT4_M01] * v[MAT4_M32] * v[MAT4_M23]
        + v[MAT4_M21] * v[MAT4_M02] * v[MAT4_M33]
        + v[MAT4_M31] * v[MAT4_M22] * v[MAT4_M03]
        - v[MAT4_M01] * v[MAT4_M22] * v[MAT4_M33]
        - v[MAT4_M21] * v[MAT4_M32] * v[MAT4_M03]
        - v[MAT4_M31] * v[MAT4_M02] * v[MAT4_M23]);

    t[MAT4_M11] = inv_det
      * (v[MAT4_M00] * v[MAT4_M22] * v[MAT4_M33]
        + v[MAT4_M20] * v[MAT4_M02] * v[MAT4_M33]
        + v[MAT4_M30] * v[MAT4_M22] * v[MAT4_M03]
        - v[MAT4_M00] * v[MAT4_M32] * v[MAT4_M23]
        - v[MAT4_M20] * v[MAT4_M02] * v[MAT4_M33]
        - v[MAT4_M30] * v[MAT4_M22] * v[MAT4_M03]);

    t[MAT4_M12] = inv_det
      * (v[MAT4_M00] * v[MAT4_M31] * v[MAT4_M23]
        + v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M33]
        + v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M03]
        - v[MAT4_M00] * v[MAT4_M21] * v[MAT4_M33]
        - v[MAT4_M20] * v[MAT4_M31] * v[MAT4_M03]
        - v[MAT4_M30] * v[MAT4_M01] * v[MAT4_M23]);

    t[MAT4_M13] = inv_det
      * (v[MAT4_M00] * v[MAT4_M21] * v[MAT4_M32]
        + v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M32]
        + v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M02]
        - v[MAT4_M00] * v[MAT4_M31] * v[MAT4_M22]
        - v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M32]
        - v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M02]);

    t[MAT4_M20] = inv_det
      * (v[MAT4_M01] * v[MAT4_M12] * v[MAT4_M33]
        + v[MAT4_M11] * v[MAT4_M32] * v[MAT4_M03]
        + v[MAT4_M31] * v[MAT4_M02] * v[MAT4_M13]
        - v[MAT4_M01] * v[MAT4_M32] * v[MAT4_M13]
        - v[MAT4_M11] * v[MAT4_M02] * v[MAT4_M33]
        - v[MAT4_M31] * v[MAT4_M12] * v[MAT4_M03]);

    t[MAT4_M21] = inv_det
      * (v[MAT4_M00] * v[MAT4_M32] * v[MAT4_M13]
        + v[MAT4_M10] * v[MAT4_M02] * v[MAT4_M33]
        + v[MAT4_M30] * v[MAT4_M12] * v[MAT4_M03]
        - v[MAT4_M00] * v[MAT4_M12] * v[MAT4_M33]
        - v[MAT4_M10] * v[MAT4_M32] * v[MAT4_M03]
        - v[MAT4_M30] * v[MAT4_M02] * v[MAT4_M13]);

    t[MAT4_M22] = inv_det
      * (v[MAT4_M00] * v[MAT4_M11] * v[MAT4_M33]
        + v[MAT4_M10] * v[MAT4_M31] * v[MAT4_M03]
        + v[MAT4_M30] * v[MAT4_M01] * v[MAT4_M13]
        - v[MAT4_M00] * v[MAT4_M31] * v[MAT4_M13]
        - v[MAT4_M10] * v[MAT4_M01] * v[MAT4_M33]
        - v[MAT4_M30] * v[MAT4_M11] * v[MAT4_M03]);

    t[MAT4_M23] = inv_det
      * (v[MAT4_M00] * v[MAT4_M31] * v[MAT4_M12]
        + v[MAT4_M10] * v[MAT4_M01] * v[MAT4_M32]
        + v[MAT4_M30] * v[MAT4_M11] * v[MAT4_M02]
        - v[MAT4_M00] * v[MAT4_M11] * v[MAT4_M32]
        - v[MAT4_M10] * v[MAT4_M31] * v[MAT4_M02]
        - v[MAT4_M30] * v[MAT4_M01] * v[MAT4_M12]);

    t[MAT4_M30] = inv_det
      * (v[MAT4_M01] * v[MAT4_M22] * v[MAT4_M13]
        + v[MAT4_M11] * v[MAT4_M02] * v[MAT4_M23]
        + v[MAT4_M21] * v[MAT4_M12] * v[MAT4_M03]
        - v[MAT4_M01] * v[MAT4_M12] * v[MAT4_M23]
        - v[MAT4_M11] * v[MAT4_M22] * v[MAT4_M03]
        - v[MAT4_M21] * v[MAT4_M02] * v[MAT4_M13]);

    t[MAT4_M31] = inv_det
      * (v[MAT4_M00] * v[MAT4_M12] * v[MAT4_M23]
        + v[MAT4_M10] * v[MAT4_M22] * v[MAT4_M03]
        + v[MAT4_M20] * v[MAT4_M02] * v[MAT4_M13]
        - v[MAT4_M00] * v[MAT4_M22] * v[MAT4_M13]
        - v[MAT4_M10] * v[MAT4_M02] * v[MAT4_M23]
        - v[MAT4_M20] * v[MAT4_M12] * v[MAT4_M03]);

    t[MAT4_M32] = inv_det
      * (v[MAT4_M00] * v[MAT4_M21] * v[MAT4_M13]
        + v[MAT4_M10] * v[MAT4_M01] * v[MAT4_M23]
        + v[MAT4_M20] * v[MAT4_M11] * v[MAT4_M03]
        - v[MAT4_M00] * v[MAT4_M11] * v[MAT4_M23]
        - v[MAT4_M10] * v[MAT4_M21] * v[MAT4_M03]
        - v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M13]);

    t[MAT4_M33] = inv_det
      * (v[MAT4_M00] * v[MAT4_M11] * v[MAT4_M22]
        + v[MAT4_M10] * v[MAT4_M21] * v[MAT4_M02]
        + v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M12]
        - v[MAT4_M00] * v[MAT4_M21] * v[MAT4_M12]
        - v[MAT4_M10] * v[MAT4_M01] * v[MAT4_M22]
        - v[MAT4_M20] * v[MAT4_M11] * v[MAT4_M02]);

    v[MAT4_M00] = t[MAT4_M00];
    v[MAT4_M01] = t[MAT4_M01];
    v[MAT4_M02] = t[MAT4_M02];
    v[MAT4_M03] = t[MAT4_M03];
    v[MAT4_M10] = t[MAT4_M10];
    v[MAT4_M11] = t[MAT4_M11];
    v[MAT4_M12] = t[MAT4_M12];
    v[MAT4_M13] = t[MAT4_M13];
    v[MAT4_M20] = t[MAT4_M20];
    v[MAT4_M21] = t[MAT4_M21];
    v[MAT4_M22] = t[MAT4_M22];
    v[MAT4_M23] = t[MAT4_M23];
    v[MAT4_M30] = t[MAT4_M30];
    v[MAT4_M31] = t[MAT4_M31];
    v[MAT4_M32] = t[MAT4_M32];
    v[MAT4_M33] = t[MAT4_M33];

    self
  }

  pub fn determinant(&self) -> f32 {
    let v = self.values;
    v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M12] * v[MAT4_M03]
      - v[MAT4_M20] * v[MAT4_M31] * v[MAT4_M12] * v[MAT4_M03]
      - v[MAT4_M30] * v[MAT4_M11] * v[MAT4_M22] * v[MAT4_M03]
      + v[MAT4_M10] * v[MAT4_M31] * v[MAT4_M22] * v[MAT4_M03]
      + v[MAT4_M20] * v[MAT4_M11] * v[MAT4_M32] * v[MAT4_M03]
      - v[MAT4_M10] * v[MAT4_M21] * v[MAT4_M32] * v[MAT4_M03]
      - v[MAT4_M30] * v[MAT4_M21] * v[MAT4_M02] * v[MAT4_M13]
      + v[MAT4_M20] * v[MAT4_M31] * v[MAT4_M02] * v[MAT4_M13]
      + v[MAT4_M30] * v[MAT4_M01] * v[MAT4_M22] * v[MAT4_M13]
      - v[MAT4_M00] * v[MAT4_M31] * v[MAT4_M22] * v[MAT4_M13]
      - v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M32] * v[MAT4_M13]
      + v[MAT4_M00] * v[MAT4_M21] * v[MAT4_M32] * v[MAT4_M13]
      + v[MAT4_M30] * v[MAT4_M11] * v[MAT4_M02] * v[MAT4_M23]
      - v[MAT4_M10] * v[MAT4_M31] * v[MAT4_M02] * v[MAT4_M23]
      - v[MAT4_M30] * v[MAT4_M01] * v[MAT4_M12] * v[MAT4_M23]
      + v[MAT4_M00] * v[MAT4_M31] * v[MAT4_M12] * v[MAT4_M23]
      + v[MAT4_M10] * v[MAT4_M01] * v[MAT4_M32] * v[MAT4_M23]
      - v[MAT4_M00] * v[MAT4_M11] * v[MAT4_M32] * v[MAT4_M23]
      - v[MAT4_M20] * v[MAT4_M11] * v[MAT4_M02] * v[MAT4_M33]
      + v[MAT4_M10] * v[MAT4_M21] * v[MAT4_M02] * v[MAT4_M33]
      + v[MAT4_M20] * v[MAT4_M01] * v[MAT4_M12] * v[MAT4_M33]
      - v[MAT4_M00] * v[MAT4_M21] * v[MAT4_M12] * v[MAT4_M33]
      - v[MAT4_M10] * v[MAT4_M01] * v[MAT4_M22] * v[MAT4_M33]
      + v[MAT4_M00] * v[MAT4_M11] * v[MAT4_M22] * v[MAT4_M33]
  }

  pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
    self.values[MAT4_M03] += x;
    self.values[MAT4_M13] += y;
    self.values[MAT4_M23] += z;
    self
  }

  pub fn set_translation(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
    self.values[MAT4_M03] = x;
    self.values[MAT4_M13] = y;
    self.values[MAT4_M23] = z;
    self
  }

  pub fn get_translation<'a>(&self, vec3: &'a mut Vec3) -> &'a mut Vec3 {
    vec3.x = self.values[MAT4_M03];
    vec3.y = self.values[MAT4_M13];
    vec3.z = self.values[MAT4_M23];
    vec3
  }

  pub fn has_rotation_or_scaling(&self) -> bool {
    let v = self.values;
    let never_rotate_or_scale = is_equal(v[MAT4_M00], 1.)
      && is_equal(v[MAT4_M11], 1.)
      && is_equal(v[MAT4_M22], 1.)
      && is_zero(v[MAT4_M01])
      && is_zero(v[MAT4_M02])
      && is_zero(v[MAT4_M10])
      && is_zero(v[MAT4_M12])
      && is_zero(v[MAT4_M20])
      && is_zero(v[MAT4_M21]);
    !never_rotate_or_scale
  }

  pub fn det3x3(&self) -> f32 {
    let v = self.values;
    v[MAT4_M00] * (v[MAT4_M11] * v[MAT4_M22] - v[MAT4_M12] * v[MAT4_M21])
      + v[MAT4_M01] * (v[MAT4_M12] * v[MAT4_M20] - v[MAT4_M10] * v[MAT4_M22])
      + v[MAT4_M02] * (v[MAT4_M10] * v[MAT4_M21] - v[MAT4_M11] * v[MAT4_M20])
  }

  pub fn copy(&self) -> Self {
    let mut mat = Mat4::new();
    mat.set(&self.values);
    mat
  }

  pub fn projection(&mut self, near: f32, far: f32, fovy: f32, aspect_ratio: f32) -> &mut Self {
    self.identity();
    let l_fd = 1.0 / (fovy / 2.0).tan();
    let l_a1 = (far + near) / (near - far);
    let l_a2 = (2.0 * far * near) / (near - far);
    let v = &mut self.values;
    v[MAT4_M00] = l_fd / aspect_ratio;
    v[MAT4_M11] = l_fd;
    v[MAT4_M22] = l_a1;
    v[MAT4_M23] = -1.0;
    v[MAT4_M32] = l_a2;
    v[MAT4_M33] = 0.0;

    self
  }

  pub fn ortho2d(&mut self, x: f32, y: f32, width: f32, height: f32) -> &mut Self {
    self.ortho(x, x + width, y, y + height, 0.0, 1.0)
  }

  pub fn ortho(
    &mut self,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
  ) -> &mut Self {
    self.identity();
    let x_orth = 2.0 / (right - left);
    let y_orth = 2.0 / (top - bottom);
    let z_orth = -2.0 / (far - near);

    let tx = -(right + left) / (right - left);
    let ty = -(top + bottom) / (top - bottom);
    let tz = -(far + near) / (far - near);

    let v = &mut self.values;
    v[MAT4_M00] = x_orth;
    v[MAT4_M11] = y_orth;
    v[MAT4_M22] = z_orth;
    v[MAT4_M03] = tx;
    v[MAT4_M13] = ty;
    v[MAT4_M23] = tz;

    self
  }

  pub fn multiply(&mut self, matrix: &Self) -> &mut Self {
    let mut t = self.temp;
    let v = self.values;
    let m = matrix.values;

    t[MAT4_M00] = v[MAT4_M00] * m[MAT4_M00]
      + v[MAT4_M01] * m[MAT4_M10]
      + v[MAT4_M02] * m[MAT4_M20]
      + v[MAT4_M03] * m[MAT4_M30];

    t[MAT4_M01] = v[MAT4_M00] * m[MAT4_M01]
      + v[MAT4_M01] * m[MAT4_M11]
      + v[MAT4_M02] * m[MAT4_M21]
      + v[MAT4_M03] * m[MAT4_M31];

    t[MAT4_M02] = v[MAT4_M00] * m[MAT4_M02]
      + v[MAT4_M01] * m[MAT4_M12]
      + v[MAT4_M02] * m[MAT4_M22]
      + v[MAT4_M03] * m[MAT4_M32];

    t[MAT4_M03] = v[MAT4_M00] * m[MAT4_M03]
      + v[MAT4_M01] * m[MAT4_M13]
      + v[MAT4_M02] * m[MAT4_M23]
      + v[MAT4_M03] * m[MAT4_M33];

    t[MAT4_M10] = v[MAT4_M10] * m[MAT4_M00]
      + v[MAT4_M11] * m[MAT4_M10]
      + v[MAT4_M12] * m[MAT4_M20]
      + v[MAT4_M13] * m[MAT4_M30];

    t[MAT4_M11] = v[MAT4_M10] * m[MAT4_M01]
      + v[MAT4_M11] * m[MAT4_M11]
      + v[MAT4_M12] * m[MAT4_M21]
      + v[MAT4_M13] * m[MAT4_M31];

    t[MAT4_M12] = v[MAT4_M10] * m[MAT4_M02]
      + v[MAT4_M11] * m[MAT4_M12]
      + v[MAT4_M12] * m[MAT4_M22]
      + v[MAT4_M13] * m[MAT4_M32];

    t[MAT4_M13] = v[MAT4_M10] * m[MAT4_M03]
      + v[MAT4_M11] * m[MAT4_M13]
      + v[MAT4_M12] * m[MAT4_M23]
      + v[MAT4_M13] * m[MAT4_M33];

    t[MAT4_M20] = v[MAT4_M20] * m[MAT4_M00]
      + v[MAT4_M21] * m[MAT4_M10]
      + v[MAT4_M22] * m[MAT4_M20]
      + v[MAT4_M23] * m[MAT4_M30];

    t[MAT4_M21] = v[MAT4_M20] * m[MAT4_M01]
      + v[MAT4_M21] * m[MAT4_M11]
      + v[MAT4_M22] * m[MAT4_M21]
      + v[MAT4_M23] * m[MAT4_M31];

    t[MAT4_M22] = v[MAT4_M20] * m[MAT4_M02]
      + v[MAT4_M21] * m[MAT4_M12]
      + v[MAT4_M22] * m[MAT4_M22]
      + v[MAT4_M23] * m[MAT4_M32];

    t[MAT4_M23] = v[MAT4_M20] * m[MAT4_M03]
      + v[MAT4_M21] * m[MAT4_M13]
      + v[MAT4_M22] * m[MAT4_M23]
      + v[MAT4_M23] * m[MAT4_M33];

    t[MAT4_M30] = v[MAT4_M30] * m[MAT4_M00]
      + v[MAT4_M31] * m[MAT4_M10]
      + v[MAT4_M32] * m[MAT4_M20]
      + v[MAT4_M33] * m[MAT4_M30];

    t[MAT4_M31] = v[MAT4_M30] * m[MAT4_M01]
      + v[MAT4_M31] * m[MAT4_M11]
      + v[MAT4_M32] * m[MAT4_M21]
      + v[MAT4_M33] * m[MAT4_M31];

    t[MAT4_M32] = v[MAT4_M30] * m[MAT4_M02]
      + v[MAT4_M31] * m[MAT4_M12]
      + v[MAT4_M32] * m[MAT4_M22]
      + v[MAT4_M33] * m[MAT4_M32];

    t[MAT4_M33] = v[MAT4_M30] * m[MAT4_M03]
      + v[MAT4_M31] * m[MAT4_M13]
      + v[MAT4_M32] * m[MAT4_M23]
      + v[MAT4_M33] * m[MAT4_M33];

    self.set(&t)
  }

  pub fn multiply_left(&mut self, matrix: &Self) -> &mut Self {
    // TODO: we're copying here, try to do it in place
    let mut t = self.temp;
    let v = self.values;
    let m = matrix.values;

    t[MAT4_M00] = m[MAT4_M00] * v[MAT4_M00]
      + m[MAT4_M01] * v[MAT4_M10]
      + m[MAT4_M02] * v[MAT4_M20]
      + m[MAT4_M03] * v[MAT4_M30];

    t[MAT4_M01] = m[MAT4_M00] * v[MAT4_M01]
      + m[MAT4_M01] * v[MAT4_M11]
      + m[MAT4_M02] * v[MAT4_M21]
      + m[MAT4_M03] * v[MAT4_M31];

    t[MAT4_M02] = m[MAT4_M00] * v[MAT4_M02]
      + m[MAT4_M01] * v[MAT4_M12]
      + m[MAT4_M02] * v[MAT4_M22]
      + m[MAT4_M03] * v[MAT4_M32];

    t[MAT4_M03] = m[MAT4_M00] * v[MAT4_M03]
      + m[MAT4_M01] * v[MAT4_M13]
      + m[MAT4_M02] * v[MAT4_M23]
      + m[MAT4_M03] * v[MAT4_M33];

    t[MAT4_M10] = m[MAT4_M10] * v[MAT4_M00]
      + m[MAT4_M11] * v[MAT4_M10]
      + m[MAT4_M12] * v[MAT4_M20]
      + m[MAT4_M13] * v[MAT4_M30];

    t[MAT4_M11] = m[MAT4_M10] * v[MAT4_M01]
      + m[MAT4_M11] * v[MAT4_M11]
      + m[MAT4_M12] * v[MAT4_M21]
      + m[MAT4_M13] * v[MAT4_M31];

    t[MAT4_M12] = m[MAT4_M10] * v[MAT4_M02]
      + m[MAT4_M11] * v[MAT4_M12]
      + m[MAT4_M12] * v[MAT4_M22]
      + m[MAT4_M13] * v[MAT4_M32];

    t[MAT4_M13] = m[MAT4_M10] * v[MAT4_M03]
      + m[MAT4_M11] * v[MAT4_M13]
      + m[MAT4_M12] * v[MAT4_M23]
      + m[MAT4_M13] * v[MAT4_M33];

    t[MAT4_M20] = m[MAT4_M20] * v[MAT4_M00]
      + m[MAT4_M21] * v[MAT4_M10]
      + m[MAT4_M22] * v[MAT4_M20]
      + m[MAT4_M23] * v[MAT4_M30];

    t[MAT4_M21] = m[MAT4_M20] * v[MAT4_M01]
      + m[MAT4_M21] * v[MAT4_M11]
      + m[MAT4_M22] * v[MAT4_M21]
      + m[MAT4_M23] * v[MAT4_M31];

    t[MAT4_M22] = m[MAT4_M20] * v[MAT4_M02]
      + m[MAT4_M21] * v[MAT4_M12]
      + m[MAT4_M22] * v[MAT4_M22]
      + m[MAT4_M23] * v[MAT4_M32];

    t[MAT4_M23] = m[MAT4_M20] * v[MAT4_M03]
      + m[MAT4_M21] * v[MAT4_M13]
      + m[MAT4_M22] * v[MAT4_M23]
      + m[MAT4_M23] * v[MAT4_M33];

    t[MAT4_M30] = m[MAT4_M30] * v[MAT4_M00]
      + m[MAT4_M31] * v[MAT4_M10]
      + m[MAT4_M32] * v[MAT4_M20]
      + m[MAT4_M33] * v[MAT4_M30];

    t[MAT4_M31] = m[MAT4_M30] * v[MAT4_M01]
      + m[MAT4_M31] * v[MAT4_M11]
      + m[MAT4_M32] * v[MAT4_M21]
      + m[MAT4_M33] * v[MAT4_M31];

    t[MAT4_M32] = m[MAT4_M30] * v[MAT4_M02]
      + m[MAT4_M31] * v[MAT4_M12]
      + m[MAT4_M32] * v[MAT4_M22]
      + m[MAT4_M33] * v[MAT4_M32];

    t[MAT4_M33] = m[MAT4_M30] * v[MAT4_M03]
      + m[MAT4_M31] * v[MAT4_M13]
      + m[MAT4_M32] * v[MAT4_M23]
      + m[MAT4_M33] * v[MAT4_M33];

    self.set(&t)
  }

  pub fn idt(&mut self) -> &mut Self {
    self.identity()
  }

  pub fn look_at(&mut self, position: &Vec3, direction: &Vec3, up: &Vec3) -> &mut Self {
    self.z_axis.set_from(direction).normalize();
    self.x_axis.set_from(direction).normalize();
    self.x_axis.cross(up).normalize();
    self
      .y_axis
      .set_from(&self.x_axis)
      .cross(&self.z_axis)
      .normalize();
    self.identity();

    let mut v = self.values;
    v[MAT4_M00] = self.x_axis.x;
    v[MAT4_M01] = self.x_axis.y;
    v[MAT4_M02] = self.x_axis.z;
    v[MAT4_M10] = self.y_axis.x;
    v[MAT4_M11] = self.y_axis.y;
    v[MAT4_M12] = self.y_axis.z;

    v[MAT4_M20] = -self.z_axis.x;
    v[MAT4_M21] = -self.z_axis.y;
    v[MAT4_M22] = -self.z_axis.z;

    self.set(&v);

    let mut temp_mat4 = Mat4::new();
    temp_mat4.identity();
    temp_mat4.translate(-position.x, -position.y, -position.z);

    self.multiply(&temp_mat4)
  }

  pub fn set_to_look_at(&mut self, position: &Vec3, target: &Vec3, up: &Vec3) -> &mut Self {
    let mut tmp_vec = Vec3::new(0., 0., 0.);
    tmp_vec.set_from(target).sub(position);
    self.look_at(position, &tmp_vec, up)
  }

  pub fn rotate(&mut self, rotation: &Quaternion) -> &mut Self {
    let mut val = self.values;

    let x = rotation.x;
    let y = rotation.y;
    let z = rotation.z;
    let w = rotation.w;

    let xx = x * x;
    let xy = x * y;
    let xz = x * z;
    let xw = x * w;

    let yy = y * y;
    let yz = y * z;
    let yw = y * w;

    let zz = z * z;
    let zw = z * w;

    let r00 = 1.0 - 2.0 * (yy + zz);
    let r01 = 2.0 * (xy - zw);
    let r02 = 2.0 * (xz + yw);

    let r10 = 2.0 * (xy + zw);
    let r11 = 1.0 - 2.0 * (xx + zz);
    let r12 = 2.0 * (yz - xw);

    let r20 = 2.0 * (xz - yw);
    let r21 = 2.0 * (yz + xw);
    let r22 = 1.0 - 2.0 * (xx + yy);

    let m00 = val[MAT4_M00] * r00 + val[MAT4_M01] * r10 + val[MAT4_M02] * r20;
    let m01 = val[MAT4_M00] * r01 + val[MAT4_M01] * r11 + val[MAT4_M02] * r21;
    let m02 = val[MAT4_M00] * r02 + val[MAT4_M01] * r12 + val[MAT4_M02] * r22;

    let m10 = val[MAT4_M10] * r00 + val[MAT4_M11] * r10 + val[MAT4_M12] * r20;
    let m11 = val[MAT4_M10] * r01 + val[MAT4_M11] * r11 + val[MAT4_M12] * r21;
    let m12 = val[MAT4_M10] * r02 + val[MAT4_M11] * r12 + val[MAT4_M12] * r22;

    let m20 = val[MAT4_M20] * r00 + val[MAT4_M21] * r10 + val[MAT4_M22] * r20;
    let m21 = val[MAT4_M20] * r01 + val[MAT4_M21] * r11 + val[MAT4_M22] * r21;
    let m22 = val[MAT4_M20] * r02 + val[MAT4_M21] * r12 + val[MAT4_M22] * r22;

    let m30 = val[MAT4_M30] * r00 + val[MAT4_M31] * r10 + val[MAT4_M32] * r20;
    let m31 = val[MAT4_M30] * r01 + val[MAT4_M31] * r11 + val[MAT4_M32] * r21;
    let m32 = val[MAT4_M30] * r02 + val[MAT4_M31] * r12 + val[MAT4_M32] * r22;

    val[MAT4_M00] = m00;
    val[MAT4_M01] = m01;
    val[MAT4_M02] = m02;

    val[MAT4_M10] = m10;
    val[MAT4_M11] = m11;
    val[MAT4_M12] = m12;

    val[MAT4_M20] = m20;
    val[MAT4_M21] = m21;
    val[MAT4_M22] = m22;

    val[MAT4_M30] = m30;
    val[MAT4_M31] = m31;
    val[MAT4_M32] = m32;

    self.set(&val)
  }

  pub fn scale(&mut self, scale_x: f32, scale_y: f32, scale_z: f32) -> &mut Self {
    let mut val = self.values;
    val[MAT4_M00] *= scale_x;
    val[MAT4_M01] *= scale_y;
    val[MAT4_M02] *= scale_z;
    val[MAT4_M10] *= scale_x;
    val[MAT4_M11] *= scale_y;
    val[MAT4_M12] *= scale_z;
    val[MAT4_M20] *= scale_x;
    val[MAT4_M21] *= scale_y;
    val[MAT4_M22] *= scale_z;
    val[MAT4_M30] *= scale_x;
    val[MAT4_M31] *= scale_y;
    val[MAT4_M32] *= scale_z;

    self.set(&val)
  }

  pub fn set_to_rotation_rad(&mut self, axis: &Vec3, radians: f32) -> &mut Self {
    if is_zero(radians) {
      return self.idt();
    }
    let mut quat = Quaternion::new(0., 0., 0., 0.);
    quat.set_from_axis_rad(axis.x, axis.y, axis.z, radians);
    self.set_from_quaternion(&quat)
  }

  pub fn set_to_rotation_deg(&mut self, axis: &Vec3, degrees: f32) -> &mut Self {
    self.set_to_rotation_rad(axis, degrees.to_radians())
  }

  pub fn mat4_proj<'a>(values: &[f32; 16], vec: &'a mut Vec3) -> &'a mut Vec3 {
    let inv_w = 1.0
      / (vec.x * values[MAT4_M30]
        + vec.y * values[MAT4_M31]
        + vec.z * values[MAT4_M32]
        + values[MAT4_M33]);
    let x = (vec.x * values[MAT4_M00]
      + vec.y * values[MAT4_M01]
      + vec.z * values[MAT4_M02]
      + values[MAT4_M03])
      * inv_w;
    let y = (vec.x * values[MAT4_M10]
      + vec.y * values[MAT4_M11]
      + vec.z * values[MAT4_M12]
      + values[MAT4_M13])
      * inv_w;
    let z = (vec.x * values[MAT4_M20]
      + vec.y * values[MAT4_M21]
      + vec.z * values[MAT4_M22]
      + values[MAT4_M23])
      * inv_w;
    vec.x = x;
    vec.y = y;
    vec.z = z;
    vec
  }

  pub fn prj<'a>(
    mat: &Mat4,
    vecs: &'a mut [f32],
    offset: usize,
    num_vecs: usize,
    stride: usize,
  ) -> &'a mut [f32] {
    let mut vec = Vec3::new(0., 0., 0.);
    for i in 0..num_vecs {
      let start = offset + i * stride;
      vec.x = vecs[start];
      vec.y = vecs[start + 1];
      vec.z = vecs[start + 2];

      Mat4::mat4_proj(&mat.values, &mut vec);
      vecs[start] = vec.x;
      vecs[start + 1] = vec.y;
      vecs[start + 2] = vec.z;
    }
    vecs
  }
}
