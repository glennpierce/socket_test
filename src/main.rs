//! This module does stuff.
//!
//! It does this really well

//TODO
//spawn(bmos_http_server::serve);

/// Bmos2
/// Goals
/// Make easier to install on servers
/// -- Auto creation of db backend
/// Make distributed
/// -- Different sensors get forwarded to different servers
/// -- HTTP interface speak JSON directly

// Quest consolidated query different machines ?
// bicest meters update ?
// server hang still


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

extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]

extern crate utils;

//extern crate libc;
extern crate systemd;
//extern crate serde_json;
use systemd::daemon;


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
use std::collections::HashMap;

use mio::*;
use mio::tcp::*;

use bmos_sensor::BmosTimeConverter;
use bmos_server::*;
use bmos_sensor::test1;

use clap::{Arg, App};

use chrono::{NaiveDateTime, ParseResult, ParseError};


//use std::ffi::*;
//use ffi::daemon as ffi;
//use libc::{c_int, size_t};

//use std::os::raw::*;

// pub fn notify(unset_environment: bool, state: collections::HashMap<&str, &str>) -> Result<bool> {
//     let c_state = ffi.state_to_c_string(state).as_ptr() as *const c_char;
//     let result = sd_try!(ffi::sd_notify(unset_environment as c_int, c_state));
//     Ok(result != 0)
// }

fn main() {

    // Before doing anything, let us register a logger. The mio library has really good logging
    // at the _trace_ and _debug_ levels. Having a logger setup is invaluable when trying to
    // figure out why something is not working correctly.
    pretty_env_logger::init().expect("Failed to init logger");

    // Pull some optional arguments off the commandline
    let matches = App::new("bmosserver")
                          .version("0.1")
                          .author("Glenn Pierce <glennpierce@gmail.com>")
                          .about("Bmos sensor data store")
                          .arg(Arg::with_name("slave")
                               .short("s")
                               .long("slave")
                               //.index(1)
                               .help("Is this server a slave ?")
                               .takes_value(false))
                               //.required(true))
                    
                        //   .arg(Arg::with_name("CONFIG")
                        //        .short("c")
                        //        .long("config")
                        //        .help("Path to the config.toml")
                        //        .takes_value(true))
                        //   .arg(Arg::with_name("THREADS")
                        //        .short("t")
                        //        .long("threads")
                        //        .help("Configures the number of threads")
                        //        .takes_value(true))
                          .get_matches();

    let is_master: bool = if matches.is_present("slave") { false } else { true };

     // Config is located in same directory as `config.toml` unless specified
    //let is_master: bool = matches.value_of("master").parse().unwrap();

    println!("is_master: {}", is_master);

    test1();










    //let date_str = "2013-02-14 15:41:07";
    //let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").unwrap();
    // let date = NaiveDateTime::from_bmos_time_string("1485547437.987534").unwrap();

    // println!("{:?}", date);

    // match date {
    //     Ok(v) => println!("{:?}", v),
    //     Err(e) => println!("{:?}", e)
    // }



    // Create the storage for values. For development this is sqlite in memory
    // Eventually it will be postgres
    let storage = bmos_storage_sqlite::BmosSqliteStorage::new();
    storage.create_tables().unwrap();



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
