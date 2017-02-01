use std;
use std::io::{Error, ErrorKind};
use std::num;
use std::result;
use std::str::FromStr;
use chrono;
use chrono::{NaiveDateTime, ParseResult, ParseError};

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

use std::mem;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use bincode;


#[derive(Debug, Serialize, Deserialize)]
pub struct SensorValueArray {
    //packet_type: PacketType,
    pub id: i32,
    pub values: Vec<SensorValue>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorValue {
    #[serde(serialize_with = "dt_as_tuple", deserialize_with = "dt_from_tuple")]
    pub dt: NaiveDateTime,
    pub value: f64
}

// NaiveDateTime serializes in string representation by default; serialize it as
// (seconds, nanos) tuple instead.
fn dt_as_tuple<S>(ndt: &NaiveDateTime, serializer: &mut S) -> Result<(), S::Error>
    where S: Serializer
{
    (ndt.timestamp(), ndt.timestamp_subsec_nanos()).serialize(serializer)
}

// NaiveDateTime deserializes from string representation by default; deserialize
// it from (seconds, nano) tuple instead.
fn dt_from_tuple<D>(deserializer: &mut D) -> Result<NaiveDateTime, D::Error>
    where D: Deserializer
{
    //println!("HHHHHHHH: {}", deserializer);
    let (secs, nanos) = Deserialize::deserialize(deserializer)?;
    println!("{:?}", secs);
    Ok(NaiveDateTime::from_timestamp(secs, nanos))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TestStruct {
    pub test: u32,
    pub vec: Vec<String>,
    
}



pub fn test1() {
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

    let array: SensorValueArray = bincode::serde::deserialize(&bytes).unwrap();
    println!("{:#?}", array);
}