extern crate web_sys;
use log::info;

use percy_dom::event::VirtualEvents;
//use percy_dom::patch;
use percy_dom::prelude::*;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;
use web_sys::{js_sys, MessageEvent, WebSocket};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Initialize logging
    console_log::init_with_level(log::Level::Info).expect("error initializing log");
    
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

                // --------------compiles, no effect-----------------------

                // let mut div: VirtualNode = html! {<div>here will be your message</div>};
                // div.as_velement_mut()
                //     .unwrap()
                //     .special_attributes
                //     .dangerous_inner_html = Some(input.to_string());

                // let mut events = VirtualEvents::new();
                // let mut old_div: VirtualNode = html! {<div>here will be your essage</div>}; 
                
                // let (root_node, enode) = old_div.create_dom_node(&mut events);
                // events.set_root(enode);

                // let old_vnode: VirtualNode = VirtualNode::from(old_div);
                // let patches = percy_dom::diff(&old_vnode, &div);
                // percy_dom::patch(root_node, &old_vnode, &mut events, &patches);

                // -------------- compiles and updates document -----------------------

                // // Initialize a div with id `ws-div` in the DOM
                // let mut initial_div: VirtualNode = html! {<div id="ws-div">{"bar"}</div>};

                // // Mount the initial VirtualNode to the actual DOM
                // let root_node = web_sys::window()
                //     .unwrap()
                //     .document()
                //     .unwrap()
                //     .get_element_by_id("ws-div")
                //     .expect("`app` div not found in the DOM");
                // let mut percy_dom_root_node = PercyDom::new_replace_mount(initial_div, root_node);

                // // Render a new VirtualNode with the updated HTML
                // let mut updated_div: VirtualNode = html! {<div id="ws-div"></div>};
                // updated_div
                //     .as_velement_mut()
                //     .unwrap()
                //     .special_attributes
                //     .dangerous_inner_html = Some(input);

                // // Patch the existing DOM with the new VirtualNode
                // percy_dom_root_node.update(updated_div);

                // --------------experiment-----------------------

                let document = web_sys::window().unwrap().document().unwrap();
                let real_div: Element = document
                    .get_element_by_id("ws-div")
                    .expect("No element with id `ws-div` found in the DOM");

                let real_div_html: String = real_div.inner_html();

                let mut real_div_virtual_node: VirtualNode = html! {<div id="ws-div"></div>};

                    real_div_virtual_node
                        .as_velement_mut()
                        .unwrap()
                        .special_attributes
                        .dangerous_inner_html = Some(real_div_html.clone());
                
                log::info!("real_div_html: {}", real_div_html);

                // let initial_node =
                //     VirtualNode::from(&format!("<div id='ws-div'>{}</div>", initial_html))
                //         .expect("Failed to create initial VirtualNode");

                // // The new content to update the #ws-div element
                // let input = String::from("<p>bar</p>");

                // // Create an updated VirtualNode
                // let mut updated_node: VirtualNode = html! { <div id="ws-div"></div> };
                // updated_node
                //     .as_velement_mut()
                //     .unwrap()
                //     .special_attributes
                //     .dangerous_inner_html = Some(input);

                // // Patch the DOM using the `patch` function
                // patch(&real_div, initial_node, updated_node).unwrap();
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

// use std::{cell::RefCell, rc::Rc};

// use futures::StreamExt;
// use gloo_net::websocket::{futures::WebSocket, Message};
// use gloo_utils::{body, window};
// use percy_dom::prelude::*;
// use wasm_bindgen_futures::spawn_local;

// #[wasm_bindgen(start)]
// pub fn main_js() -> Result<(), JsValue> {
//     //console_log::init_with_level(log::Level::Debug).expect("error initializing log");
//     log::info!("WASM hello world");

//     let location = window().location();
//     let protocol = if location.protocol().unwrap() == "https:" {
//         "wss"
//     } else {
//         "ws"
//     };

//     let host = location.host().unwrap();
//     let websocket_address = format!("{protocol}://{host}/ws");
//     let ws = WebSocket::open(&websocket_address).expect("Failed to create WebSocket");

//     let (_write, mut read) = ws.split();
//     let initial_node = html! { <div>{ "Hello world!" }</div> };
//     let vdom = Rc::new(RefCell::new(PercyDom::new_append_to_mount(
//         initial_node,
//         &body(),
//     )));

//     spawn_local({
//         let vdom = Rc::clone(&vdom);
//         async move {
//             while let Some(msg_result) = read.next().await {
//                 if let Ok(msg) = msg_result {
//                     match msg {
//                         Message::Text(message) => {
//                             log::debug!("Handle message: {message:?}");
//                             // let mut new_node = html! { <div></div> };
//                             // new_node
//                             //     .as_velement_mut()
//                             //     .unwrap()
//                             //     .special_attributes
//                             //     .dangerous_inner_html = Some(message);
//                             // log::debug!("{new_node:?}");
//                             // vdom.borrow_mut().update(new_node);
//                         }
//                         Message::Bytes(_) => {
//                             log::warn!("Binary messages are not supported yet");
//                         }
//                     }
//                 } else {
//                     log::error!("Error reading message: {msg_result:?}");
//                 }
//             }
//             log::info!("WebSocket Closed");
//             // TODO: reconnect
//         }
//     });
//     Ok(())
// }
