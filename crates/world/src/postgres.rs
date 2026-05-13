use tokio::task::JoinHandle;
use tokio_postgres::{
    Client, NoTls, Row, connect,
    types::{FromSql, IsNull, ToSql, Type, private::BytesMut},
};
use wasmtime_wasi::ResourceTableError;

use crate::wassel::foundation::postgres;

pub struct PgConnectionConfig {
    pub string: String,
}

pub struct PgConnection {
    client: Client,
    connection_task: JoinHandle<()>,
}

impl PgConnection {
    pub async fn new(config: &PgConnectionConfig) -> Result<Self, tokio_postgres::Error> {
        let (client, conn) = connect(&config.string, NoTls).await?;

        let handle = tokio::spawn(async {
            if let Err(e) = conn.await {
                eprintln!("Error polling connection: {e}");
            }
        });

        Ok(Self {
            client,
            connection_task: handle,
        })
    }

    pub async fn query(
        &self,
        statement: &str,
        params: &[postgres::Parameter],
    ) -> Result<Vec<Row>, tokio_postgres::Error> {
        let sql_params: Vec<_> = params
            .iter()
            .map(|p| (p as &(dyn ToSql + Sync), p.get_oid()))
            .collect();
        self.client.query_typed(statement, &sql_params).await
    }

    pub async fn execute(
        &self,
        statement: &str,
        params: &[postgres::Parameter],
    ) -> Result<u64, tokio_postgres::Error> {
        let sql_params: Vec<_> = params
            .iter()
            .map(|p| (p as &(dyn ToSql + Sync), p.get_oid()))
            .collect();

        self.client.execute_typed(statement, &sql_params).await
    }
}

impl Drop for PgConnection {
    fn drop(&mut self) {
        self.connection_task.abort();
    }
}

impl postgres::Parameter {
    fn data_type(&self) -> postgres::DataType {
        match self {
            postgres::Value::Boolean(_) => postgres::DataType::Boolean,
            postgres::Value::Int16(_) => postgres::DataType::Int16,
            postgres::Value::Int32(_) => postgres::DataType::Int32,
            postgres::Value::Int64(_) => postgres::DataType::Int64,
            postgres::Value::Float32(_) => postgres::DataType::Float32,
            postgres::Value::Float64(_) => postgres::DataType::Float64,
            postgres::Value::Decimal(_) => postgres::DataType::Decimal,
            postgres::Value::Money(_) => postgres::DataType::Money,
            postgres::Value::Uuid(_) => postgres::DataType::Uuid,
            postgres::Value::Text(_) => postgres::DataType::Text,
            postgres::Value::Binary(_) => postgres::DataType::Binary,
            postgres::Value::Date(_) => postgres::DataType::Date,
            postgres::Value::Time(_) => postgres::DataType::Time,
            postgres::Value::Datetime(_) => postgres::DataType::Datetime,
            postgres::Value::Interval(_) => postgres::DataType::Interval,
            postgres::Value::RangeInt32(_) => postgres::DataType::RangeInt32,
            postgres::Value::RangeInt64(_) => postgres::DataType::RangeInt64,
            postgres::Value::RangeDecimal(_) => postgres::DataType::RangeDecimal,
            postgres::Value::ArrayInt32(_) => postgres::DataType::ArrayInt32,
            postgres::Value::ArrayInt64(_) => postgres::DataType::ArrayInt64,
            postgres::Value::ArrayDecimal(_) => postgres::DataType::ArrayDecimal,
            postgres::Value::ArrayStr(_) => postgres::DataType::ArrayStr,
            postgres::Value::Jsonb(_) => postgres::DataType::Jsonb,
            postgres::Value::Hstore(_) => postgres::DataType::Hstore,
            postgres::Value::Point(_) => postgres::DataType::Point,
            postgres::Value::Circle(_) => postgres::DataType::Circle,
            postgres::Value::Line(_) => postgres::DataType::Line,
            postgres::Value::LineSegment(_) => postgres::DataType::LineSegment,
            postgres::Value::Path(_) => postgres::DataType::Path,
            postgres::Value::Polygon(_) => postgres::DataType::Polygon,
            postgres::Value::Cube(_) => postgres::DataType::Cube,
            _ => postgres::DataType::Other("UNKNOWN".to_owned()),
        }
    }

    // TODO: Utilize DataType
    fn get_oid(&self) -> Type {
        match self.data_type() {
            postgres::DataType::Binary => Type::BYTEA,
            postgres::DataType::Boolean => Type::BOOL,
            postgres::DataType::Int32 => Type::INT4,
            postgres::DataType::Int64 => Type::INT8,
            postgres::DataType::Text => Type::TEXT,
            postgres::DataType::Datetime => Type::TIMESTAMP,
            postgres::DataType::Uuid => Type::UUID,
            postgres::DataType::PgNull => Type::ANY,
            // TODO: Other types
            _ => Type::UNKNOWN,
        }
    }
}

