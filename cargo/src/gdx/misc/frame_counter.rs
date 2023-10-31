use std::time::Instant;

pub struct FrameCounter {
  elapsed: f32,
  pub frames: u32,
  frames_in_last_second: u32,
  last_frame: Instant,
  fps: u32,
}

impl FrameCounter {
  pub fn new() -> Self {
    Self {
      elapsed: 0.0,
      frames: 0,
      frames_in_last_second: 0,
      last_frame: Instant::now(),
      fps: 0,
    }
  }

  pub fn update(&mut self) -> f32 {
    let now = Instant::now();
    let delta = now.duration_since(self.last_frame).as_secs_f32();
    self.last_frame = now;
    self.elapsed += delta;
    self.frames += 1;
    self.frames_in_last_second += 1;
    if self.elapsed >= 1.0 {
      self.elapsed = 0.0;
      self.fps = self.frames_in_last_second;
      self.frames_in_last_second = 0;
    }
    delta
  }

  pub fn fps(&self) -> u32 {
    self.fps
  }
}
