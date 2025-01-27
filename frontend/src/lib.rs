use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_utils::window;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

use sauron_core::{
    dom::{self, DomNode},
    prelude::Node,
    vdom,
    vdom::diff,
};
use sauron_html_parser::{parse_html, raw_html};

#[derive(Clone)]
struct DomUpdater {
    id: String,
    current_vdom: Node<()>,
    root_node: Rc<RefCell<Option<DomNode>>>,
    mount_node: Rc<RefCell<Option<DomNode>>>,
}

impl DomUpdater {
    fn new(id: String) -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let div: web_sys::Element = document
            .get_element_by_id(id.as_str())
            .expect("Element with specified id not found");

        let web_sys_node: web_sys::Node = web_sys::Node::from(div);
        let div_node = DomNode::from(web_sys_node);

        let current_vdom: Node<()> = parse_html::<()>("").unwrap().unwrap();
        let ev_callback = |_| {};
        let root: DomNode = dom::create_dom_node(&current_vdom, ev_callback);

        DomUpdater {
            id,
            current_vdom,
            root_node: Rc::new(RefCell::new(Some(root))),
            mount_node: Rc::new(RefCell::new(Some(div_node))),
        }
    }
    fn update(&mut self, next_html: String) {
        let new_node: Node<()> = parse_html::<()>(next_html.as_str()).unwrap().unwrap();

        let old_vdom = self.current_vdom.clone();

        //log::debug!("-------------------------------------------------");
        //log::debug!("old_node: {}", old_vdom.render_to_string());
        //log::debug!("inner_html: {}", self.inner_html());
        // fn same(a: String, b: String) -> String {
        //     if a == b {
        //         "same".to_string()
        //     } else {
        //         "different".to_string()
        //     }
        // }
        // log::debug!(
        //     "   => {}",
        //     same(old_vdom.render_to_string(), self.inner_html())
        // );
        //log::debug!("new_node: {}", new_node.render_to_string());
        // log::debug!("new_node: {:#?}", new_node);

        let vdom_patches = vdom::diff(&old_vdom, &new_node).unwrap();

        //log::debug!("Created {} VDOM patch(es)", vdom_patches.len());
        //log::debug!("Created {:#?}", vdom_patches);
        let dom_patches = dom::convert_patches(
            self.root_node.borrow().as_ref().unwrap(),
            &vdom_patches,
            |_| {},
        )
        .unwrap();
        //log::debug!("Converted {} DOM patch(es)", dom_patches.len());
        //log::debug!("Converted {:#?}", dom_patches);
        //log::debug!("-------------------------------------------------");
        dom::apply_dom_patches(
            Rc::clone(&self.root_node),
            Rc::clone(&self.mount_node),
            dom_patches,
        )
        .unwrap();
        self.current_vdom = new_node.clone();

        //assert_eq!(next_html, self.inner_html());
    }
    fn inner_html(&self) -> String {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let target: Element = document.get_element_by_id(self.id.as_str()).unwrap();
        target.inner_html()
    }
}

#[wasm_bindgen]
pub fn foo() {
    //console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    log::info!("Hello, world!");
}

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

    //-0--------- sync VDOM to DOM --------------------
    //-0------------------------------------
    let host = location.host().unwrap();
    let websocket_address = format!("{protocol}://{host}/ws");
    let ws = WebSocket::open(&websocket_address).expect("Failed to create WebSocket");

    let (_write, mut read) = ws.split();

    spawn_local({
        async move {
            let id: String = "ws-div".to_string();
            let mut dom_updater: DomUpdater = DomUpdater::new(id.clone());
            while let Some(Ok(msg)) = read.next().await {
                match msg {
                    Message::Text(message) => {
                        log::info!("Received WS message");
                        //log::debug!("Handle message: {message:?}");

                        dom_updater.update(format!("<div>{}</div>", message));
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