impl<'a> FromSql<'a> for postgres::Value {
    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let val = match *ty {
            Type::BYTEA => Self::Binary(Vec::<u8>::from_sql(ty, raw)?),
            Type::BOOL => Self::Boolean(bool::from_sql(ty, raw)?),
            Type::INT2 => Self::Int16(i16::from_sql(ty, raw)?),
            Type::INT4 => Self::Int32(i32::from_sql(ty, raw)?),
            Type::INT8 => Self::Int64(i64::from_sql(ty, raw)?),
            Type::FLOAT4 => Self::Float32(f32::from_sql(ty, raw)?),
            Type::FLOAT8 => Self::Float64(f64::from_sql(ty, raw)?),
            Type::TEXT => Self::Text(String::from_sql(ty, raw)?),
            Type::TIMESTAMP => return Err("TIMESTAMP is not yet implemented".into()),
            Type::UUID => return Err("UUID is not yet implemented".into()),
            _ => Self::Other(raw.to_vec()),
        };

        Ok(val)
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    fn from_sql_null(_ty: &Type) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Self::PgNull)
    }
}

impl ToSql for postgres::Parameter {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        match self {
            postgres::Parameter::Binary(val) => val.to_sql(ty, out),
            postgres::Parameter::Boolean(val) => val.to_sql(ty, out),
            postgres::Parameter::Int16(val) => val.to_sql(ty, out),
            postgres::Parameter::Int32(val) => val.to_sql(ty, out),
            postgres::Parameter::Int64(val) => val.to_sql(ty, out),
            postgres::Parameter::Float32(val) => val.to_sql(ty, out),
            postgres::Parameter::Float64(val) => val.to_sql(ty, out),
            postgres::Parameter::Text(val) => val.to_sql(ty, out),
            postgres::Parameter::Datetime(_) => {
                Err("Datetime is not yet implemented".into())
            }
            postgres::Parameter::Uuid(_) => Err("Uuid is not yet implemented".into()),
            postgres::Parameter::PgNull => Ok(IsNull::Yes),
            _ => {
                Err(
                    format!("{} is not yet implemented", self.get_oid()).into(),
                )
            }
        }
    }

    fn accepts(_ty: &Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    tokio_postgres::types::to_sql_checked!();
}

impl From<tokio_postgres::types::Type> for postgres::DataType {
    fn from(value: tokio_postgres::types::Type) -> Self {
        match value {
            Type::BYTEA => Self::Binary,
            Type::BOOL => Self::Boolean,
            Type::INT4 => Self::Int32,
            Type::INT8 => Self::Int64,
            Type::TEXT => Self::Text,
            Type::TIMESTAMP => Self::Datetime,
            Type::UUID => Self::Uuid,
            _ => Self::Other(value.to_string()),
        }
    }
}

impl From<&tokio_postgres::Column> for postgres::Column {
    fn from(value: &tokio_postgres::Column) -> Self {
        Self {
            name: value.name().to_owned(),
            data_type: value.type_().to_owned().into(),
        }
    }
}

impl From<tokio_postgres::Error> for postgres::Error {
    fn from(value: tokio_postgres::Error) -> Self {
        let Some(db_error) = value.as_db_error() else {
            return Self::Other(value.to_string());
        };

        let db_error = postgres::DatabaseError {
            severity: db_error.severity().to_owned(),
            parsed_severity: db_error.parsed_severity().map(Into::into),
            code: db_error.code().code().to_string(),
            message: db_error.message().to_owned(),
            detail: db_error.detail().map(|s| s.to_owned()),
            hint: db_error.hint().map(|s| s.to_owned()),
            position: db_error.position().map(|p| p.to_owned().into()),
            where_: db_error.where_().map(|s| s.to_owned()),
            schema: db_error.schema().map(|s| s.to_owned()),
            table: db_error.table().map(|s| s.to_owned()),
            column: db_error.column().map(|s| s.to_owned()),
            datatype: db_error.datatype().map(|s| s.to_owned()),
            constraint: db_error.constraint().map(|s| s.to_owned()),
            file: db_error.file().map(|s| s.to_owned()),
            line: db_error.line().map(|n| n.to_owned()),
            routine: db_error.routine().map(|s| s.to_owned()),
        };

        Self::Database(db_error)
    }
}

impl From<wasmtime::Error> for postgres::Error {
    fn from(value: wasmtime::Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<ResourceTableError> for postgres::Error {
    fn from(value: ResourceTableError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<tokio_postgres::error::Severity> for postgres::Severity {
    fn from(value: tokio_postgres::error::Severity) -> Self {
        match value {
            tokio_postgres::error::Severity::Panic => Self::Panic,
            tokio_postgres::error::Severity::Fatal => Self::Fatal,
            tokio_postgres::error::Severity::Error => Self::Error,
            tokio_postgres::error::Severity::Warning => Self::Warning,
            tokio_postgres::error::Severity::Notice => Self::Notice,
            tokio_postgres::error::Severity::Debug => Self::Debug,
            tokio_postgres::error::Severity::Info => Self::Info,
            tokio_postgres::error::Severity::Log => Self::Log,
        }
    }
}

impl From<tokio_postgres::error::ErrorPosition> for postgres::ErrorPosition {
    fn from(value: tokio_postgres::error::ErrorPosition) -> Self {
        match value {
            tokio_postgres::error::ErrorPosition::Original(o) => Self::Original(o),
            tokio_postgres::error::ErrorPosition::Internal { position, query } => {
                Self::Internal(postgres::ErrorPositionInternal { position, query })
            }
        }
    }
}
