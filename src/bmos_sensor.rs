use std;
use std::io::{Error, ErrorKind};
use std::num;
use std::result;
use std::str::FromStr;
use chrono;
use chrono::{NaiveDateTime, ParseResult, ParseError};


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

type Result<T> = result::Result<T, BmosTimeError>;

pub trait BmosTimeConverter { 
    fn from_bmos_time_string(&str) -> Result<NaiveDateTime>;
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

    fn from_bmos_time_string(s : &str) -> Result<NaiveDateTime> { 
        let len = s.chars().count();
   
        if len != 17 {
             return Err(BmosTimeError::INVALID_BMOS_TIME_STRING);
        }

        let parts : Vec<&str> = s.split('.').collect();

        if parts.len() != 2 {
            return Err(BmosTimeError::INVALID_BMOS_TIME_STRING);
        }

        let timestamp = try!(parts[0].parse::<i64>());
        let microseconds = try!(parts[1].parse::<u32>());
        
        return Ok(NaiveDateTime::from_timestamp(timestamp, microseconds*1000));
    }
}


// #[derive(Debug)]
// struct BmosSensor {
//     id: i32,
//     name: String,
//     namespace: String,
//     description: String,

//     name: String,
//     name: String,
//     name: String,
//     name: String,
//     name: String,
//     name: String,

//     data: Option<Vec<u8>>,
// }

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
