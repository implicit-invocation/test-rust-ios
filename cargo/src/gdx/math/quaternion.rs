use std::f32::consts::PI;

use super::common::{is_equal, is_zero};
use super::mat3::*;
use super::mat4::*;
use super::vector3::Vec3;

pub struct Quaternion {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Quaternion {
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }

  pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) -> &mut Self {
    self.x = x;
    self.y = y;
    self.z = z;
    self.w = w;
    self
  }

  pub fn set_from(&mut self, q: &Quaternion) -> &mut Self {
    self.x = q.x;
    self.y = q.y;
    self.z = q.z;
    self.w = q.w;
    self
  }

  pub fn len2(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
  }

  pub fn len(&self) -> f32 {
    self.len2().sqrt()
  }

  pub fn set_euler_angles_rad(&mut self, yaw: f32, pitch: f32, roll: f32) -> &mut Self {
    let half_yaw = yaw * 0.5;
    let half_pitch = pitch * 0.5;
    let half_roll = roll * 0.5;

    let sin_yaw = half_yaw.sin();
    let cos_yaw = half_yaw.cos();
    let sin_pitch = half_pitch.sin();
    let cos_pitch = half_pitch.cos();
    let sin_roll = half_roll.sin();
    let cos_roll = half_roll.cos();

    self.x = cos_roll * sin_pitch * cos_yaw + sin_roll * cos_pitch * sin_yaw;
    self.y = cos_roll * cos_pitch * sin_yaw - sin_roll * sin_pitch * cos_yaw;
    self.z = sin_roll * cos_pitch * cos_yaw - cos_roll * sin_pitch * sin_yaw;
    self.w = cos_roll * cos_pitch * cos_yaw + sin_roll * sin_pitch * sin_yaw;

    self
  }

  pub fn set_euler_angles_deg(&mut self, yaw: f32, pitch: f32, roll: f32) -> &mut Self {
    self.set_euler_angles_rad(yaw.to_radians(), pitch.to_radians(), roll.to_radians())
  }

  pub fn get_gimbal_pole(&self) -> f32 {
    let pitch = self.y;
    let yaw = self.x;
    let t = pitch * yaw;
    if t > 0.499 {
      1.0
    } else if t < -0.499 {
      -1.0
    } else {
      0.
    }
  }

  pub fn get_roll_rad(&self) -> f32 {
    let pole = self.get_gimbal_pole();
    if pole == 0. {
      self.y.atan2(self.z)
    } else {
      pole
        * 2.
        * (self.w * self.x + self.y * self.z).atan2(1. - 2. * (self.x * self.x + self.y * self.y))
    }
  }

  pub fn get_roll_deg(&self) -> f32 {
    self.get_roll_rad().to_degrees()
  }

  pub fn get_pitch_rad(&self) -> f32 {
    let pole = self.get_gimbal_pole();
    if pole == 0. {
      -2.
        * (self.x * self.z - self.w * self.y)
          .atan2(self.w * self.w + self.x * self.x - self.y * self.y - self.z * self.z)
    } else {
      pole * PI * 0.5
    }
  }

  pub fn get_pitch_deg(&self) -> f32 {
    self.get_pitch_rad().to_degrees()
  }

  pub fn get_yaw_rad(&self) -> f32 {
    let pole = self.get_gimbal_pole();
    if pole == 0. {
      self.x.atan2(self.w)
    } else {
      pole
        * 2.
        * (self.x * self.y + self.w * self.z).atan2(1. - 2. * (self.x * self.x + self.z * self.z))
    }
  }

  pub fn get_yaw_deg(&self) -> f32 {
    self.get_yaw_rad().to_degrees()
  }

  pub fn normalize(&mut self) -> &mut Self {
    let len = self.len();
    if len > 0.0 {
      self.w /= len;
      self.x /= len;
      self.y /= len;
      self.z /= len;
    }
    self
  }

  pub fn conjugate(&mut self) -> &mut Self {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
    self
  }

  pub fn transform<'a>(&self, v: &'a mut Vec3) -> &'a mut Vec3 {
    let mut tmp2 = Quaternion {
      x: self.x,
      y: self.y,
      z: self.z,
      w: self.w,
    };
    tmp2.conjugate();

    let quaternion = Quaternion::new(v.x, v.y, v.z, 0.0);

    tmp2
      .mul_left(quaternion.x, quaternion.y, quaternion.z, quaternion.w)
      .mul_left(self.x, self.y, self.z, self.w);

    v.x = tmp2.x;
    v.y = tmp2.y;
    v.z = tmp2.z;
    v
  }

  pub fn mul(&mut self, x: f32, y: f32, z: f32, w: f32) -> &mut Self {
    let x_ = self.w * x + self.x * w + self.y * z - self.z * y;
    let y_ = self.w * y + self.y * w + self.z * x - self.x * z;
    let z_ = self.w * z + self.z * w + self.x * y - self.y * x;
    let w_ = self.w * w - self.x * x - self.y * y - self.z * z;
    self.x = x_;
    self.y = y_;
    self.z = z_;
    self.w = w_;
    self
  }

  pub fn mul_left(&mut self, x: f32, y: f32, z: f32, w: f32) -> &mut Self {
    let x_ = x * self.w + y * self.z - z * self.y + w * self.x;
    let y_ = -x * self.z + y * self.w + z * self.x + w * self.y;
    let z_ = x * self.y - y * self.x + z * self.w + w * self.z;
    let w_ = -x * self.x - y * self.y - z * self.z + w * self.w;
    self.x = x_;
    self.y = y_;
    self.z = z_;
    self.w = w_;
    self
  }

  pub fn mul_quaternion(&mut self, q: &Quaternion) -> &mut Self {
    self.mul(q.x, q.y, q.z, q.w)
  }

  pub fn mul_left_quaternion(&mut self, q: &Quaternion) -> &mut Self {
    self.mul_left(q.x, q.y, q.z, q.w)
  }

  pub fn add(&mut self, qx: f32, qy: f32, qz: f32, qw: f32) -> &mut Self {
    self.x += qx;
    self.y += qy;
    self.z += qz;
    self.w += qw;
    self
  }

  pub fn to_matrix_values<'a>(&self, matrix: &'a mut [f32; 16]) -> &'a mut [f32; 16] {
    let xx = self.x * self.x;
    let xy = self.x * self.y;
    let xz = self.x * self.z;
    let xw = self.x * self.w;
    let yy = self.y * self.y;
    let yz = self.y * self.z;
    let yw = self.y * self.w;
    let zz = self.z * self.z;
    let zw = self.z * self.w;

    matrix[MAT4_M00] = 1.0 - 2.0 * (yy + zz);
    matrix[MAT4_M01] = 2.0 * (xy - zw);
    matrix[MAT4_M02] = 2.0 * (xz + yw);
    matrix[MAT4_M03] = 0.0;

    matrix[MAT4_M10] = 2.0 * (xy + zw);
    matrix[MAT4_M11] = 1.0 - 2.0 * (xx + zz);
    matrix[MAT4_M12] = 2.0 * (yz - xw);
    matrix[MAT4_M13] = 0.0;

    matrix[MAT4_M20] = 2.0 * (xz - yw);
    matrix[MAT4_M21] = 2.0 * (yz + xw);
    matrix[MAT4_M22] = 1.0 - 2.0 * (xx + yy);
    matrix[MAT4_M23] = 0.0;

    matrix[MAT4_M30] = 0.0;
    matrix[MAT4_M31] = 0.0;
    matrix[MAT4_M32] = 0.0;
    matrix[MAT4_M33] = 1.0;
    matrix
  }

  pub fn idt(&mut self) -> &mut Self {
    self.set(0., 0., 0., 1.);
    self
  }

  pub fn is_identity(&self) -> bool {
    is_zero(self.x) && is_zero(self.y) && is_zero(self.z) && is_equal(self.w, 1.)
  }

  pub fn set_from_axis_deg(&mut self, x: f32, y: f32, z: f32, degrees: f32) -> &mut Self {
    self.set_from_axis_rad(x, y, z, degrees.to_radians())
  }

  pub fn set_from_axis_rad(&mut self, x: f32, y: f32, z: f32, radians: f32) -> &mut Self {
    let mut d = Vec3::len_of(x, y, z);
    if is_zero(d) {
      return self.idt();
    }
    d = 1. / d;
    let l_ang = if radians < 0. {
      PI * 2. - (-radians % (PI * 2.))
    } else {
      radians % (PI * 2.)
    };
    let l_sin = l_ang.sin();
    let l_cos = l_ang.cos();
    self
      .set(d * x * l_sin, d * y * l_sin, d * z * l_sin, l_cos)
      .normalize()
  }

  pub fn set_from_axes(
    &mut self,
    mut xx: f32,
    mut xy: f32,
    mut xz: f32,
    mut yx: f32,
    mut yy: f32,
    mut yz: f32,
    mut zx: f32,
    mut zy: f32,
    mut zz: f32,
    normalize_axes: bool,
  ) -> &mut Self {
    if normalize_axes {
      let lx = 1. / Vec3::len_of(xx, xy, xz);
      let ly = 1. / Vec3::len_of(yx, yy, yz);
      let lz = 1. / Vec3::len_of(zx, zy, zz);
      xx *= lx;
      xy *= lx;
      xz *= lx;
      yx *= ly;
      yy *= ly;
      yz *= ly;
      zx *= lz;
      zy *= lz;
      zz *= lz;
    }
    let t = xx + yy + zz;

    if t >= 0.0 {
      let mut s = (t + 1.0).sqrt();
      self.w = 0.5 * s;
      s = 0.5 / s;
      self.x = (zy - yz) * s;
      self.y = (xz - zx) * s;
      self.z = (yx - xy) * s;
    } else if xx > yy && xx > zz {
      let mut s = (1.0 + xx - yy - zz).sqrt();
      self.x = s * 0.5;
      s = 0.5 / s;
      self.y = (yx + xy) * s;
      self.z = (xz + zx) * s;
      self.w = (zy - yz) * s;
    } else if yy > zz {
      let mut s = (1.0 + yy - xx - zz).sqrt();
      self.y = s * 0.5;
      s = 0.5 / s;
      self.x = (yx + xy) * s;
      self.z = (zy + yz) * s;
      self.w = (xz - zx) * s;
    } else {
      let mut s = (1.0 + zz - xx - yy).sqrt();
      self.z = s * 0.5;
      s = 0.5 / s;
      self.x = (xz + zx) * s;
      self.y = (zy + yz) * s;
      self.w = (yx - xy) * s;
    }
    self
  }

  pub fn set_from_mat4(&mut self, mat4: &Mat4, normalize_axes: bool) -> &mut Self {
    self.set_from_axes(
      mat4.values[MAT4_M00],
      mat4.values[MAT4_M01],
      mat4.values[MAT4_M02],
      mat4.values[MAT4_M10],
      mat4.values[MAT4_M11],
      mat4.values[MAT4_M12],
      mat4.values[MAT4_M20],
      mat4.values[MAT4_M21],
      mat4.values[MAT4_M22],
      normalize_axes,
    )
  }

  pub fn set_from_mat3(&mut self, mat3: &Mat3) -> &mut Self {
    self.set_from_axes(
      mat3.values[MAT3_M00],
      mat3.values[MAT3_M01],
      mat3.values[MAT3_M02],
      mat3.values[MAT3_M10],
      mat3.values[MAT3_M11],
      mat3.values[MAT3_M12],
      mat3.values[MAT3_M20],
      mat3.values[MAT3_M21],
      mat3.values[MAT3_M22],
      true,
    )
  }

  pub fn set_from_cross(&mut self, v1: &Vec3, v2: &Vec3) -> &mut Self {
    let dot = v1.dot(v2).clamp(-1., 1.);
    let angle = dot.acos();
    self.set_from_axis_rad(
      v1.y * v2.z - v1.z * v2.y,
      v1.z * v2.x - v1.x * v2.z,
      v1.x * v2.y - v1.y * v2.x,
      angle,
    )
  }

  pub fn slerp(&mut self, end: &Quaternion, alpha: f32) -> &mut Self {
    let d = self.x * end.x + self.y * end.y + self.z * end.z + self.w * end.w;
    let abs_dot = if d < 0. { -d } else { d };

    let scale0 = if (1. - abs_dot) > 0.0001 {
      (1. - alpha).sin() * (1. - abs_dot).atan2((1. - alpha).cos())
    } else {
      1. - alpha
    };
    let mut scale1 = if abs_dot > 0.0001 {
      alpha.sin() * abs_dot.atan2(alpha.cos())
    } else {
      alpha
    };

    if d < 0. {
      scale1 = -scale1;
    }

    self.x = scale0 * self.x + scale1 * end.x;
    self.y = scale0 * self.y + scale1 * end.y;
    self.z = scale0 * self.z + scale1 * end.z;
    self.w = scale0 * self.w + scale1 * end.w;

    self
  }

  pub fn slerp_with_quaternions(&mut self, q: &[Self]) -> &mut Self {
    let w = 1. / q.len() as f32;
    self.set(q[0].x, q[0].y, q[0].z, q[0].w).exp(w);
    for i in 1..q.len() {
      let mut quat = Quaternion::new(q[i].x, q[i].y, q[i].z, q[i].w);
      quat.exp(w);

      self.mul(quat.x, quat.y, quat.z, quat.w);
    }
    self
  }

  pub fn exp(&mut self, alpha: f32) -> &mut Self {
    let norm = self.len();
    let norm_exp = norm.powf(alpha);

    let theta = (self.w / norm).acos();
    let coeff = if theta.abs() < 0.001 {
      norm_exp * alpha / norm
    } else {
      norm_exp * theta.sin() / norm
    };

    self.w *= norm_exp * theta.cos();
    self.x *= coeff;
    self.y *= coeff;
    self.z *= coeff;

    self.normalize()
  }

  pub fn dot(&mut self, x: f32, y: f32, z: f32, w: f32) -> f32 {
    self.x * x + self.y * y + self.z * z + self.w * w
  }

  pub fn dot_values(x1: f32, y1: f32, z1: f32, w1: f32, x2: f32, y2: f32, z2: f32, w2: f32) -> f32 {
    x1 * x2 + y1 * y2 + z1 * z2 + w1 * w2
  }

  pub fn mul_by_scalar(&mut self, scalar: f32) -> &mut Self {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
    self.w *= scalar;
    self
  }

  pub fn get_angle_rad(&self) -> f32 {
    if self.w > 1. {
      return 2.0 * self.w.acos() / self.len();
    }
    2. * self.w.acos()
  }

  pub fn get_angle_deg(&self) -> f32 {
    self.get_angle_rad().to_degrees()
  }
}
