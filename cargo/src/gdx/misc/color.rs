#[derive(Clone, Copy)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color {
  pub const WHITE: Color = Color::new(1., 1., 1., 1.);
  pub const BLACK: Color = Color::new(0., 0., 0., 1.);
  pub const RED: Color = Color::new(1., 0., 0., 1.);
  pub const GREEN: Color = Color::new(0., 1., 0., 1.);
  pub const BLUE: Color = Color::new(0., 0., 1., 1.);
  pub const LIGHT_GRAY: Color = Color::new(0.75, 0.75, 0.75, 1.);
  pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.);
  pub const DARK_GRAY: Color = Color::new(0.25, 0.25, 0.25, 1.);
  pub const PINK: Color = Color::new(1., 0.68, 0.68, 1.);
  pub const ORANGE: Color = Color::new(1., 0.78, 0., 1.);
  pub const YELLOW: Color = Color::new(1., 1., 0., 1.);
  pub const MAGENTA: Color = Color::new(1., 0., 1., 1.);
  pub const CYAN: Color = Color::new(0., 1., 1., 1.);
  pub const OLIVE: Color = Color::new(0.5, 0.5, 0., 1.);
  pub const PURPLE: Color = Color::new(0.5, 0., 0.5, 1.);
  pub const MAROON: Color = Color::new(0.5, 0., 0., 1.);
  pub const TEAL: Color = Color::new(0., 0.5, 0.5, 1.);
  pub const NAVY: Color = Color::new(0., 0., 0.5, 1.);
  pub const CORAL: Color = Color::new(1., 0.5, 0.31, 1.);
  pub const GOLD: Color = Color::new(1., 0.84, 0., 1.);
  pub const SKY: Color = Color::new(0.53, 0.81, 0.92, 1.);
  pub const LIME: Color = Color::new(0.2, 0.8, 0.2, 1.);
  pub const ROSE: Color = Color::new(1., 0.41, 0.71, 1.);
  pub const SALMON: Color = Color::new(0.98, 0.5, 0.45, 1.);
  pub const TAN: Color = Color::new(0.82, 0.71, 0.55, 1.);
  pub const FOREST: Color = Color::new(0.13, 0.55, 0.13, 1.);
  pub const AQUA: Color = Color::new(0., 1., 1., 1.);
  pub const VIOLET: Color = Color::new(0.93, 0.51, 0.93, 1.);
  pub const WHEAT: Color = Color::new(0.96, 0.87, 0.7, 1.);
  pub const WHITE_SMOKE: Color = Color::new(0.96, 0.96, 0.96, 1.);
  pub const FUCHSIA: Color = Color::new(1., 0., 1., 1.);
  pub const CHARTREUSE: Color = Color::new(0.5, 1., 0., 1.);
  pub const GREEN_YELLOW: Color = Color::new(0.68, 1., 0.18, 1.);
  pub const SPRING_GREEN: Color = Color::new(0., 1., 0.5, 1.);
  pub const INDIGO: Color = Color::new(0.29, 0., 0.51, 1.);
  pub const CRIMSON: Color = Color::new(0.86, 0.08, 0.24, 1.);
  pub const DARK_ORANGE: Color = Color::new(1., 0.55, 0., 1.);
  pub const DARK_ORCHID: Color = Color::new(0.6, 0.2, 0.8, 1.);
  pub const DARK_VIOLET: Color = Color::new(0.58, 0., 0.83, 1.);
  pub const DEEP_PINK: Color = Color::new(1., 0.08, 0.58, 1.);
  pub const FIREBRICK: Color = Color::new(0.7, 0.13, 0.13, 1.);
  pub const HOT_PINK: Color = Color::new(1., 0.41, 0.71, 1.);
  pub const KHAKI: Color = Color::new(0.94, 0.9, 0.55, 1.);
  pub const LIGHT_BLUE: Color = Color::new(0.68, 0.85, 0.9, 1.);
  pub const LIGHT_CORAL: Color = Color::new(0.94, 0.5, 0.5, 1.);
  pub const LIGHT_CYAN: Color = Color::new(0.88, 1., 1., 1.);
  pub const LIGHT_GREEN: Color = Color::new(0.56, 0.93, 0.56, 1.);
  pub const LIGHT_PINK: Color = Color::new(1., 0.71, 0.76, 1.);
  pub const LIGHT_SALMON: Color = Color::new(1., 0.63, 0.48, 1.);
  pub const LIGHT_SEA_GREEN: Color = Color::new(0.13, 0.7, 0.67, 1.);
  pub const LIGHT_SKY_BLUE: Color = Color::new(0.53, 0.81, 0.98, 1.);
  pub const LIGHT_SLATE_GRAY: Color = Color::new(0.47, 0.53, 0.6, 1.);
  pub const LIGHT_STEEL_BLUE: Color = Color::new(0.69, 0.77, 0.87, 1.);
  pub const LIME_GREEN: Color = Color::new(0.2, 0.8, 0.2, 1.);
  pub const MEDIUM_AQUAMARINE: Color = Color::new(0.4, 0.8, 0.67, 1.);
  pub const MEDIUM_BLUE: Color = Color::new(0., 0., 0.8, 1.);
  pub const MEDIUM_ORCHID: Color = Color::new(0.73, 0.33, 0.83, 1.);
  pub const MEDIUM_PURPLE: Color = Color::new(0.58, 0.44, 0.86, 1.);
  pub const MEDIUM_SEA_GREEN: Color = Color::new(0.24, 0.7, 0.44, 1.);
  pub const MEDIUM_SLATE_BLUE: Color = Color::new(0.48, 0.41, 0.93, 1.);
  pub const MEDIUM_SPRING_GREEN: Color = Color::new(0., 0.98, 0.6, 1.);
  pub const MEDIUM_TURQUOISE: Color = Color::new(0.28, 0.82, 0.8, 1.);
  pub const MEDIUM_VIOLET_RED: Color = Color::new(0.78, 0.08, 0.52, 1.);
  pub const MIDNIGHT_BLUE: Color = Color::new(0.1, 0.1, 0.44, 1.);
  pub const MISTY_ROSE: Color = Color::new(1., 0.89, 0.88, 1.);
  pub const MOCCASIN: Color = Color::new(1., 0.89, 0.71, 1.);
  pub const NAVAJO_WHITE: Color = Color::new(1., 0.87, 0.68, 1.);
  pub const OLD_LACE: Color = Color::new(0.99, 0.96, 0.9, 1.);
  pub const OLIVE_DRAB: Color = Color::new(0.42, 0.56, 0.14, 1.);
  pub const ORANGE_RED: Color = Color::new(1., 0.27, 0., 1.);
  pub const ORCHID: Color = Color::new(0.85, 0.44, 0.84, 1.);
  pub const PALE_GOLDENROD: Color = Color::new(0.93, 0.91, 0.67, 1.);

  pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self { r, g, b, a }
  }

  pub fn set(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
    self.r = r;
    self.g = g;
    self.b = b;
    self.a = a;
    self.clamp()
  }

  pub fn unsafe_set(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
    self.r = r;
    self.g = g;
    self.b = b;
    self.a = a;
    self
  }

  pub fn clamp(&mut self) -> &mut Self {
    self.r = self.r.min(1.).max(0.);
    self.g = self.g.min(1.).max(0.);
    self.b = self.b.min(1.).max(0.);
    self.a = self.a.min(1.).max(0.);
    self
  }

  pub fn set_from_string(&mut self, s: &str) -> &mut Self {
    let mut s = String::from(s);
    if s.starts_with("#") {
      s.remove(0);
    }
    let r = &s[0..2].parse::<f32>().unwrap() / 255.;
    let g = &s[2..4].parse::<f32>().unwrap() / 255.;
    let b = &s[4..6].parse::<f32>().unwrap() / 255.;
    let a = if s.len() > 6 {
      &s[6..8].parse::<f32>().unwrap() / 255.
    } else {
      1.
    };
    self.set(r, g, b, a)
  }

  pub fn to_int_bits(&self) -> u32 {
    let r = (self.r * 255.) as u32;
    let g = (self.g * 255.) as u32;
    let b = (self.b * 255.) as u32;
    let a = (self.a * 255.) as u32;
    (a << 24) | (b << 16) | (g << 8) | r
  }

  pub fn lerp<'a>(target: &'a mut Color, from: &Color, to: &Color, t: f32) -> &'a mut Color {
    target.r = from.r + (to.r - from.r) * t;
    target.g = from.g + (to.g - from.g) * t;
    target.b = from.b + (to.b - from.b) * t;
    target.a = from.a + (to.a - from.a) * t;
    target
  }

  pub fn rgba8888_to_color(value: u32) -> Color {
    let r = ((value & 0xff000000) >> 24) as f32 / 255.;
    let g = ((value & 0x00ff0000) >> 16) as f32 / 255.;
    let b = ((value & 0x0000ff00) >> 8) as f32 / 255.;
    let a = (value & 0x000000ff) as f32 / 255.;
    Color::new(r, g, b, a)
  }

  pub fn rgb888_to_color(value: u32) -> Color {
    let r = ((value & 0x00ff0000) >> 16) as f32 / 255.;
    let g = ((value & 0x0000ff00) >> 8) as f32 / 255.;
    let b = (value & 0x000000ff) as f32 / 255.;
    Color::new(r, g, b, 1.)
  }

  pub fn add(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
    self.r += r;
    self.g += g;
    self.b += b;
    self.a += a;
    self.clamp()
  }

  pub fn mul(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
    self.r *= r;
    self.g *= g;
    self.b *= b;
    self.a *= a;
    self.clamp()
  }
}
