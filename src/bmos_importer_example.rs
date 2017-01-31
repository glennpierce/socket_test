/// Example importer into bmos
/// Importers always import into their own 'local' server bmos hence 127.0.0.1


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

//bind_port

extern crate rustc_serialize;
extern crate toml;
extern crate byteorder;
extern crate clap;
mod bmos_config;

use bmos_config::Config;

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::sync::{Arc, RwLock};

use byteorder::{ByteOrder, BigEndian};

//static NTHREADS: i32 = 10;

use clap::{Arg, App};




fn main() {

    let matches = App::new("bmos_importer_example")
                          .version("0.1")
                          .author("Glenn Pierce <glennpierce@gmail.com>")
                          .about("Bmos example importer")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .help("Path to the config.toml")
                               .takes_value(true))
                          .get_matches();

    let path: String = matches.value_of("config").unwrap_or("bmos.toml").parse().unwrap();

    let fullpath = "../..".to_string() + path.to_string();

    //let attr = try!(fs::metadata("../..".to_string() + path));
    //attr = try!(fs::metadata("/etc".to_string() + path));

    // We place the deserialized Config into an Arc, so that we can share it between
    // multiple threads in the future.  It will be immutable and not a problem to share
    let config = Arc::new(RwLock::new(Config::parse(path)));
    // let state = Arc::new(RwLock::new(State::new()));
    // {
    //     info!("Starting server \"{}\" [{}]",
    //         &config.read().unwrap().node.name, state.read().unwrap().node_id.to_hyphenated_string());
    // }


    println!("{}",path);

    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    // for i in 0..NTHREADS {

    //     let _ = thread::spawn(move || {

    //         let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    //         loop {
    //             let msg = format!("the answer is {}", i);
    //             let mut buf = [0u8; 8];

    //             println!("thread {}: Sending over message length of {}", i, msg.len());
    //             BigEndian::write_u64(&mut buf, msg.len() as u64);
    //             stream.write_all(buf.as_ref()).unwrap();
    //             stream.write_all(msg.as_ref()).unwrap();

    //             let mut buf = [0u8; 8];
    //             stream.read(&mut buf).unwrap();

    //             let msg_len = BigEndian::read_u64(&mut buf);
    //             println!("thread {}: Reading message length of {}", i, msg_len);

    //             let mut r = [0u8; 256];
    //             let s_ref = <TcpStream as Read>::by_ref(&mut stream);

    //             match s_ref.take(msg_len).read(&mut r) {
    //                 Ok(0) => {
    //                     println!("thread {}: 0 bytes read", i);
    //                 }
    //                 Ok(n) => {
    //                     println!("thread {}: {} bytes read", i, n);

    //                     let s = std::str::from_utf8(&r[..]).unwrap();
    //                     println!("thread {} read = {}", i, s);
    //                 }
    //                 Err(e) => {
    //                     panic!("thread {}: {}", i, e);
    //                 }
    //             }
    //         }
    //     });
    //}

    //loop {}
}
