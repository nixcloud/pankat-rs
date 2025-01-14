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

    let vdiv = html! { <div> <div> <div><p>hello world1</p></div> </div> </div> };
    let div: Element = vdiv.create_dom_node().node.unchecked_into::<Element>();
    div.set_id("nested-div");

    body.append_child(&div).map_err(|e| JsValue::from(e))?;
    boak();
    Ok(())
}

#[wasm_bindgen]
pub fn boak() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Create a closure to execute the delayed code
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        let custom_div =
            html! { <div><h1>Custom Title</h1><p>Custom content rendered in Rust!</p></div> };
        let element: web_sys::Element = custom_div
            .create_dom_node()
            .node
            .unchecked_into::<web_sys::Element>();
        element.set_id("custom-content");

        body.append_child(&element).unwrap();
    }) as Box<dyn FnMut()>);

    // Set a timeout to execute the closure after 2 seconds
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            2000,
        )
        .unwrap();

    // Forget the closure to prevent it from being dropped
    closure.forget();

    Ok(())
}
