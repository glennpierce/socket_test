extern crate rustc_serialize;
extern crate toml;
extern crate byteorder;
extern crate clap;
extern crate bincode;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use bincode::SizeLimit::Infinite;

use std::time;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::sync::{Arc, RwLock};
use chrono::{NaiveDateTime, ParseResult, ParseError};
use byteorder::{ByteOrder, BigEndian};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use clap::{Arg, App};
mod sensor;
use sensor::{SensorValue, SensorValueArray};


#[derive(Debug, Serialize, Deserialize)]
struct S {
    
    vec: Vec<String>,
     test: u32,
}

fn main() {


    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    loop {

        let array = S { test: 456, vec: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()] };

        // let array = SensorValueArray {
        //     id: 0x01010101,
        //     values: vec![
        //         SensorValue {
        //             dt: NaiveDateTime::from_timestamp(1485682118, 0x04040404),
        //             value: 526282.2826,
        //         },
        //         SensorValue {
        //             dt: NaiveDateTime::from_timestamp(1485682118, 0x07070707),
        //             value: 8262946352.6,
        //         },
        //     ],
        // };

        //let bytes = bincode::serde::serialize(&array, bincode::SizeLimit::Infinite).unwrap();
        //println!("{:?}", bytes);

        //

        let mut buf = Vec::new();
        bincode::serde::serialize_into(&mut buf, &array, Infinite).unwrap();
        println!("{:?}", buf);
        let mut buf: &[u8] = &buf;

        stream.write_all(buf).unwrap();

        //stream.write_all(bytes.as_ref()).unwrap();
        //let _ = stream.read(&mut [0; 128]); // ignore here too

        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }

}
