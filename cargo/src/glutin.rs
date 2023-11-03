use glow::*;
use std::rc::Rc;

use crate::{gdx::misc::frame_counter::FrameCounter, CommonAppHandler};

struct GlutinAppHandler {
  update: Option<Box<dyn FnMut(&Rc<Context>, f32) -> ()>>,
  width: f32,
  height: f32,
  frame_counter: FrameCounter,
}

impl GlutinAppHandler {
  pub fn new(width: f32, height: f32) -> Self {
    Self {
      update: None,
      width,
      height,
      frame_counter: FrameCounter::new(),
    }
  }
  pub fn update(&mut self, gl: &Rc<Context>) {
    let delta = self.frame_counter.update();
    match self.update {
      Some(ref mut update) => {
        update(gl, delta);
      }
      None => (),
    }
  }
}

impl CommonAppHandler for GlutinAppHandler {
  fn set_update_fn(&mut self, update: Box<dyn FnMut(&Rc<Context>, f32) -> () + 'static>) {
    self.update = Some(update);
  }

  fn get_width(&self) -> f32 {
    self.width
  }

  fn get_height(&self) -> f32 {
    self.height
  }

  fn get_file_path(&self, file_name: &str) -> String {
    format!("assets/{}", file_name)
  }
}

pub fn start_glutin<F>(init_func: F)
where
  F: FnOnce(&mut dyn CommonAppHandler, &Rc<Context>) -> () + 'static,
{
  unsafe {
    let (gl, _shader_version, window, event_loop) = {
      let event_loop = glutin::event_loop::EventLoop::new();
      let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Hello triangle!")
        .with_inner_size(glutin::dpi::LogicalSize::new(325.0, 768.0));
      let window = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap()
        .make_current()
        .unwrap();
      let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
      (gl, "#version 410", window, event_loop)
    };

    let gl = Rc::new(gl);
    let mut app = GlutinAppHandler::new(325.0, 768.0);
    init_func(&mut app, &gl);
    {
      use glutin::event::{Event, WindowEvent};
      use glutin::event_loop::ControlFlow;

      event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
          Event::LoopDestroyed => {
            return;
          }
          Event::MainEventsCleared => {
            window.window().request_redraw();
          }
          Event::RedrawRequested(_) => {
            app.update(&gl);
            window.swap_buffers().unwrap();
          }
          Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::Resized(physical_size) => {
              window.resize(*physical_size);
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
          },
          _ => (),
        }
      });
    }
  }
}
