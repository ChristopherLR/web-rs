#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate log;
extern crate ws;
mod web;
use log::{info, warn};
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

fn main() {
    let server = thread::spawn(move || loop {
        listen("127.0.0.1:3012", |out| Server { out }).unwrap()
    });
    rocket::ignite()
        .mount("/", routes![web::index, web::files])
        .launch();
}
