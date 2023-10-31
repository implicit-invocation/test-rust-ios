use crate::gdx::math::{mat4::Mat4, vector2::Vec2, vector3::Vec3};

pub struct OrthoCamera {
  position: Vec3,
  direction: Vec3,
  up: Vec3,

  near: f32,
  far: f32,
  zoom: f32,

  viewport_width: f32,
  viewport_height: f32,

  projection_view: Mat4,
  inverse_projection_view: Mat4,

  projection: Mat4,
  view: Mat4,

  pub combined: [f32; 16],

  screen_width: f32,
  screen_height: f32,
  y_down: bool,
}

impl OrthoCamera {
  pub fn new(
    viewport_width: f32,
    viewport_height: f32,
    screen_width: f32,
    screen_height: f32,
  ) -> Self {
    let mut cam = Self {
      position: Vec3::new(0.0, 0.0, 0.0),
      direction: Vec3::new(0.0, 0.0, -1.0),
      up: Vec3::new(0.0, 1.0, 0.0),

      near: 0.0,
      far: 100.0,
      zoom: 1.0,

      viewport_width,
      viewport_height,

      projection_view: Mat4::new(),
      inverse_projection_view: Mat4::new(),

      projection: Mat4::new(),
      view: Mat4::new(),

      combined: [0.0; 16],

      screen_width,
      screen_height,
      y_down: true,
    };
    OrthoCamera::update_cam(&mut cam);
    cam
  }

  pub fn get_y_down(&self) -> bool {
    self.y_down
  }

  pub fn set_y_down(&mut self, y_down: bool) -> &mut Self {
    self.y_down = y_down;
    if y_down {
      self.up.set(0.0, -1.0, 0.0);
      self.direction.set(0., 0., 1.);
    } else {
      self.up.set(0.0, 1.0, 0.0);
      self.direction.set(0., 0., -1.);
    }
    self
  }

  pub fn resize(
    &mut self,
    viewport_width: f32,
    viewport_height: f32,
    screen_width: f32,
    screen_height: f32,
  ) -> &mut Self {
    OrthoCamera::resize_cam(
      self,
      viewport_width,
      viewport_height,
      screen_width,
      screen_height,
    );
    self
  }

  pub fn resize_cam(
    cam: &mut Self,
    viewport_width: f32,
    viewport_height: f32,
    screen_width: f32,
    screen_height: f32,
  ) {
    cam.viewport_width = viewport_width;
    cam.viewport_height = viewport_height;
    cam.screen_width = screen_width;
    cam.screen_height = screen_height;
    OrthoCamera::update_cam(cam);
  }

  pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
    self.position.set(x, y, 0.0);
    OrthoCamera::update_cam(self);
    self
  }

  pub fn update(&mut self) -> &mut Self {
    OrthoCamera::update_cam(self);
    self
  }

  pub fn update_cam(cam: &mut Self) {
    cam.projection.ortho(
      cam.zoom * (-cam.viewport_width / 2.),
      cam.zoom * (cam.viewport_width / 2.),
      cam.zoom * (-cam.viewport_height / 2.),
      cam.zoom * (cam.viewport_height / 2.),
      cam.near,
      cam.far,
    );
    cam.view.look_at(&cam.position, &cam.direction, &cam.up);
    cam.projection_view.set(&cam.projection.values);
    cam.projection_view.multiply(&cam.view);
    cam
      .inverse_projection_view
      .set(&cam.projection_view.values)
      .invert();
    cam.combined = cam.projection_view.values;
  }

  pub fn screen_to_world<'a>(
    screen_coords: &'a mut Vec3,
    screen_width: f32,
    screen_height: f32,
  ) -> &'a mut Vec3 {
    screen_coords.x = (screen_coords.x / screen_width) * 2.0 - 1.0;
    screen_coords.y = (screen_coords.y / screen_height) * 2.0 - 1.0;
    screen_coords
  }

  pub fn world_to_screen<'a>(
    world_coords: &'a mut Vec3,
    screen_width: f32,
    screen_height: f32,
  ) -> &'a mut Vec3 {
    world_coords.x = (world_coords.x + 1.0) / 2.0 * screen_width;
    world_coords.y = (world_coords.y + 1.0) / 2.0 * screen_height;
    world_coords.z = (world_coords.z + 1.) / 2.;
    world_coords
  }

  pub fn unproject_vec2<'a>(&self, world_coord: &'a mut Vec2, screen_coord: &Vec2) -> &'a mut Vec2 {
    let mut vec3 = Vec3::new(screen_coord.x, screen_coord.y, 0.0);
    Self::screen_to_world(&mut vec3, self.screen_width, self.screen_height);
    world_coord.set(vec3.x, vec3.y)
  }

  pub fn set_viewport(&mut self, viewport_width: f32, viewport_height: f32) -> &mut Self {
    self.viewport_width = viewport_width;
    self.viewport_height = viewport_height;
    OrthoCamera::update_cam(self);
    self
  }
}
