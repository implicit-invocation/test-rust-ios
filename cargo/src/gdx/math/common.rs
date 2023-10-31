pub const FLOAT_ROUNDING_ERROR: f32 = 0.000001;

pub fn is_zero(f: f32) -> bool {
  f.abs() <= FLOAT_ROUNDING_ERROR
}

pub fn is_equal(a: f32, b: f32) -> bool {
  (a - b).abs() <= FLOAT_ROUNDING_ERROR
}
