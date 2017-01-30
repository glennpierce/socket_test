//use bmos_sensor::BmosTimeConverter;
use bmos_sensor::SensorValueArray;
use rusqlite::Error as SqliteError;

#[derive(Debug)]
pub enum BmosStorageError {
    BmosStorageSqliteError(SqliteError),
}

//trait BmosStorageConnection;
pub type BmosStorageResult<T> = Result<T, BmosStorageError>;


pub trait BmosStorage {
    //fn get_connection(&self) -> BmosStorageConnection;
    //fn new(&self) -> BmosStorage;
    fn create_tables(&self) -> BmosStorageResult<()>;
    fn insert_sensor_values(&self, sensor_value_array : &SensorValueArray) -> BmosStorageResult<()>;
}





// #[derive(Debug)]
// pub enum BmosTimeError {
//     INVALID_BMOS_TIME_STRING,
//     ChronoParse(chrono::ParseError),
//     ParseInt(num::ParseIntError),
// }

// impl From<std::num::ParseIntError> for BmosTimeError {
//     fn from(err: std::num::ParseIntError) -> BmosTimeError {
//         BmosTimeError::ParseInt(err)
//     }
// }

//type BmosTimeConverterResult<T> = result::Result<T, BmosTimeError>;
