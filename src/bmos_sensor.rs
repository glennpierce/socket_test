#[derive(Debug)]
struct BmosSensor {
    id: i32,
    name: String,
    namespace: String,
    description: String,

    name: String,
    name: String,
    name: String,
    name: String,
    name: String,
    name: String,

    data: Option<Vec<u8>>,
}

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
