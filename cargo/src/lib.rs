use gdx::{
  g2d::{
    batcher::PolygonBatch,
    texture::{ImageData, Texture},
  },
  misc::frame_counter::FrameCounter,
};
use glow::*;
use image::io::Reader;
use rand::Rng;
use std::rc::Rc;

pub mod gdx;

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(not(target_os = "ios"))]
pub mod glutin;

#[derive(Debug)]
struct Sprite {
  x: f32,
  y: f32,
  speed_x: f32,
  speed_y: f32,
}

pub trait CommonAppHandler {
  fn set_update_fn(&mut self, update: Box<dyn FnMut(&Rc<Context>, f32) -> () + 'static>);
  fn get_width(&self) -> f32;
  fn get_height(&self) -> f32;
  fn get_file_path(&self, file_name: &str) -> String;
}

pub fn init_game<F>(init_func: F)
where
  F: FnOnce(&mut dyn CommonAppHandler, &Rc<Context>) -> () + 'static,
{
  #[cfg(target_os = "ios")]
  ios::start_ios(init_func);

  #[cfg(not(target_os = "ios"))]
  glutin::start_glutin(init_func);
}

#[no_mangle]
pub extern "C" fn start_app() {
  unsafe {
    init_game(|app, gl| {
      let width = app.get_width();
      let height = app.get_height();
      let mut batch = PolygonBatch::create(&gl);
      batch.set_y_down(true);

      let mut camera = crate::gdx::g2d::ortho_cam::OrthoCamera::new(width, height, width, height);
      camera.set_position(width / 2., height / 2.);
      camera.set_y_down(true);
      camera.update();

      let fox = {
        Reader::open(app.get_file_path("fox.png"))
          .unwrap()
          .decode()
          .unwrap()
      };
      let fox = Texture::new(
        &gl,
        ImageData {
          width: fox.width(),
          height: fox.height(),
          data: fox.as_rgba8().unwrap(),
        },
      );

      gl.clear_color(0.5, 0.8, 0.2, 1.);

      let mut sprites = Vec::<Sprite>::new();
      let mut rng = rand::thread_rng();

      for _i in 0..100 {
        sprites.push(Sprite {
          x: rng.gen::<f32>() * width,
          y: rng.gen::<f32>() * height,
          speed_x: rng.gen::<f32>() * width - width / 2.,
          speed_y: rng.gen::<f32>() * height - height / 2.,
        });
      }

      let draw_width = 50.;
      let draw_height = 50.;
      let mut accumulate = 0.;
      let mut frame_counter = FrameCounter::new();

      app.set_update_fn(Box::new(move |gl, delta| {
        accumulate += delta;
        frame_counter.update();
        if accumulate >= 1. {
          println!("fps: {}", frame_counter.fps());
          accumulate = 0.;
        }

        gl.clear(COLOR_BUFFER_BIT);

        gl.clear(COLOR_BUFFER_BIT);
        batch.set_projection(&camera.combined);
        batch.begin();
        for sprite in &mut sprites {
          sprite.x += sprite.speed_x * delta;
          sprite.y += sprite.speed_y * delta;
          if sprite.x >= width {
            sprite.x = width;
            sprite.speed_x = -sprite.speed_x;
          } else if sprite.x <= 0. {
            sprite.x = 0.;
            sprite.speed_x = -sprite.speed_x;
          }
          if sprite.y >= height {
            sprite.y = height;
            sprite.speed_y = -sprite.speed_y;
          } else if sprite.y <= 0. {
            sprite.y = 0.;
            sprite.speed_y = -sprite.speed_y;
          }
          batch.draw(
            &fox,
            sprite.x - draw_width / 2.,
            sprite.y - draw_height / 2.,
            draw_width,
            draw_height,
          );
        }
        batch.end();
      }));
    });
  }
}
