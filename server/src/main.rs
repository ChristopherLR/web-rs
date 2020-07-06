#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate notify;
extern crate ws;
mod web;
use log::{info, warn};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc;
use std::thread;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
// Server WebSocket handler
struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let init_size = CONNS.lock().unwrap().len();
        CONNS.lock().unwrap().push(self.out.clone());
        if CONNS.lock().unwrap().len() > init_size {
            Ok(())
        } else {
            Err(ws::Error {
                kind: ws::ErrorKind::Capacity,
                details: std::borrow::Cow::Owned(String::from("Did not increase connection pool")),
            })
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        info!("Server got message '{}'. ", msg);
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        info!("WebSocket closing for ({:?}) {}", code, reason);
        self.out.shutdown().unwrap();
    }
}

fn websocket_listener() {
    thread::spawn(move || loop {
        listen("127.0.0.1:3012", |out| Server { out }).unwrap()
    });
}

fn watch() {
    // Create a channel to receive the events.
    let (tx, rx) = mpsc::channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |res| tx.send(res).unwrap()).unwrap();
    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("../dist", RecursiveMode::Recursive).unwrap();

    match rx.recv() {
        Ok(event) => {
            for con in CONNS.lock().unwrap().iter() {
                con.send("Hello");
                info!("Change in dir");
            }
        }
        Err(e) => warn!("watch error: {:?}", e),
    }
}

lazy_static! {
    static ref CONNS: std::sync::Mutex<Vec<ws::Sender>> = std::sync::Mutex::new(vec![]);
}

fn main() {
    env_logger::init();
    // Automatically select the best implementation for your platform.
    thread::spawn(|| loop {
        watch()
    });

    websocket_listener();

    rocket::ignite()
        .mount("/", routes![web::index, web::files])
        .launch();
}
