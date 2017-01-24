//#![deny(warnings)]
extern crate hyper;
extern crate env_logger;

use std::thread::spawn;

mod bmos_server;

fn main() {

    spawn(bmos_server::serve);
}
