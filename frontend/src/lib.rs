extern crate web_sys;
use log::info;
use percy_dom::event::VirtualEvents;
use percy_dom::prelude::*;
use std::cell::Cell;
use std::rc::Rc;
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
                let mut div: VirtualNode = html! {<div>here will be your message</div>
                };
                div.as_velement_mut()
                    .unwrap()
                    .special_attributes
                    .dangerous_inner_html = Some(input.to_string());

                let mut events = VirtualEvents::new();
                let new_div: Element = div.create_dom_node(&mut events).0.unchecked_into();
                new_div.set_attribute("id", "ws-div").expect("Failed to set id attribute");
                let old_div = body_clone.query_selector("#ws-div").unwrap();
                if let Some(old_div) = old_div {
                    body_clone.replace_child(&new_div, &old_div).unwrap();
                } else {
                    body_clone.append_child(&new_div).unwrap();
                }
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
