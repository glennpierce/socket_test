//TODO
//spawn(bmos_http_server::serve);

extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate clap;
extern crate byteorder;
extern crate mio;
extern crate slab;
extern crate rustc_serialize;
extern crate toml;
extern crate postgres;
extern crate time;
extern crate chrono;
extern crate rusqlite;
#[macro_use]
extern crate log;

mod bmos_sensor;
mod bmos_config;
mod bmos_server;
mod bmos_http_server;
mod bmos_connection;
mod bmos_storage;
mod bmos_storage_sqlite;

use bmos_storage::BmosStorage;
use std::net::SocketAddr;
use std::thread;

use mio::*;
use mio::tcp::*;

use bmos_sensor::BmosTimeConverter;
use bmos_server::*;

use clap::{Arg, App};

use chrono::{NaiveDateTime, ParseResult, ParseError};

fn main() {

    // Before doing anything, let us register a logger. The mio library has really good logging
    // at the _trace_ and _debug_ levels. Having a logger setup is invaluable when trying to
    // figure out why something is not working correctly.
    pretty_env_logger::init().expect("Failed to init logger");


    // let date_str = "2013-02-14 15:41:07";
    // let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S");
    // match date {
    //     Ok(v) => println!("{:?}", v),
    //     //Err(NotEnough) => println!("{}", "ddd"),
    //     Err(NotEnough) => break,
    //     Err(e) => println!("{:?}", e)
    // }

    //let date_str = "2013-02-14 15:41:07";
    //let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").unwrap();
    let date = NaiveDateTime::from_bmos_time_string("1485547437.987534").unwrap();

    println!("{:?}", date);

    // match date {
    //     Ok(v) => println!("{:?}", v),
    //     Err(e) => println!("{:?}", e)
    // }



    // Create the storage for values. For development this is sqlite in memory
    // Eventually it will be postgres
    let storage = bmos_storage_sqlite::BmosSqliteStorage::new();
    storage.create_tables().unwrap();

    // Pull some optional arguments off the commandline
    // let matches = App::new("cormorant")
    //                       .version("0.1")
    //                       .author("Zachary Tong <zacharyjtong@gmail.com>")
    //                       .about("Toy Distributed Key:Value Store")
    //                       .arg(Arg::with_name("CONFIG")
    //                            .short("c")
    //                            .long("config")
    //                            .help("Path to the config.toml")
    //                            .takes_value(true))
    //                       .arg(Arg::with_name("THREADS")
    //                            .short("t")
    //                            .long("threads")
    //                            .help("Configures the number of threads")
    //                            .takes_value(true))
    //                       .get_matches();

    /*
// Default to 4 threads unless specified
    let threads: usize = matches.value_of("threads").unwrap_or("4").parse().unwrap();

    // Config is located in same directory as `config.toml` unless specified
    let path: String = matches.value_of("config").unwrap_or("config.toml").parse().unwrap();

    // We place the deserialized Config into an Arc, so that we can share it between
    // multiple threads in the future.  It will be immutable and not a problem to share
    let config = Arc::new(RwLock::new(Config::parse(path)));
    let state = Arc::new(RwLock::new(State::new()));

    {
        info!("Starting server \"{}\" [{}]",
            &config.read().unwrap().node.name, state.read().unwrap().node_id.to_hyphenated_string());
    }
*/


    std::thread::spawn(bmos_http_server::serve);

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
    let mut server = BmosTcpServer::new(sock, &storage);
    server.run(&mut poll).expect("Failed to run server");
}
