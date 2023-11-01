use rand::prelude::*;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;

use gdx::g2d::batcher::PolygonBatch;
use gdx::g2d::ortho_cam::OrthoCamera;
use gdx::g2d::texture::{ImageData, Texture};
use gdx::misc::frame_counter::FrameCounter;
use image::io::Reader;
use objc::declare::ClassDecl;
use objc::runtime::*;
use objc::*;

use glow::*;

pub mod gdx;

#[derive(Debug)]
struct Sprite {
  x: f32,
  y: f32,
  speed_x: f32,
  speed_y: f32,
}

macro_rules! msg_send_ {
  ($obj:expr, $name:ident) => ({
      let res: ObjcId = msg_send!($obj, $name);
      res
  });
  ($obj:expr, $($name:ident : $arg:expr)+) => ({
      let res: ObjcId = msg_send!($obj, $($name: $arg)*);
      res
  });
}

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
  let c_str = unsafe { CStr::from_ptr(to) };
  let recipient = match c_str.to_str() {
    Err(_) => "there",
    Ok(string) => string,
  };

  CString::new("Hello ".to_owned() + recipient)
    .unwrap()
    .into_raw()
}

#[no_mangle]
pub extern "C" fn rust_greeting_free(s: *mut c_char) {
  unsafe {
    if s.is_null() {
      return;
    }
    let _ = CString::from_raw(s);
  };
}

pub type ObjcId = *mut Object;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSPoint {
  pub x: f64,
  pub y: f64,
}

unsafe impl Encode for NSPoint {
  fn encode() -> Encoding {
    let encoding = format!(
      "{{CGPoint={}{}}}",
      f64::encode().as_str(),
      f64::encode().as_str()
    );
    unsafe { Encoding::from_str(&encoding) }
  }
}

#[repr(C)]
#[derive(Copy, Debug, Clone)]
pub struct NSSize {
  pub width: f64,
  pub height: f64,
}

unsafe impl Encode for NSSize {
  fn encode() -> Encoding {
    let encoding = format!(
      "{{CGSize={}{}}}",
      f64::encode().as_str(),
      f64::encode().as_str()
    );
    unsafe { Encoding::from_str(&encoding) }
  }
}

