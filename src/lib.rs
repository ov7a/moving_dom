use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Window};

mod dom_utils;
mod wasm_utils;
use crate::dom_utils::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Context {
    pub element: HtmlElement,
    pub adx: i32,
    pub ady: i32,
    pub dx: i32,
    pub dy: i32
}

pub fn next_step(window: &Window, context: &mut Context) {
    let max_dimensions = get_max_dimensions(window);
    let element_rect = get_element_rect(&context.element);

    if element_rect.left - context.adx < 0 {
        context.dx = context.adx;
    }
    if element_rect.right + context.adx >= max_dimensions.width {
        context.dx = -context.adx;
    }
    if element_rect.top - context.ady < 0 {
        context.dy = context.ady;
    }
    if element_rect.bottom + context.ady >= max_dimensions.height {
        context.dy = -context.ady;
    }
    move_element(&context.element, element_rect.left + context.dx, element_rect.top + context.dy);
}

fn start_loop(element: HtmlElement, stop_flag: Rc<Cell<bool>>){
    prepare_element_to_move(&element);

    let context = Context {
        element: element,
        adx: 1,
        ady: 1,
        dx: 1,
        dy: 1
    };

    request_animation_loop(context, stop_flag, next_step);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    wasm_utils::set_panic_hook();

    let text_element = get_element(&window(), "p");

    let stop = Rc::new(Cell::new(false));

    let stop_copy = stop.clone();
    let stopper = closure!({
        stop_copy.set(!stop_copy.get());
    });
    text_element.set_onclick(Some(stopper.as_ref().unchecked_ref()));

    start_loop(text_element, stop);

    stopper.forget(); //"safety": persists closure

    Ok(())
}

