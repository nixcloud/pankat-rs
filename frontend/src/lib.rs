use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_utils::window;
// use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

use log::trace;
use sauron::div;
// use sauron::html::lookup::match_tag;
use sauron::vdom::Node;
use sauron::Application;
use sauron::Cmd;
use sauron::Program;
use sauron_html_parser::parse_html;
use std::mem::ManuallyDrop;

/// This is a simple component for the puprpose of testing
#[derive(Copy, Clone, Debug)]
pub struct SimpleComponent;

impl Application for SimpleComponent {
    type MSG = ();

    fn update(&mut self, _msg: ()) -> Cmd<Self::MSG> {
        trace!("updating in SimpleComponent");
        Cmd::none()
    }

    fn view(&self) -> Node<()> {
        div(vec![], vec![])
    }
}

pub fn simple_program(mount_node: &web_sys::Node) -> ManuallyDrop<Program<SimpleComponent>> {
    console_log::init_with_level(log::Level::Trace).ok();
    Program::append_to_mount(SimpleComponent, mount_node)
}

struct SimpleProgram {
    simple_program: ManuallyDrop<Program<SimpleComponent>>,
}

impl SimpleProgram {
    fn new(root_node: &Element) -> Self {
        SimpleProgram {
            simple_program: simple_program(root_node),
        }
    }

    fn get(&mut self) -> &mut ManuallyDrop<Program<SimpleComponent>> {
        &mut self.simple_program
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
    //-0--------- String to Sauron Node ---------------------------
    let html = r#"<article class="side-to-side">
        <div>
            This is div content
        </div>
        <footer>
            This is footer
        </footer>
    </article>"#;
    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    let html1 = r#"<article class="side-to-side">
        <div>
            This is div content
        </div>
        <footer>
            This is footer1
        </footer>
    </article>"#;
    let node1: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    log::info!("node: {:#?}", node);
    log::info!("render: {}", node.render_to_string());
    //-0--------- Modify DOM --------------------------
    let root_node: Element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("ws-div")
        .expect("`app` div not found in the DOM");

    let mut simple_program = SimpleProgram::new(&root_node);

    // let mut simple_program = simple_program(&root_node);
    simple_program
        .get()
        .update_dom_with_vdom(node)
        .expect("must update dom");

    let button = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("button")
        .expect("No element with id `button` found in the DOM");
        
    let closure = Closure::wrap(Box::new(move || {
        log::info!("Button was clicked!");
        simple_program
            .get()
            .update_dom_with_vdom(node1)
            .expect("must update dom");
        log::info!("did the update happen?");
    }) as Box<dyn FnMut()>);

    button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Failed to add click event listener");
        
    closure.forget();
    
    // To keep the closure alive and not dropped prematurely, it must be stored.
    //closure.forget();
    
    

    //-0--------- VDOM diff ---------------------------
    //-0--------- sync VDOM to DOM --------------------
    //-0------------------------------------
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
