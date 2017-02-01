extern crate pretty_env_logger;
extern crate byteorder;
extern crate mio;
extern crate slab;
extern crate rustc_serialize;
extern crate toml;
extern crate time;
extern crate chrono;

#[macro_use]
extern crate log;

extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]

mod sensor;
mod server;
mod connection;

use std::net::SocketAddr;
use std::thread;
use std::collections::HashMap;

use mio::*;
use mio::tcp::*;

use server::*;

use chrono::{NaiveDateTime, ParseResult, ParseError};



fn main() {

    // Before doing anything, let us register a logger. The mio library has really good logging
    // at the _trace_ and _debug_ levels. Having a logger setup is invaluable when trying to
    // figure out why something is not working correctly.
    pretty_env_logger::init().expect("Failed to init logger");

    let addr = "127.0.0.1:8000"
        .parse::<SocketAddr>()
        .expect("Failed to parse host:port string");
    let sock = TcpListener::bind(&addr).expect("Failed to bind address");

    // Create a polling object that will be used by the server to receive events
    let mut poll = Poll::new().expect("Failed to create Poll");

    // Create our Server object and start polling for events. I am hiding away
    // the details of how registering works inside of the `Server` object. One reason I
    // really like this is to get around having to have `const SERVER = Token(0)` at the top of my
    // file. It also keeps our polling options inside `Server`.
    let mut server = TcpServer::new(sock);
    server.run(&mut poll).expect("Failed to run server");
}
