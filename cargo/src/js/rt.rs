use rquickjs::{bind, Context, Function, Object, Persistent, Runtime};
use std::rc::Rc;

use crate::CommonAppHandler;

#[bind(object)]
#[quickjs(bare)]
mod glow_js {
  use glow::*;
  use std::rc::Rc;

  pub struct JsContext {
    #[quickjs(skip)]
    pub gl: Rc<Context>,
  }

  impl JsContext {
    #[quickjs(proto)]
    pub const COLOR_BUFFER_BIT: u32 = glow::COLOR_BUFFER_BIT;
    #[quickjs(skip)]
    pub fn new(gl: &Rc<Context>) -> Self {
      Self { gl: gl.clone() }
    }
    pub fn clear(&self, mask: u32) {
      unsafe {
        self.gl.clear(mask);
      }
    }
    #[quickjs(rename = "clearColor")]
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
      unsafe {
        self.gl.clear_color(r, g, b, a);
      }
    }
  }
}

fn print(msg: String) {
  println!("{msg}");
}

fn print_obj(obj: Object) {
  println!("{:?}", obj);
}

pub fn init_js_runtime(app: &mut dyn CommonAppHandler, gl: &Rc<glow::Context>) {
  let js_gl_context = glow_js::JsContext::new(gl);
  let rt = Runtime::new().unwrap();
  let ctx = Context::full(&rt).unwrap();
  let update = ctx.with(|ctx| {
    let _ = ctx.globals().init_def::<GlowJs>().unwrap();
    let _ = ctx
      .globals()
      .set("print", Function::new(ctx.clone(), print).unwrap());
    let _ = ctx
      .globals()
      .set("printObj", Function::new(ctx.clone(), print_obj).unwrap());

    let start_module = ctx
      .compile(
        "main",
        r#"
    export const start = (gl) => {
      gl.clearColor(1.0, .2, 0.4, 1.0);
      let frame = 0;
      // setTimeout(() => {
      //   print('Hello from JS!');
      // }, 1000);
      return delta => {
        frame++;
        print(`Frame: ${frame}. Delta: ${delta}.`);
        gl.clear(gl.COLOR_BUFFER_BIT);
      };
    }
"#,
      )
      .unwrap();
    unsafe {
      let _ = start_module.eval().unwrap();
      let start: Function = start_module.get("start").unwrap();
      let update: Function = start.call::<_, Function>((js_gl_context,)).unwrap();
      Persistent::save(ctx, update)
    }
  });
  app.set_update_fn(Box::new(move |_gl, delta| {
    ctx.with(|ctx| {
      let _: () = update.clone().restore(ctx).unwrap().call((delta,)).unwrap();
    });
  }));
}
