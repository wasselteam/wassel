use std::error::Error;
use std::fmt::Debug;

use wassel_sdk::{
    bindings::wassel::foundation::postgres::{self, Parameter, Value},
    http::{IntoResponse, Request, Response, StatusCode, handler},
};

const CONNECTION_STRING: &str =
    "host=127.0.0.1 port=25432 user=wassel-test password=wassel-test dbname=wassel-test";

macro_rules! test_case_select {
    ($sql:expr, $expected:expr) => {
        ($sql, test_query_select($sql, $expected))
    };
}

macro_rules! test_case_bind {
    ($sql:expr, $expected:expr) => {
        ($sql, test_query_bind($sql, $expected))
    };
}

#[handler]
fn handle_request(request: Request) -> impl IntoResponse {
    match request.uri().path() {
        "/select" => {
            make_response(&[
                test_case_select!("SELECT 1::BOOLEAN", true),
                test_case_select!("SELECT 123::INT2", 123i16),
                test_case_select!("SELECT 123::INT4", 123i32),
                test_case_select!("SELECT 123::INT8", 123i64),
                test_case_select!("SELECT 123::REAL", 123f32),
                test_case_select!("SELECT 123::DOUBLE PRECISION", 123f64),
                // TODO: decimal(decimal),
                // TODO: money(s64),
                test_case_select!("SELECT 'Hello, World!'::TEXT", "Hello, World!".to_owned()),
                test_case_select!("SELECT 'Hello, World!'::BYTEA", b"Hello, World!".to_vec()),
                // TODO: date(date),
                // TODO: time(time),
                // TODO: datetime(datetime),
                // TODO: interval(interval),
                // TODO: range-int32(range-int32),
                // TODO: range-int64(range-int64),
                // TODO: range-decimal(range-decimal),
                // TODO: array-int32(list<option<s32>>),
                // TODO: array-int64(list<option<s64>>),
                // TODO: array-decimal(list<option<decimal>>),
                // TODO: array-str(list<option<string>>),
                // TODO: jsonb(list<u8>),
                // TODO: hstore(list<tuple<string, option<string>>>),
                // TODO: point(point),
                // TODO: circle(circle),
                // TODO: line(line),
                // TODO: line-segment(line-segment),
                // TODO: path(path),
                // TODO: polygon(polygon),
                // TODO: cube(cube),
            ])
        }
        "/bind" => {
            make_response(&[
                test_case_bind!("SELECT $1 = 1::BOOLEAN", Parameter::Boolean(true)),
                test_case_bind!("SELECT $1 = 123::INT2", Parameter::Int16(123)),
                test_case_bind!("SELECT $1 = 123::INT4", Parameter::Int32(123)),
                test_case_bind!("SELECT $1 = 123::INT8", Parameter::Int64(123)),
                test_case_bind!("SELECT $1 = 123::REAL", Parameter::Float32(123.0)),
                test_case_bind!(
                    "SELECT $1 = 123::DOUBLE PRECISION",
                    Parameter::Float64(123.0)
                ),
                test_case_bind!(
                    "SELECT $1 = 123456.789::DECIMAL",
                    Parameter::Decimal("123456.789".to_owned())
                ),
                // TODO: money(s64),
                test_case_bind!(
                    "SELECT $1 = 'Hello, World!'::TEXT",
                    Parameter::Text("Hello, World!".to_owned())
                ),
                test_case_bind!(
                    "SELECT $1 = 'Hello, World!'::BYTEA",
                    Parameter::Binary(b"Hello, World!".to_vec())
                ),
                test_case_bind!(
                    "SELECT $1 = '2026-05-13'::DATE",
                    Parameter::Date(postgres::Date {
                        year: 2026,
                        month: 05,
                        day: 13
                    })
                ),
                test_case_bind!(
                    "SELECT $1 = '12:34:56.123'::TIME",
                    Parameter::Time(postgres::Time {
                        hour: 12,
                        minute: 34,
                        second: 56,
                        nanosecond: 123000
                    })
                ),
                test_case_bind!(
                    "SELECT $1 = '2026-05-13 12:34:56.123'::TIMESTAMP",
                    Parameter::Datetime(postgres::Datetime {
                        date: postgres::Date {
                            year: 2026,
                            month: 05,
                            day: 13
                        },
                        time: postgres::Time {
                            hour: 12,
                            minute: 34,
                            second: 56,
                            nanosecond: 123000
                        },
                        offset: None,
                    })
                ),
                test_case_bind!(
                    "SELECT $1 = '2026-05-13 12:34:56.123 Etc/GMT-6'::TIMESTAMP",
                    Parameter::Datetime(postgres::Datetime {
                        date: postgres::Date {
                            year: 2026,
                            month: 05,
                            day: 13
                        },
                        time: postgres::Time {
                            hour: 12,
                            minute: 34,
                            second: 56,
                            nanosecond: 123000
                        },
                        offset: Some(6),
                    })
                ),
                test_case_bind!(
                    "SELECT $1 = '3 month 5 days 02:47:33'::INTERVAL",
                    Parameter::Interval(postgres::Interval {
                        months: 3,
                        days: 5,
                        microseconds: 10_053_000
                    })
                ),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: range-int32(range-int32),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: range-int64(range-int64),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: range-decimal(range-decimal),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: array-int32(list<option<s32>>),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: array-int64(list<option<s64>>),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: array-decimal(list<option<decimal>>),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: array-str(list<option<string>>),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: jsonb(list<u8>),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: hstore(list<tuple<string, option<string>>>),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: point(point),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: circle(circle),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: line(line),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: line-segment(line-segment),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: path(path),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: polygon(polygon),
                // test_case_bind!("SELECT $1 = val::TYPE", Parameter::Unknown()), // TODO: cube(cube),
            ])
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

fn make_response(results: &[(&str, Result<(), Box<dyn Error>>)]) -> Response {
    (get_status(results), make_table(results)).into_response()
}

fn get_status(results: &[(&str, Result<(), Box<dyn Error>>)]) -> StatusCode {
    match results.into_iter().any(|(_, r)| r.is_err()) {
        false => StatusCode::OK,
        true => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn make_table(results: &[(&str, Result<(), Box<dyn Error>>)]) -> String {
    let mut table = results
        .into_iter()
        .map(|(s, r)| match r {
            Ok(()) => format!("'{s}' => OK"),
            Err(e) => format!("'{s}' => ERR: {e}"),
        })
        .collect::<Vec<String>>()
        .join("\n");

    table.push_str("\n");
    table
}

trait FromValue: Sized {
    type Error;

    fn from_value(value: Value) -> Result<Self, Self::Error>;
}

fn test_query_select<T, E>(sql: &str, expected: T) -> Result<(), Box<dyn Error>>
where
    T: FromValue<Error = E> + PartialEq + Debug,
    E: Into<Box<dyn Error>>,
{
    let config = postgres::ConnectionConfig::new(CONNECTION_STRING);
    let conn = postgres::Connection::open(config)?;
    let rows = conn.query(sql, &[])?;
    let Some(val) = rows.rows.first().and_then(|r| r.first()) else {
        return Err("Empty rowset returned".to_owned().into());
    };

    let val = T::from_value(val.clone()).map_err(Into::into)?;
    match val == expected {
        false => Err(format!("Expected: {expected:?}, actual: {val:?}").into()),
        true => Ok(()),
    }
}

fn test_query_bind(sql: &str, parameter: Parameter) -> Result<(), Box<dyn Error>> {
    let config = postgres::ConnectionConfig::new(CONNECTION_STRING);
    let conn = postgres::Connection::open(config)?;
    let rows = conn.query(sql, &[parameter])?;
    let Some(Value::Boolean(val)) = rows.rows.first().and_then(|r| r.first()) else {
        return Err("Empty rowset returned".to_owned().into());
    };
    match val {
        false => Err(format!("Values are not equal").into()),
        true => Ok(()),
    }
}

impl FromValue for bool {
    type Error = Box<dyn std::error::Error>;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(val) => Ok(val),
            _ => Err(format!("{value:?}").into()),
        }
    }
}

impl FromValue for i16 {
    type Error = Box<dyn std::error::Error>;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int16(val) => Ok(val),
            _ => Err(format!("{value:?}").into()),
        }
    }
}

impl FromValue for i32 {
    type Error = Box<dyn std::error::Error>;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int32(val) => Ok(val),
            _ => Err(format!("{value:?}").into()),
        }
    }
}

impl FromValue for i64 {
    type Error = Box<dyn std::error::Error>;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int64(val) => Ok(val),
            _ => Err(format!("{value:?}").into()),
        }
    }
}

impl FromValue for f32 {
    type Error = Box<dyn std::error::Error>;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float32(val) => Ok(val),
            _ => Err(format!("{value:?}").into()),
        }
    }
}

impl FromValue for f64 {
    type Error = Box<dyn std::error::Error>;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float64(val) => Ok(val),
            _ => Err(format!("{value:?}").into()),
        }
    }
}

impl FromValue for String {
    type Error = Box<dyn std::error::Error>;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(val) => Ok(val),
            _ => Err(format!("{value:?}").into()),
        }
    }
}

impl FromValue for Vec<u8> {
    type Error = Box<dyn std::error::Error>;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Binary(val) => Ok(val),
            _ => Err(format!("{value:?}").into()),
        }
    }
}
