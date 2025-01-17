extern crate web_sys;
use log::info;

use percy_dom::event::VirtualEvents;
use percy_dom::patch;
use percy_dom::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;
use web_sys::{js_sys, MessageEvent, WebSocket};

#[wasm_bindgen]
pub fn foo() {
    static INIT: std::sync::Once = std::sync::Once::new();
    static mut PERCY_DOM_ROOT_NODE: Option<PercyDom> = None;

    INIT.call_once(|| {
        info!("WASM hello world foo");

        // Initialize a div with id `ws-div` in the DOM
        let initial_div: VirtualNode = html! {<div id="ws-div"></div>};

        // Mount the initial VirtualNode to the actual DOM
        let root_node: Element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("ws-div")
            .expect("`app` div not found in the DOM");
        let percy_dom_root_node = PercyDom::new_replace_mount(initial_div, root_node);

        unsafe {
            PERCY_DOM_ROOT_NODE = Some(percy_dom_root_node);
        }
    });

    unsafe {
        if let Some(percy_dom_root_node) = &mut PERCY_DOM_ROOT_NODE {
            info!("updating");

            let input2 = "<div>Hello World</div><div>this is fine, yeah, srly!!!</div>".to_string();
            let mut updated_div2: VirtualNode = html! {<div id="ws-div"></div>};
            updated_div2
                .as_velement_mut()
                .unwrap()
                .special_attributes
                .dangerous_inner_html = Some(input2);
            percy_dom_root_node.update(updated_div2);
        }
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Initialize logging
    console_log::init_with_level(log::Level::Info).expect("error initializing log");

    let ws = Rc::new(Cell::new(None));
    let ws_clone = ws.clone();

    let connect = Rc::new(move || {
        let location = web_sys::window().unwrap().location();
        let protocol = if location.protocol().unwrap_or_default() == "https:" {
            "wss:"
        } else {
            "ws:"
        };

        let host = location.host().unwrap_or_default();
        let websocket_address = format!("{}/ws", host);
        let ws = WebSocket::new(&format!("{}//{}", protocol, websocket_address))
            .expect("Failed to create WebSocket");

        // Handle WebSocket open
        let on_open = Closure::wrap(Box::new(move || {
            info!("WebSocket opened");
        }) as Box<dyn FnMut()>);

        let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(message) = e.data().dyn_into::<js_sys::JsString>() {
                if let Some(message_str) = message.as_string() {
                    log::info!("Handle message");
                    let initial_node = html! { <div id="ws-id">{ "Hello world!" }</div> };
                    let root_node: Element = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id("ws-div")
                        .expect("`app` div not found in the DOM");
                    let mut vdom = PercyDom::new_replace_mount(initial_node, root_node);
                    let mut new_node = html! { <div id="ws-id"></div> };
                    new_node
                        .as_velement_mut()
                        .unwrap()
                        .special_attributes
                        .dangerous_inner_html = Some(message_str);
                    //log::debug!("{new_node:?}");
                    vdom.update(new_node);
                } else {
                    log::error!("Failed to convert JsString to String");
                }
            } else {
                log::error!("Message data is not a JsString");
            }
        }) as Box<dyn FnMut(_)>);

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
