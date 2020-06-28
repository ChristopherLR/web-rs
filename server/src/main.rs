#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate log;
extern crate notify;
extern crate ws;
mod web;
use log::{info, warn};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::thread;
use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender};
// Server WebSocket handler
struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        info!("Server got message '{}'. ", msg);
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        info!("WebSocket closing for ({:?}) {}", code, reason);
        self.out.shutdown().unwrap();
    }
}

fn watch() {
    // Create a channel to receive the events.
    let (tx, rx) = std::sync::mpsc::channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |res| tx.send(res).unwrap()).unwrap();
    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("../dist", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => info!("{:?}", event),
            Err(e) => warn!("watch error: {:?}", e),
        }
    }
}

fn main() {
    env_logger::init();
    let server = thread::spawn(move || loop {
        listen("127.0.0.1:3012", |out| Server { out }).unwrap()
    });

    // Automatically select the best implementation for your platform.
    thread::spawn(move || watch());

    rocket::ignite()
        .mount("/", routes![web::index, web::files])
        .launch();
}
