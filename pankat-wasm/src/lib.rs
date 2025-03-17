use futures::StreamExt;
use gloo_utils::window;
use log::Level;
use sauron_core::{
    dom::{self, DomNode},
    prelude::Node,
    vdom,
    vdom::diff,
};
use sauron_html_parser::{parse_html, raw_html};
use std::{cell::RefCell, rc::Rc};
use tokio::sync::{broadcast, mpsc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{js_sys, Element, ErrorEvent, HtmlElement, MessageEvent, WebSocket};

#[wasm_bindgen]
extern "C" {
    fn myExportedFunction(message: &str);
}

#[derive(Clone)]
struct DomUpdater {
    id: String,
}

impl DomUpdater {
    fn new(id: String) -> Self {
        DomUpdater { id }
    }

    fn update(&mut self, next_html: String) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let target: Element = document.get_element_by_id(self.id.as_str()).unwrap();
        myExportedFunction(&next_html);
    }
}

fn ws_close() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if let Some(status_element) = document.get_element_by_id("websocketStatus") {
        if let Some(status_element) = status_element.dyn_ref::<HtmlElement>() {
            status_element
                .class_list()
                .add_1("glyphicon-remove")
                .expect("Failed to add class");
            status_element
                .class_list()
                .remove_1("glyphicon-ok")
                .expect("Failed to remove class");
        }
    }
}

fn ws_open() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if let Some(ws_element) = document.get_element_by_id("websocket") {
        if let Some(ws_element) = ws_element.dyn_ref::<HtmlElement>() {
            ws_element
                .style()
                .set_property("display", "block")
                .expect("Failed to set display property");
        }
    }

    if let Some(status_element) = document.get_element_by_id("websocketStatus") {
        if let Some(status_element) = status_element.dyn_ref::<HtmlElement>() {
            status_element
                .class_list()
                .remove_1("glyphicon-remove")
                .expect("Failed to remove class");
            status_element
                .class_list()
                .add_1("glyphicon-ok")
                .expect("Failed to add class");
        }
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).expect("error initializing logger");
    log::info!("Now executing WASM code from lib.rs in pankat_wasm");

    let location = window().location();
    let protocol = if location.protocol().unwrap() == "https:" {
        "wss"
    } else {
        "ws"
    };

    let host = location.host().unwrap();
    let websocket_address = format!("{protocol}://{host}/api/ws");
    let id: String = "NavAndContent".to_string();
    let dom_updater: DomUpdater = DomUpdater::new(id.clone()); // Removed mut
    spawn_local({
        async move {
            loop {
                let mut dom_updater = dom_updater.clone();
                let ws = WebSocket::new(&websocket_address).unwrap();
                let cloned_ws = ws.clone(); // Fixed unused variable warning
                let (tx, mut rx) = futures::channel::mpsc::unbounded();

                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        let txt_string: String = String::from(txt);
                        //log::info!("message event, received Text: {}", txt_string);

                        if txt_string == "ping" {
                            match cloned_ws.send_with_str("pong") {
                                Ok(_) => log::info!("message successfully sent"),
                                Err(err) => log::info!("error sending message: {:?}", err),
                            }
                            return;
                        } else {
                            dom_updater
                                .update(format!(r#"<div id="NavAndContent">{}</div>"#, txt_string));
                        }
                    } else {
                        log::info!("message event, received Unknown: {:?}", e.data());
                    }
                });
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();

                let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
                    log::info!("error event: {:?}", e);
                    return;
                });
                ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
                onerror_callback.forget();

                let cloned_ws = ws.clone();
                let onopen_callback = Closure::<dyn FnMut()>::new(move || {
                    log::info!("socket opened");
                    ws_open();
                });
                ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                onopen_callback.forget();

                let onclose_callback = Closure::<dyn FnMut()>::new(move || {
                    log::info!("socket closed");
                    ws_close();
                    let _ = tx.unbounded_send(());
                });
                let closed_ws = ws.clone();
                closed_ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
                onclose_callback.forget();

                // Wait until the websocket is closed
                rx.next().await;
                gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    });
    Ok(())
}
