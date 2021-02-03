use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

use std::cmp::max;
use std::cell::{RefCell, Cell};
use std::rc::Rc;

#[macro_export]
macro_rules! closure {
    ($closure:block) => { Closure::wrap(Box::new(move || $closure) as Box<dyn FnMut()> ); }
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(window: &Window, f: &Closure<dyn FnMut()>) {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame`");
}

pub fn window_document(window: &Window) -> Document {
    window
        .document()
        .unwrap()
}

pub fn document_element(window: &Window) -> Element {
    window_document(window)
        .document_element()
        .unwrap()
}

pub struct Dimensions {
    pub width: i32,
    pub height: i32
}

trait InnerDimension {
    fn parse(&self) -> i32;
}

impl InnerDimension for Result<JsValue, JsValue> {
    fn parse(&self) -> i32 {
        self.as_ref()
            .ok()
             .and_then(|h| h.as_f64())
             .map(|x| x.floor() as i32)
            .unwrap_or(0)
    }
}

pub fn get_max_dimensions(window: &Window) -> Dimensions {
    let document = document_element(window);
    Dimensions {
        width: max(document.client_width(), window.inner_width().parse()),
        height: max(document.client_height(), window.inner_height().parse())
    }
}

pub fn get_element(window: &Window, selector: &str) -> HtmlElement {
    let document = window_document(window);
    document.query_selector(selector)
                .unwrap()
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
}

pub fn prepare_element_to_move(element: &HtmlElement){
    element.style().set_property("position", "fixed").unwrap();
    element.style().set_property("margin", "0").unwrap();
}

pub fn move_element(element: &HtmlElement, left: i32, top: i32){
    element.style().set_property("top", &(top.to_string() + "px")).unwrap();
    element.style().set_property("left", &(left.to_string() + "px")).unwrap();
}

pub struct ElementRect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32
}

pub fn get_element_rect(element: &HtmlElement) -> ElementRect {
    let left = element.offset_left() as i32;
    let top = element.offset_top() as i32;
    ElementRect {
        left: left,
        right: left + element.offset_width() as i32,
        top: top,
        bottom: top + element.offset_height() as i32
    }
}

pub fn request_animation_loop<TContext: 'static>(mut context: TContext, stop_flag: Rc<Cell<bool>>, process_frame: fn(&Window, &mut TContext) -> ()) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let window_instance = window();
    *g.borrow_mut() = Some(closure!({
        if stop_flag.get() {
            let _ = f.borrow_mut().take();
            return;
        }

        process_frame(&window_instance, &mut context);

        request_animation_frame(&window_instance, f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(&window(), g.borrow().as_ref().unwrap());
}

