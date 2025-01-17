use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_utils::{body, window};
use percy_dom::event::VirtualEvents;
use percy_dom::prelude::*;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

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
                        log::info!("Received WS message");
                        //log::debug!("Handle message: {message:?}");
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
