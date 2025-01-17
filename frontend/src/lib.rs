use std::{cell::RefCell, rc::Rc};

use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_utils::{body, window};
use percy_dom::prelude::*;
use wasm_bindgen_futures::spawn_local;

// extern crate web_sys;
// use log::info;

use percy_dom::event::VirtualEvents;
// use percy_dom::patch;
// use percy_dom::prelude::*;
// use std::cell::Cell;
// use std::cell::RefCell;
// use std::rc::Rc;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen::JsValue;
use web_sys::Element;
// use web_sys::{js_sys, MessageEvent, WebSocket};

// #[wasm_bindgen]
// pub fn foo() {
//     static INIT: std::sync::Once = std::sync::Once::new();
//     static mut PERCY_DOM_ROOT_NODE: Option<PercyDom> = None;

//     INIT.call_once(|| {
//         info!("WASM hello world foo");

//         // Initialize a div with id `ws-div` in the DOM
//         let initial_div: VirtualNode = html! {<div id="ws-div"></div>};

//         // Mount the initial VirtualNode to the actual DOM
//         let root_node: Element = web_sys::window()
//             .unwrap()
//             .document()
//             .unwrap()
//             .get_element_by_id("ws-div")
//             .expect("`app` div not found in the DOM");
//         let percy_dom_root_node = PercyDom::new_replace_mount(initial_div, root_node);

//         unsafe {
//             PERCY_DOM_ROOT_NODE = Some(percy_dom_root_node);
//         }
//     });

//     unsafe {
//         if let Some(percy_dom_root_node) = &mut PERCY_DOM_ROOT_NODE {
//             info!("updating");

//             let input2 = "<div>Hello World</div><div>this is fine, yeah, srly!!!</div>".to_string();
//             let mut updated_div2: VirtualNode = html! {<div id="ws-div"></div>};
//             updated_div2
//                 .as_velement_mut()
//                 .unwrap()
//                 .special_attributes
//                 .dangerous_inner_html = Some(input2);
//             percy_dom_root_node.update(updated_div2);
//         }
//     }
// }

// #[wasm_bindgen(start)]
// pub fn main_js() -> Result<(), JsValue> {
//     // Initialize logging
//     console_log::init_with_level(log::Level::Info).expect("error initializing log");

//     let ws = Rc::new(Cell::new(None));
//     let ws_clone = ws.clone();

//     let connect = Rc::new(move || {
//         let location = web_sys::window().unwrap().location();
//         let protocol = if location.protocol().unwrap_or_default() == "https:" {
//             "wss:"
//         } else {
//             "ws:"
//         };

//         let host = location.host().unwrap_or_default();
//         let websocket_address = format!("{}/ws", host);
//         let ws = WebSocket::new(&format!("{}//{}", protocol, websocket_address))
//             .expect("Failed to create WebSocket");

//         // Handle WebSocket open
//         let on_open = Closure::wrap(Box::new(move || {
//             info!("WebSocket opened");
//         }) as Box<dyn FnMut()>);

//         let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
//             if let Ok(message) = e.data().dyn_into::<js_sys::JsString>() {
//                 if let Some(message_str) = message.as_string() {
//                     let message_str = "<div>hi</div>".to_string();
//                     log::info!("Handle message");
//                     let initial_node = html! { <div id="ws-id">{ "Hello world!" }</div> };
//                     log::info!("Handle message1");
//                     let root_node: Element = web_sys::window()
//                         .unwrap()
//                         .document()
//                         .unwrap()
//                         .get_element_by_id("ws-div")
//                         .expect("`app` div not found in the DOM");
//                     log::info!("Handle message2");
//                     let mut vdom = PercyDom::new_replace_mount(initial_node, root_node);
//                     log::info!("Handle message3");
//                     let mut new_node = html! { <div id="ws-id"></div> };
//                     log::info!("Handle message4");
//                     log::info!("message: {message_str}");
//                     if let Some(velement) = new_node.as_velement_mut() {
//                         velement.special_attributes.dangerous_inner_html =
//                             Some(message_str.clone());
//                     } else {
//                         log::error!("Failed to unwrap new_node as VElement");
//                         return;
//                     }
//                     log::debug!("before update");
//                     vdom.update(new_node);
//                 } else {
//                     log::error!("Failed to convert JsString to String");
//                 }
//             } else {
//                 log::error!("Message data is not a JsString");
//             }
//         }) as Box<dyn FnMut(_)>);

//         // let connect_clone = connect.clone();
//         // let on_close = Closure::wrap(Box::new(move |_| {
//         //     info!("WebSocket closed, reconnecting...");
//         //     ws_clone.set(Some((connect_clone)()));
//         // }) as Box<dyn FnMut(_)>);

//         // Set event listeners
//         ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
//         ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
//         //ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));

//         // Forget closures to keep them alive
//         on_open.forget();
//         on_message.forget();
//         //on_close.forget();

//         ws
//     });

//     ws.set(Some(connect()));

//     Ok(())
// }

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    log::info!("WASM hello world");

    let location = window().location();
    let protocol = if location.protocol().unwrap() == "https:" {
        "wss"
    } else {
        "ws"
    };

    let host = location.host().unwrap();
    let websocket_address = format!("{protocol}://{host}/ws");
    let ws = WebSocket::open(&websocket_address).expect("Failed to create WebSocket");

    let (_write, mut read) = ws.split();

    spawn_local({
        async move {
            while let Some(Ok(msg)) = read.next().await {
                match msg {
                    Message::Text(message) => {
                        log::debug!("Handle message: {message:?}");
                        let document = web_sys::window().unwrap().document().unwrap();
                        let real_div: Element = document
                            .get_element_by_id("ws-div")
                            .expect("No element with id `ws-div` found in the DOM");

                        let real_div_html: String = real_div.inner_html();
                        let mut real_div_virtual_node: VirtualNode =
                            html! {<div id="ws-div"></div>};
                        real_div_virtual_node
                            .as_velement_mut()
                            .unwrap()
                            .special_attributes
                            .dangerous_inner_html = Some(real_div_html.clone());
                        //log::info!("real_div_html: {}", real_div_html);

                        let mut new_div_virtual_node: VirtualNode = html! {<div id="ws-div"></div>};
                        new_div_virtual_node
                            .as_velement_mut()
                            .unwrap()
                            .special_attributes
                            .dangerous_inner_html = Some(message);

                        let mut events = VirtualEvents::new();
                        let (root_node, enode) = real_div_virtual_node.create_dom_node(&mut events);
                        events.set_root(enode);

                        let patches =
                            percy_dom::diff(&real_div_virtual_node, &new_div_virtual_node);
                        percy_dom::patch(root_node, &real_div_virtual_node, &mut events, &patches);

                        let mut percy_dom_root_node =
                            PercyDom::new_replace_mount(real_div_virtual_node, real_div);
                        percy_dom_root_node.update(new_div_virtual_node);
                    }
                    Message::Bytes(_) => {
                        log::warn!("Binary messages are not supported yet");
                    }
                }
            }
            log::info!("WebSocket Closed")
        }
    });

    Ok(())
}
