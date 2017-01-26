use bmos_storage::{BmosStorage, BmosStorageResult, BmosStorageError};

use std::io;
use std::error::Error;
use std::convert::From;
use time::Timespec;

use rusqlite;
use rusqlite::Connection;


struct BmosSqliteStorage {
    conn: Connection,
}

impl From<rusqlite::Error> for BmosStorageError {
    fn from(val: rusqlite::Error) -> Self {
        BmosStorageError { detail: val.description().to_owned() }
    }
}

impl BmosSqliteStorage {
    pub fn new(&self) -> BmosSqliteStorage {
        BmosSqliteStorage { conn: Connection::open_in_memory().unwrap() }
    }
}

impl BmosStorage for BmosSqliteStorage {
    fn create_sensors_table(&self) -> BmosStorageResult<()> {

        //No need to check this exists as the table is in memory only.

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
                )",
                     &[])
            .unwrap();

        Ok(())
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
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('C', 'Celsius')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('C', 'Celsius')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('C', 'Celsius')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('C', 'Celsius')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('C', 'Celsius')",
                     &[])
            .unwrap();

        self.conn
            .execute("INSERT INTO sensor_value_units (name, description) VALUES ('C', 'Celsius')",
                     &[])
            .unwrap();
            
        Ok(())
    }
}


// FOREIGN KEY(trackartist) REFERENCES artist(artistid)


/*




CREATE TABLE sensor_value_units (
    id SERIAL PRIMARY KEY,
    name character varying(50) UNIQUE,
    description character varying(200)
);

ALTER TABLE sensor_value_units OWNER TO bmos;
INSERT INTO sensor_value_units (name, description) VALUES ('C', 'Celsius');
INSERT INTO sensor_value_units (name, description) VALUES ('%RH', 'Relative Humidity Percentage');
INSERT INTO sensor_value_units (name, description) VALUES ('PPM', 'Parts Per Million');
INSERT INTO sensor_value_units (name, description) VALUES ('L', 'Litres');
INSERT INTO sensor_value_units (name, description) VALUES ('W^m2', 'Watts per metre squared');
INSERT INTO sensor_value_units (name, description) VALUES ('kW', 'Kilowatts');
INSERT INTO sensor_value_units (name, description) VALUES ('W', 'Watts');
INSERT INTO sensor_value_units (name, description) VALUES ('kWh', 'Kilowatt Hours');


*/

