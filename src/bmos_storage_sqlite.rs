use bmos_storage::{BmosStorage, BmosStorageResult, BmosStorageError};

use std::io;
use std::error::Error;
use std::convert::From;
use time::Timespec;

use rusqlite;
use rusqlite::Connection;


#[derive(Debug)]
pub struct BmosSqliteStorage {
    conn: Connection,
}

impl From<rusqlite::Error> for BmosStorageError {
    fn from(val: rusqlite::Error) -> Self {
        BmosStorageError { detail: val.description().to_owned() }
    }
}

impl BmosSqliteStorage {
    pub fn new() -> BmosSqliteStorage {
        BmosSqliteStorage { conn: Connection::open_in_memory().unwrap() }
    }

    fn create_types_and_units(&self) -> BmosStorageResult<()> {
        self.conn
            .execute("CREATE TABLE sensor_value_types (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR(100) UNIQUE
                )",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_types (name) VALUES ('Temperature')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_types (name) VALUES ('Humidity')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_types (name) VALUES ('Carbon dioxide')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_types (name) VALUES ('Litre')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_types (name) VALUES ('SolarIncidience')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_types (name) VALUES ('Electrical')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_types (name) VALUES ('HeatFlow')",
                     &[])
            .unwrap();

        self.conn
            .execute("CREATE TABLE sensor_value_units (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR(100) UNIQUE,
                    description     VARCHAR(200)
                )",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('C', 'Celsius')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('%RH', 'Relative Humidity Percentage')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('PPM', 'Parts Per Million')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('W^m2', 'Watts per metre squared')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('kW', 'Kilowatts')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('W', 'Watts')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('kWh', 'Kilowatt Hours')",
                     &[])
            .unwrap();

        Ok(())
    }

    fn create_sensor_value_table(&self) -> BmosStorageResult<()> {
        self.conn
            .execute("CREATE TABLE sensor_values (
                    ts              TIMESTAMP NOT NULL PRIMARY KEY,
                    sensor_id       INTEGER NOT NULL,
                    value           REAL NOT NULL DEFAULT 'NaN',
                    FOREIGN KEY     (sensor_id) REFERENCES sensors(id)
                )",
                     &[])
            .unwrap();

        Ok(())
    }
}

impl BmosStorage for BmosSqliteStorage {
    fn create_tables(&self) -> BmosStorageResult<()> {

        //No need to check this exists as the table is in memory only.
        self.create_types_and_units();

        self.conn
            .execute("CREATE TABLE sensors (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR(100) NOT NULL,
                    namespace       VARCHAR(500) NOT NULL,
                    description     VARCHAR(200) NOT NULL,
                    time_created    TIMESTAMP NOT NULL,
                    last_timestamp  TIMESTAMP DEFAULT '1970-01-01 00:00:00.000',
                    first_timestamp TIMESTAMP DEFAULT '1970-01-01 00:00:00.000',
                    max_meter_value INTEGER DEFAULT 9999,
                    type_id         INTEGER DEFAULT 1,
                    unit_id         INTEGER DEFAULT 1,
                    resolution      REAL DEFAULT 0.0,
                    accuracy        REAL DEFAULT 0.0,
                    kw_calibration_factor REAL DEFAULT 1.0 NOT NULL,
                    sample_interval INTEGER DEFAULT 300 NOT NULL,
                    ever_increasing INTEGER DEFAULT 0,
                    FOREIGN KEY (type_id) REFERENCES sensor_value_types(id),
                    FOREIGN KEY (unit_id) REFERENCES sensor_value_units(id)
                )",
                     &[])
            .unwrap();

        self.create_sensor_value_table();

        Ok(())
    }
}
