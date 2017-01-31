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


#[derive(Debug)]
pub enum BmosTimeError {
    INVALID_BMOS_TIME_STRING,
    ChronoParse(chrono::ParseError),
    ParseInt(num::ParseIntError),
}

impl From<std::num::ParseIntError> for BmosTimeError {
    fn from(err: std::num::ParseIntError) -> BmosTimeError {
        BmosTimeError::ParseInt(err)
    }
}

type BmosTimeConverterResult<T> = result::Result<T, BmosTimeError>;

/// Converts a time string of the format 1485547437.987534 to a chronos::NaiveDateTime object
pub trait BmosTimeConverter {
    fn from_bmos_time_string(&str) -> BmosTimeConverterResult<NaiveDateTime>;
}


/// # Examples
///
/// ```
/// use chrono::{NaiveDateTime, ParseResult, ParseError};
/// use bmos_sensor::BmosTimeConverter;
///
/// let date = NaiveDateTime::from_bmos_time_string("1485547437.987534").unwrap();
/// println!("{:?}", date);
/// ```
impl BmosTimeConverter for NaiveDateTime {
    fn from_bmos_time_string(s: &str) -> BmosTimeConverterResult<NaiveDateTime> {
        let len = s.chars().count();

        if len != 17 {
            return Err(BmosTimeError::INVALID_BMOS_TIME_STRING);
        }

        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 2 {
            return Err(BmosTimeError::INVALID_BMOS_TIME_STRING);
        }

        let timestamp = try!(parts[0].parse::<i64>());
        let microseconds = try!(parts[1].parse::<u32>());

        return Ok(NaiveDateTime::from_timestamp(timestamp, microseconds * 1000));
    }
}


// #[derive(Debug)]
// struct BmosSensor {
//     id: i32,
//     name: String,
//     namespace: String,
//     description: String,

//     // name: String,
//     // name: String,
//     // name: String,
//     // name: String,
//     // name: String,
//     // name: String,

//     // data: Option<Vec<u8>>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// enum PacketType {
//     SENSOR_VALUES_ADD,
// }


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
    let (secs, nanos) = Deserialize::deserialize(deserializer)?;
    Ok(NaiveDateTime::from_timestamp(secs, nanos))
}

/*


#[derive(Debug)]
struct BmosSensorValue {
    dt: NaiveDateTime,   // byte format i64 + u32 = 8 + 4 = 12 bytes
    vec: Vec<i64>        // min bytes 8 bytes for one entry
}


// = Vec::new();

#[derive(Debug, Clone, Copy)]
struct BmosSensorValueArray {
    id: i32,
    len: i32,
    values: Vec<BmosSensorValue>
}


impl BmosSensorValueByteReader {
    pub fn from_byte_array(raw: &[u8]) -> Option<BmosSensorValueArray> {
        if raw.len() < 28 {
            None
        } 

        self.id = read_u32(raw);
        self.len = read_u32(raw[4..]);
        self.values = Vec<BmosSensorValue>::with_capacity(self.len);
        


        let mut rdr = Cursor::new(raw[8..]);
        // Note that we use type parameters to indicate which kind of byte order
        // we want!
        assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());

   //         Some(MyPacketReader { raw: raw })
        
    }
}

fn read_u32(data: &[u8]) -> u32 {
    (data[0] as u32 << 24) +
    (data[1] as u32 << 16) +
    (data[2] as u32 <<  8) + 
    (data[3] as u32 <<  0)
}

//  .execute("CREATE TABLE sensors (
//                     id              SERIAL PRIMARY KEY,
//                     name            VARCHAR(100) NOT NULL,
//                     namespace       VARCHAR(500) NOT NULL,
//                     description     VARCHAR(200) NOT NULL,
//                     time_created    TIMESTAMP NOT NULL,
//                     last_timestamp  TIMESTAMP DEFAULT '1970-01-01 00:00:00.000',
//                     first_timestamp TIMESTAMP DEFAULT '1970-01-01 00:00:00.000',
//                     max_meter_value INTEGER DEFAULT 9999,
//                     type_id         INTEGER DEFAULT 1,
//                     unit_id         INTEGER DEFAULT 1,
//                     resolution      REAL DEFAULT 0.0,
//                     accuracy        REAL DEFAULT 0.0,
//                     kw_calibration_factor REAL DEFAULT 1.0 NOT NULL,
//                     sample_interval INTEGER DEFAULT 300 NOT NULL,
//                     ever_increasing INTEGER DEFAULT 0,
//                     FOREIGN KEY (type_id) REFERENCES sensor_value_types(id),
//                     FOREIGN KEY (unit_id) REFERENCES sensor_value_units(id)

*/





impl SensorValueArray {
     fn get_bytes(&self) -> Vec<u8> {
         let bytes = bincode::serde::serialize(&self, bincode::SizeLimit::Infinite).unwrap();
         let size = mem::size_of::<SensorValueArray>();
         bytes
     }
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