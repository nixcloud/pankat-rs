extern crate web_sys;
use std::cell::Cell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let vdiv = html! { <div> <div> <div><p>hello world</p></div> </div> </div> };
    let div: Element = vdiv.create_dom_node().node.unchecked_into::<Element>();
    div.set_id("nested-div");

    body.append_child(&div).map_err(|e| JsValue::from(e))?;
    Ok(())
}
