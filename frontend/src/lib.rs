extern crate web_sys;
use std::cell::Cell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlParagraphElement};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Initial HTML structure with text content
    let initial_vdiv = html! { <div> <div> <div><p>hello world</p></div> </div> </div> };
    let div: Element = initial_vdiv
        .create_dom_node()
        .node
        .unchecked_into::<Element>();
    body.append_child(&div).map_err(|e| JsValue::from(e))?;

    // Closure to periodically update the text content of the paragraph element
    let closure = Closure::wrap(Box::new(move |&mut ()| {
        if let Some(p_element) = div
            .query_selector("p")
            .unwrap()
            .and_then(|e| e.dyn_into::<web_sys::HtmlParagraphElement>().ok())
        {
            p_element.set_text_content(Some("Updated hello world"));
            // Optionally, you can change the text content back and forth to see variations
        }
    }) as Box<dyn FnMut(_)>);

    // Set up a periodic update every 2 seconds (2000 milliseconds)
    window.set_interval_with_callback_and_timeout_and_arguments(
        closure.as_ref().unchecked_ref(),
        2000,
        0,
    )?;

    Ok(())
}
