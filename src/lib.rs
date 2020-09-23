use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::{ Cell, RefCell};
use std::rc::Rc;
use std::f64::consts::PI;

#[wasm_bindgen()]
pub struct Color{
    R: i64,
    G: i64,
    B: i64,
    rs: i64,
    gs: i64,
    bs: i64,
}

#[wasm_bindgen()]
pub struct Circle{
  x: f64,
  y: f64,
  size: f64,
  color: JsValue,
  opc: f64,
}

#[wasm_bindgen()]
extern "C" {
  #[wasm_bindgen(js_namespace=console)]
  fn log(msg: &str);
}

impl Circle{
  fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d){
    ctx.begin_path();
    ctx.set_stroke_style(&JsValue::from(String::from("rgb(27, 183, 27)")));
    ctx.set_line_width(4f64);
    ctx.arc(self.x, self.y, self.size, 0f64, 2f64 *PI);
    ctx.stroke();
  }

  fn update(&mut self, R: f64, G: f64, B: f64, opc: f64) {
    self.size = self.size + 1f64;
    self.opc =  1f64;//(60f64 - self.size) / 60f64;
    self.color = JsValue::from(format!("rgba({},{},{},{})", R, G, B, opc));
  }
}

#[wasm_bindgen(start)]
pub fn start()->Result<(), JsValue>{
  let window = web_sys::window().expect("can not get a window");
  let document = window.document().expect("can not get a document");
  let dom = document.get_element_by_id("myCanvas").expect("dom not found").dyn_into::<web_sys::HtmlCanvasElement>()?;

  let ctx = dom.get_context("2d")?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;

  let inner_width = window.inner_width().expect("error");
  let inner_height = window.inner_height().expect("error");
  let list:Vec<Circle> = Vec::new();
  let mut lists = Rc::new(RefCell::new(list));

  let ctx2 = Rc::new(ctx); 
  mouse_move(Rc::clone(&lists), &dom);
  // loop{
  loops(Rc::clone(&ctx2), Rc::clone(&lists), 1000f64, 2080f64, &window);
  // }

  Ok(())
}

pub fn loops(ctx: Rc<web_sys::CanvasRenderingContext2d>, mut list: Rc<RefCell<Vec<Circle>>>, inner_height: f64, inner_width: f64, window: &web_sys::Window) {
  
  let f_cell = Rc::new(RefCell::new(None));

  let g_cell = f_cell.clone();
  *g_cell.borrow_mut() = Some(Closure::wrap(Box::new(move ||{
    ctx.clear_rect(0f64, 0f64, inner_width, inner_height);
    ctx.fill_rect(0f64, 0f64, inner_width, inner_height);
    let mut index = 0;
    let mut lists = list.borrow_mut();
    for elem in lists.iter_mut() {
      if (elem.size >= 60f64) {
        lists.remove(index);
        break;
      }
    }
    // let mut lists = list.borrow_mut();
    for  elem in lists.iter_mut() {
      
      elem.draw(&ctx);
      elem.update(27f64, 183f64, 27f64, 1f64);
      index= index + 1;
    }
   request_animation_frame(f_cell.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));
  request_animation_frame(g_cell.borrow().as_ref().unwrap());
}

fn mouse_move(list: Rc<RefCell<Vec<Circle>>>, canvas: &web_sys::HtmlCanvasElement){

  let fns = move  |event: web_sys::MouseEvent|{
    let mut lists = list.borrow_mut();
    let circle = Circle{
      x: event.offset_x() as f64,
      y: event.offset_y() as f64,
      size: 20f64,
      color: JsValue::from(String::from("rgba(27,183,27, 1)")) ,
      opc: 1f64,
    };
    lists.push(circle);
  };
  let closure  = Closure::wrap(Box::new(fns) as Box<dyn FnMut(_)>);
  canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
  closure.forget();
}

fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}


fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  window()
      .request_animation_frame(f.as_ref().unchecked_ref())
      .expect("should register `requestAnimationFrame` OK");
}