use super::mat3::Mat3;

pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  pub fn zero() -> Self {
    Self { x: 0.0, y: 0.0 }
  }

  pub fn cpy(&self) -> Self {
    Self {
      x: self.x,
      y: self.y,
    }
  }

  pub fn get_x(&self) -> f32 {
    self.x
  }

  pub fn get_y(&self) -> f32 {
    self.y
  }

  pub fn set(&mut self, x: f32, y: f32) -> &mut Self {
    self.x = x;
    self.y = y;
    self
  }

  pub fn set_vector(&mut self, v: &Vec2) -> &mut Self {
    self.x = v.x;
    self.y = v.y;
    self
  }

  pub fn add(&mut self, x: f32, y: f32) -> &mut Self {
    self.x += x;
    self.y += y;
    self
  }

  pub fn add_vector(&mut self, v: &Vec2) -> &mut Self {
    self.x += v.x;
    self.y += v.y;
    self
  }

  pub fn sub(&mut self, x: f32, y: f32) -> &mut Self {
    self.x -= x;
    self.y -= y;
    self
  }

  pub fn sub_vector(&mut self, v: &Vec2) -> &mut Self {
    self.x -= v.x;
    self.y -= v.y;
    self
  }

  pub fn scale(&mut self, s: f32) -> &mut Self {
    self.x *= s;
    self.y *= s;
    self
  }

  pub fn dot(&self, v: &Vec2) -> f32 {
    self.x * v.x + self.y * v.y
  }

  pub fn len2(&self) -> f32 {
    self.dot(self)
  }

  pub fn len(&self) -> f32 {
    self.len2().sqrt()
  }

  pub fn distance2(&self, v: &Vec2) -> f32 {
    let x = self.x - v.x;
    let y = self.y - v.y;
    x * x + y * y
  }

  pub fn distance(&self, v: &Vec2) -> f32 {
    self.distance2(v).sqrt()
  }

  pub fn angle(&self) -> f32 {
    self.y.atan2(self.x)
  }

  pub fn normalize(&mut self) -> &mut Self {
    let len = self.len();
    if len > 0.0 {
      self.scale(1.0 / len);
    }
    self
  }

  pub fn rotate_rad(&mut self, rad: f32) -> &mut Self {
    let cos = rad.cos();
    let sin = rad.sin();
    let x = self.x * cos - self.y * sin;
    let y = self.x * sin + self.y * cos;
    self.x = x;
    self.y = y;
    self
  }

  pub fn rotate_deg(&mut self, deg: f32) -> &mut Self {
    self.rotate_rad(deg.to_radians())
  }

  pub fn lerp(&mut self, v: &Vec2, t: f32) -> &mut Self {
    let x = self.x + (v.x - self.x) * t;
    let y = self.y + (v.y - self.y) * t;
    self.x = x;
    self.y = y;
    self
  }

  pub fn mul(&mut self, mat: &Mat3) -> &mut Self {
    let x = self.x * mat.values[0] + self.y * mat.values[3] + mat.values[6];
    let y = self.x * mat.values[1] + self.y * mat.values[4] + mat.values[7];
    self.x = x;
    self.y = y;
    self
  }
}
