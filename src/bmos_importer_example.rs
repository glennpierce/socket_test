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
extern crate bincode;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod bmos_config;

use std::time;
use bmos_config::Config;

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::sync::{Arc, RwLock};


use chrono::{NaiveDateTime, ParseResult, ParseError};

use byteorder::{ByteOrder, BigEndian};

//static NTHREADS: i32 = 10;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

use clap::{Arg, App};

mod bmos_sensor;

use bmos_sensor::{SensorValue, SensorValueArray};


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

      let config_file_name: String = matches.value_of("config").unwrap_or("bmos.toml").parse().unwrap();
      const paths : [&'static str; 2] =  ["/etc/", "./"];
      let mut config = None;

      for item in paths.iter() {
          let mut path = item.to_string() + &config_file_name;

          config = Config::parse(&path);

          match config {
            None => { 
                println!("Can't read from config file {}", path); 
                continue
            },
            _ => break
          }          
      }

      if config.is_none() {
          panic!("No configuration files found");
      }

      let config = config.unwrap();
    
      // We place the deserialized Config into an Arc, so that we can share it between
      // multiple threads in the future.  It will be immutable and not a problem to share    
      let config = Arc::new(RwLock::new(config));
      //let state = Arc::new(RwLock::new(State::new()));
      //{
      //     info!("Starting server \"{}\" [{}]",
      //         &config.read().unwrap().node.name, state.read().unwrap().node_id.to_hyphenated_string());
      //}


   // println!("{}",path);

    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    loop {


        let array = SensorValueArray {
            //packet_type: PacketType::SENSOR_VALUES_ADD,
            id: 0x01010101,
            values: vec![
                SensorValue {
                    dt: NaiveDateTime::from_timestamp(1485682118, 0x04040404),
                    value: 526282.2826,
                },
                SensorValue {
                    dt: NaiveDateTime::from_timestamp(1485682118, 0x07070707),
                    value: 8262946352.6,
                },
            ],
        };

        let bytes = bincode::serde::serialize(&array, bincode::SizeLimit::Infinite).unwrap();
        println!("{:?}", bytes);

        stream.write_all(bytes.as_ref()).unwrap();

        //let _ = stream.read(&mut [0; 128]); // ignore here too

        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);

        // let msg = format!("the answer is {}", 7);
        // let mut buf = [0u8; 8];

        // println!("Sending over message length of {}", msg.len());
        // BigEndian::write_u64(&mut buf, msg.len() as u64);
        // stream.write_all(buf.as_ref()).unwrap();
        // stream.write_all(msg.as_ref()).unwrap();

        //let mut buf = [0u8; 8];
        //stream.read(&mut buf).unwrap();

        // let msg_len = BigEndian::read_u64(&mut buf);
        // println!("Reading message length of {}", msg_len);

        // let mut r = [0u8; 256];
        // let s_ref = <TcpStream as Read>::by_ref(&mut stream);

        // match s_ref.take(msg_len).read(&mut r) {
        //     Ok(0) => {
        //         println!("0 bytes read");
        //     }
        //     Ok(n) => {
        //         println!("{} bytes read", n);

        //         let s = std::str::from_utf8(&r[..]).unwrap();
        //         println!("read = {}", s);
        //     }
        //     Err(e) => {
        //         panic!("thread {}", e);
        //     }
        // }
    }

}