#[repr(C)]
#[derive(Copy, Debug, Clone)]
pub struct NSRect {
  pub origin: NSPoint,
  pub size: NSSize,
}
impl NSRect {
  pub fn new(x: f64, y: f64, w: f64, h: f64) -> NSRect {
    NSRect {
      origin: NSPoint { x, y },
      size: NSSize {
        width: w,
        height: h,
      },
    }
  }
}
unsafe impl Encode for NSRect {
  fn encode() -> Encoding {
    let encoding = format!(
      "{{CGRect={}{}}}",
      NSPoint::encode().as_str(),
      NSSize::encode().as_str()
    );
    unsafe { Encoding::from_str(&encoding) }
  }
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod frameworks {
  pub const GLKViewDrawableColorFormatRGBA8888: i32 = 0;
  #[repr(i32)]
  pub enum GLKViewDrawableDepthFormat {
    FormatNone = 0,
    Format16,
    Format24,
  }
  #[repr(i32)]
  pub enum GLKViewDrawableStencilFormat {
    FormatNone = 0,
    Format8,
  }
}

struct View {
  view: ObjcId,
  view_ctrl: ObjcId,
  // this view failed to create gles3 context, but succeeded with gles2
  _gles2: bool,
}

pub fn define_glk_or_mtk_view(superclass: &Class) -> *const Class {
  let mut decl = ClassDecl::new("QuadView", superclass).unwrap();
  decl.add_ivar::<*mut c_void>("display_ptr");
  return decl.register();
}

unsafe fn create_opengl_view(screen_rect: NSRect, _sample_count: i32, high_dpi: bool) -> View {
  let glk_view_obj: ObjcId = msg_send![define_glk_or_mtk_view(class!(GLKView)), alloc];
  let glk_view_obj: ObjcId = msg_send![glk_view_obj, initWithFrame: screen_rect];

  let eagl_context_obj: ObjcId = msg_send![class!(EAGLContext), alloc];
  let mut eagl_context_obj: ObjcId = msg_send![eagl_context_obj, initWithAPI: 3];
  let mut gles2 = false;
  if eagl_context_obj.is_null() {
    eagl_context_obj = msg_send![eagl_context_obj, initWithAPI: 2];
    gles2 = true;
  }

  msg_send_![
      glk_view_obj,
      setDrawableColorFormat: frameworks::GLKViewDrawableColorFormatRGBA8888
  ];
  msg_send_![
      glk_view_obj,
      setDrawableDepthFormat: frameworks::GLKViewDrawableDepthFormat::Format24 as i32
  ];
  msg_send_![
      glk_view_obj,
      setDrawableStencilFormat: frameworks::GLKViewDrawableStencilFormat::FormatNone as i32
  ];
  msg_send_![glk_view_obj, setContext: eagl_context_obj];

  msg_send_![glk_view_obj, setEnableSetNeedsDisplay: NO];
  msg_send_![glk_view_obj, setUserInteractionEnabled: YES];
  msg_send_![glk_view_obj, setMultipleTouchEnabled: YES];
  if high_dpi {
    msg_send_![glk_view_obj, setContentScaleFactor: 2.0];
  } else {
    msg_send_![glk_view_obj, setContentScaleFactor: 1.0];
  }

  let superclass = class!(GLKViewController);
  let mut decl = ClassDecl::new("MyGLKViewController", superclass).unwrap();
  decl.add_ivar::<*mut c_void>("game");
  decl.add_ivar::<*mut c_void>("size_ptr");

  extern "C" fn update(this: &mut Object, _: Sel) {
    unsafe {
      let mut game_ptr: *mut c_void = *this.get_ivar("game");
      if game_ptr.is_null() {
        let size_ptr: *const c_void = *this.get_ivar("size_ptr");
        let size_ptr = size_ptr as *const NSSize;
        let size = *size_ptr;

        let game = MyGame::new(size.width as f32, size.height as f32);
        this.set_ivar("game", Box::into_raw(Box::new(game)) as *mut c_void);
        game_ptr = *this.get_ivar("game");
      }

      let game = &mut *(game_ptr as *mut MyGame);
      game.update();
    };
  }
  decl.add_method(
    sel!(update),
    update as extern "C" fn(&mut Object, Sel) -> (),
  );

  let my_controller_class = decl.register();

  let view_ctrl_obj: ObjcId = msg_send![my_controller_class, alloc];
  let view_ctrl_obj: ObjcId = msg_send![view_ctrl_obj, init];
  (*view_ctrl_obj).set_ivar(
    "size_ptr",
    Box::into_raw(Box::new(screen_rect.size)) as *const c_void,
  );

  msg_send_![view_ctrl_obj, setView: glk_view_obj];
  msg_send_![view_ctrl_obj, setPreferredFramesPerSecond:60];

  View {
    view: glk_view_obj,
    view_ctrl: view_ctrl_obj,
    _gles2: gles2,
  }
}

#[allow(dead_code)]
unsafe fn get_proc_address(name: *const u8) -> *const c_void {
  mod libc {
    use std::ffi::{c_char, c_int, c_void};

    pub const RTLD_LAZY: c_int = 1;
    extern "C" {
      pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
      pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }
  }
  static mut OPENGL: *mut std::ffi::c_void = std::ptr::null_mut();

  if OPENGL.is_null() {
    OPENGL = libc::dlopen(
      b"/System/Library/Frameworks/OpenGLES.framework/OpenGLES\0".as_ptr() as _,
      libc::RTLD_LAZY,
    );
  }

  assert!(!OPENGL.is_null());

  let symbol = libc::dlsym(OPENGL, name as _);
  if symbol.is_null() {
    return ptr::null();
  }
  unsafe { std::mem::transmute_copy(&symbol) }
}

fn define_app_delegate() -> *const Class {
  let superclass = class!(NSObject);
  let mut decl = ClassDecl::new("NSAppDelegate", superclass).unwrap();

  extern "C" fn did_finish_launching_with_options(
    _: &Object,
    _: Sel,
    _: ObjcId,
    _: ObjcId,
  ) -> BOOL {
    unsafe {
      let main_screen: ObjcId = msg_send![class!(UIScreen), mainScreen];
      let screen_rect: NSRect = msg_send![main_screen, bounds];

      let window_obj: ObjcId = msg_send![class!(UIWindow), alloc];
      let window_obj: ObjcId = msg_send![window_obj, initWithFrame: screen_rect];

      let view = create_opengl_view(screen_rect, 1, true);

      msg_send_![window_obj, addSubview: view.view];
      msg_send_![window_obj, setRootViewController: view.view_ctrl];
      msg_send_![window_obj, makeKeyAndVisible];
    }
    YES
  }

  unsafe {
    decl.add_method(
      sel!(application: didFinishLaunchingWithOptions:),
      did_finish_launching_with_options as extern "C" fn(&Object, Sel, ObjcId, ObjcId) -> BOOL,
    );
  }

  decl.register()
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
  pub fn NSStringFromClass(class: ObjcId) -> ObjcId;
}

#[link(name = "UIKit", kind = "framework")]
extern "C" {
  pub fn UIApplicationMain(
    argc: i32,
    argv: *mut *mut i8,
    principal_class_name: ObjcId,
    delegate_class_name: ObjcId,
  );
}
pub const NIL: ObjcId = 0 as ObjcId;

struct MyGame {
  gl: Rc<Context>,
  frame_counter: FrameCounter,
  white: Rc<Texture>,
  batch: PolygonBatch,
  camera: OrthoCamera,
  x: f32,
  width: f32,
  height: f32,
  fox: Rc<Texture>,
  sprites: Vec<Sprite>,
  accumulate: f32,
}
pub const UTF8_ENCODING: usize = 4;

pub fn str_to_nsstring(str: &str) -> ObjcId {
  unsafe {
    let ns_string: ObjcId = msg_send![class!(NSString), alloc];
    let ns_string: ObjcId = msg_send![
        ns_string,
        initWithBytes: str.as_ptr()
        length: str.len()
        encoding: UTF8_ENCODING as ObjcId
    ];
    let _: () = msg_send![ns_string, autorelease];
    ns_string
  }
}

pub fn nsstring_to_string(string: ObjcId) -> String {
  unsafe {
    let utf8_string: *const std::os::raw::c_uchar = msg_send![string, UTF8String];
    let utf8_len: usize = msg_send![string, lengthOfBytesUsingEncoding: UTF8_ENCODING];
    let slice = std::slice::from_raw_parts(utf8_string, utf8_len);
    std::str::from_utf8_unchecked(slice).to_owned()
  }
}

pub fn get_file_path(path: &str) -> String {
  let path = std::path::Path::new(&path);
  let path_without_extension = path.with_extension("");
  let path_without_extension = path_without_extension.to_str().unwrap();
  let extension = path.extension().unwrap_or_default().to_str().unwrap();

  unsafe {
    let main_bundle: ObjcId = msg_send![class!(NSBundle), mainBundle];
    let resource = str_to_nsstring(path_without_extension);
    let type_ = str_to_nsstring(extension);
    let file_path: ObjcId = msg_send![main_bundle, pathForResource:resource ofType:type_];
    return nsstring_to_string(file_path);
  }
}

impl MyGame {
  pub fn new(width: f32, height: f32) -> Self {
    let gl = unsafe {
      let gl = Context::from_loader_function(|s| get_proc_address(s.as_ptr()));
      gl
    };
    let gl = Rc::new(gl);
    let mut batch = PolygonBatch::create(&gl);
    batch.set_y_down(true);

    let mut camera = gdx::g2d::ortho_cam::OrthoCamera::new(width, height, width, height);
    camera.set_position(width / 2., height / 2.);
    camera.set_y_down(true);
    camera.update();

    let fox = {
      Reader::open(get_file_path("fox.png"))
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
    let white = Texture::new_white_texture(&gl);

    unsafe {
      gl.clear_color(0.5, 0.8, 0.2, 1.);
    }

    let mut sprites = Vec::<Sprite>::new();
    let mut rng = rand::thread_rng();

    for _i in 0..30000 {
      sprites.push(Sprite {
        x: rng.gen::<f32>() * width,
        y: rng.gen::<f32>() * height,
        speed_x: rng.gen::<f32>() * width - width / 2.,
        speed_y: rng.gen::<f32>() * height - height / 2.,
      });
    }

    Self {
      gl,
      white,
      batch,
      camera,
      frame_counter: FrameCounter::new(),
      x: 0.,
      width,
      height,
      fox,
      sprites,
      accumulate: 0.,
    }
  }

  #[allow(dead_code)]
  pub fn resize(&mut self, width: f32, height: f32) {
    self.camera.resize(width, height, width, height);
    self.camera.set_position(width / 2., height / 2.);
    self.camera.update();

    self.width = width;
    self.height = height;
  }

  unsafe fn update(&mut self) {
    let delta = self.frame_counter.update();

    self.x += delta * 200.;
    if self.x > self.width {
      self.x = -100.;
    }

    self.accumulate += delta;
    if self.accumulate >= 1. {
      println!(
        "FPS: {}, draw calls: {} for {} sprites",
        self.frame_counter.fps(),
        self.batch.get_draw_calls(),
        self.sprites.len()
      );
      self.accumulate = 0.;
    }

    self.gl.clear(COLOR_BUFFER_BIT);
    self.batch.set_projection(&self.camera.combined);
    self.batch.begin();
    // self.batch.draw(&self.white, self.x, 0., 100., 100.);
    // self.batch.draw(&self.fox, 50., 50., 100., 100.);
    for sprite in &mut self.sprites {
      sprite.x += sprite.speed_x * delta;
      sprite.y += sprite.speed_y * delta;
      if sprite.x >= self.width {
        sprite.x = self.width;
        sprite.speed_x = -sprite.speed_x;
      } else if sprite.x <= 0. {
        sprite.x = 0.;
        sprite.speed_x = -sprite.speed_x;
      }
      if sprite.y >= self.height {
        sprite.y = self.height;
        sprite.speed_y = -sprite.speed_y;
      } else if sprite.y <= 0. {
        sprite.y = 0.;
        sprite.speed_y = -sprite.speed_y;
      }
      self.batch.draw(
        &self.fox,
        sprite.x - 30. / 2.,
        sprite.y - 30. / 2.,
        30.,
        30.,
      );
    }
    self.batch.end();
  }
}

#[no_mangle]
pub extern "C" fn start_app() {
  unsafe {
    let argc = 1;
    let mut argv = b"Test Rust\0" as *const u8 as *mut i8;

    let class: ObjcId = msg_send!(define_app_delegate(), class);
    let class_string = NSStringFromClass(class as _);

    UIApplicationMain(argc, &mut argv, NIL, class_string);
  }
}