/*


CREATE TABLE sensors (



    FOREIGN KEY (type_id) REFERENCES sensor_value_types(id),
    FOREIGN KEY (unit_id) REFERENCES sensor_value_units(id),
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

ALTER TABLE sensors OWNER TO bmos;
ALTER TABLE sensors ADD CONSTRAINT sensor_namespace_index UNIQUE (name, namespace);




import sys

parent_table_name = sys.argv[1]

quarters = ('%s-01-01 00:00:00.000000+00:00',
            '%s-04-01 00:00:00.000000+00:00',
            '%s-07-01 00:00:00.000000+00:00',
            '%s-10-01 00:00:00.000000+00:00')

years = ["2005", "2006", "2007", "2008", "2009", "2010", "2011", "2012", "2013", "2014", "2015", "2016",
         "2017", "2018", "2019", "2020"]

dates = []

for y, year in enumerate(years):
    for i, quarter in enumerate(quarters):
        q=i+1
        start = quarter % (year,)
        if i < 3:
            end = quarters[i+1] % (year,)
        else:
            if y < len(years) - 1:
                end = quarters[0] % (years[y+1],)
        dates.append((start, end, "%s_%sq%d" %(parent_table_name, year, q)))


print """
CREATE FUNCTION plpgsql_call_handler() RETURNS language_handler AS
'$libdir/plpgsql' LANGUAGE C;
CREATE TRUSTED LANGUAGE plpgsql
HANDLER "plpgsql_call_handler";
GRANT USAGE ON LANGUAGE plpgsql TO bmos;
"""

print """
CREATE OR REPLACE FUNCTION daily_quarter_trunc(ts TIMESTAMPTZ)
  RETURNS TEXT
AS $$
BEGIN
  return ts::date + interval '6 hour' * (date_part('hour', ts)::integer / 6);
END;
$$LANGUAGE plpgsql
IMMUTABLE;
"""




print """
DROP TABLE IF EXISTS %s;
CREATE TABLE %s (
    ts timestamp with time zone NOT NULL,
    value double precision NOT NULL DEFAULT 'NaN',
    sensor_id integer NOT NULL,
    status tridium_status NOT NULL DEFAULT 'unknown'::tridium_status,
    FOREIGN KEY (sensor_id) REFERENCES sensors(id)
);

ALTER TABLE %s OWNER TO bmos;
ALTER TABLE %s ADD CONSTRAINT timestamp_sensor_index UNIQUE (ts, sensor_id);
CREATE INDEX timestamp_idx ON %s (ts) WITH (fillfactor = 10);
""" % (parent_table_name, parent_table_name, parent_table_name, parent_table_name, parent_table_name)

trigger_string = ""
first=True

for start, end, table_name in dates:
    print "DROP TABLE IF EXISTS %s;" % (table_name,)
    print "CREATE TABLE %s (CHECK ( ts >= TIMESTAMP WITH TIME ZONE '%s' AND ts < TIMESTAMP WITH TIME ZONE '%s' )) " \
          "INHERITS (%s);""" % (table_name, start, end, parent_table_name)
    print "ALTER TABLE %s OWNER TO bmos;" % (table_name,)

for start, end, table_name in dates:
    print "DROP INDEX IF EXISTS %s_timestamp_idx;" % (table_name,)
    print "CREATE INDEX %s_timestamp_idx ON %s(ts);" % (table_name, table_name)

    print "DROP INDEX IF EXISTS %s_sensor_id_idx;" % (table_name,)
    print "CREATE INDEX %s_sensor_id_idx ON %s(sensor_id);" % (table_name, table_name)

    print "DROP INDEX IF EXISTS %s_timestamp_sensor_id_idx;" % (table_name,)
    print "CREATE INDEX %s_timestamp_sensor_id_idx ON %s(ts, sensor_id);" % (table_name, table_name)

    print "DROP INDEX IF EXISTS %s_sensor_id_timestamp_idx;" % (table_name,)
    print "CREATE INDEX %s_sensor_id_timestamp_idx ON %s(sensor_id, ts);" % (table_name, table_name)

    print "DROP INDEX IF EXISTS %s_sensor_id_timestamp_value_idx;" % (table_name,)
    print "CREATE INDEX %s_sensor_id_timestamp_value_idx ON %s(sensor_id, ts, value);" % (table_name, table_name)

    print "DROP INDEX IF EXISTS %s_day_quarter_idx;" % (table_name,)    
    print "CREATE INDEX %s_day_quarter_idx ON %s(daily_quarter_trunc(ts));" % (table_name, table_name)

    print "DROP INDEX IF EXISTS %s_sensor_id_timestamp_value_inx;" % (table_name,)
    print "CREATE INDEX %s_sensor_id_timestamp_value_inx ON %s(sensor_id, ts, value);" % (table_name, table_name)

    print "ALTER TABLE %s DROP CONSTRAINT IF EXISTS %s_ts_sensor_unq;" % (table_name, table_name)
    print "ALTER TABLE %s ADD CONSTRAINT %s_ts_sensor_unq UNIQUE (ts, sensor_id);" % (table_name, table_name)


for start, end, table_name in dates:
    #Create a trigger on mother table to redirect records into child tables.
    if first:
        trigger_string = """IF ( NEW.ts >= TIMESTAMP WITH TIME ZONE '%s' AND NEW.ts < TIMESTAMP WITH TIME ZONE '%s' ) """ \
                         """ \nTHEN INSERT INTO %s VALUES (NEW.*);\n""" % (start, end, table_name)
        first = False
    else:
        trigger_string += """ELSIF ( NEW.ts >= TIMESTAMP WITH TIME ZONE '%s' AND NEW.ts < TIMESTAMP WITH TIME ZONE '%s' ) """ \
                          """\nTHEN INSERT INTO %s VALUES (NEW.*);\n""" % (start, end, table_name)


print """
CREATE OR REPLACE FUNCTION %s_timestamp_sensor_func_insert_trigger()
RETURNS TRIGGER AS $%s_timestamp_sensor_func_insert_trigger$
BEGIN""" % (parent_table_name,parent_table_name)

print trigger_string[0:-1]

print """ELSE\n""" \
      """RAISE EXCEPTION 'Date out of range.  Fix the %s_timestamp_sensor_func_insert_trigger() function!';\n""" \
      """END IF;\n""" \
      """RETURN NULL;\nEND;\n$%s_timestamp_sensor_func_insert_trigger$\nLANGUAGE plpgsql;""" % (parent_table_name,parent_table_name)
 

#If multiple triggers of the same kind are defined for the same event, they will be fired in alphabetical order by name.
#so countrows will be fired first which is what we need
print "DROP TRIGGER IF EXISTS %s_trigger_timestamp_sensor_insert ON %s;" % (parent_table_name,parent_table_name)
print """
CREATE TRIGGER %s_trigger_timestamp_sensor_insert_%s
BEFORE INSERT ON %s
      FOR EACH ROW EXECUTE PROCEDURE %s_timestamp_sensor_func_insert_trigger();
""" % (parent_table_name,parent_table_name,parent_table_name,parent_table_name)


*/




/*

fn main() {
    let conn = Connection::connect("postgres://postgres@localhost", TlsMode::None).unwrap();
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();
    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {}", person.name);
    }
}

*/
