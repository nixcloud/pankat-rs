extern crate web_sys;
use log::info;
use std::cell::Cell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{js_sys, Element, MessageEvent, WebSocket};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Initialize logging
    //console_log::init_with_level(log::Level::Info).expect("error initializing log");
    info!("WASM hello world");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let ws = Rc::new(Cell::new(None));
    let ws_clone = ws.clone();

    let connect = Rc::new(move || {
        let location = web_sys::window().unwrap().location();
        let protocol = if location.protocol().unwrap() == "https:" {
            "wss:"
        } else {
            "ws:"
        };

        let host = location.host().unwrap();
        //let host = "6dc6bb52-21ae-4297-9f17-1d299d118e3a-00-37bkydd25elf9.kirk.replit.dev";
        let websocket_address = format!("{}/ws", host);
        let ws = WebSocket::new(&format!("{}//{}", protocol, websocket_address))
            .expect("Failed to create WebSocket");

        // Handle WebSocket open
        let on_open = Closure::wrap(Box::new(move || {
            info!("WebSocket opened");
        }) as Box<dyn FnMut()>);

        // Handle WebSocket message
        let body_clone = body.clone();
        let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(data) = e.data().dyn_into::<js_sys::JsString>() {
                let input: String = data.as_string().unwrap_or_default();
                //let input: String = "<p>asdf</p>".to_string();
                // https://chinedufn.github.io/percy/html-macro/setting-inner-html/index.html
                // let vdiv = html! {

                //     <div> {input.clone()} </div>
                // };
                // let vdiv = html! {
                //     <button
                //       onclick=move|_event: web_sys::MouseEvent| {
                //         web_sys::console::log_1(&"clicked!".into());
                //       }
                //     >
                //       Click me!
                //     </button>
                // };
                // let div: Element = vdiv.create_dom_node().node.unchecked_into::<Element>();
                let mut div: VirtualNode = html! {
                <div></div>
                };
                div.as_velement_mut()
                    .unwrap()
                    .special_attributes
                    .dangerous_inner_html = Some(input);

                let div: Element = div.create_dom_node().node.unchecked_into();

                div.set_id("nested-div");

                body_clone
                    .append_child(&div)
                    .expect("Failed to append child");
            }
        }) as Box<dyn FnMut(_)>);

        // Handle WebSocket close
        // let connect_clone = connect.clone();
        // let on_close = Closure::wrap(Box::new(move |_| {
        //     info!("WebSocket closed, reconnecting...");
        //     ws_clone.set(Some((connect_clone)()));
        // }) as Box<dyn FnMut(_)>);

        // Set event listeners
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        //ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));

        // Forget closures to keep them alive
        on_open.forget();
        on_message.forget();
        //on_close.forget();

        ws
    });

    ws.set(Some(connect()));

    Ok(())
}
