use wasmtime::component::{HasData, Resource};
use wasmtime_wasi::ResourceTable;

use crate::bindings::wassel::postgres::postgres;

pub struct Postgres;

impl HasData for Postgres {
    type Data<'a> = PostgresCtxView<'a>;
}

pub trait PostgresView {
    fn postgres(&mut self) -> PostgresCtxView<'_>;
}

pub struct PostgresCtxView<'a> {
    pub table: &'a mut ResourceTable,
}

impl postgres::HostConnectionConfig for PostgresCtxView<'_> {
    async fn new(
        &mut self,
        connection_string: String,
    ) -> Result<Resource<postgres::ConnectionConfig>, wasmtime::Error> {
        let rep = self.table.push(postgres::ConnectionConfig {
            string: connection_string,
        })?;

        Ok(rep)
    }

    async fn drop(&mut self, rep: Resource<postgres::ConnectionConfig>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl postgres::HostConnection for PostgresCtxView<'_> {
    async fn open(
        &mut self,
        config: Resource<postgres::ConnectionConfig>,
    ) -> Result<Resource<postgres::Connection>, postgres::Error> {
        let config = self
            .table
            .get(&config)
            .map_err(|e| postgres::Error::Other(e.to_string()))?;

        let conn = postgres::Connection::new(config).await?;

        let res = self
            .table
            .push(conn)
            .map_err(|e| postgres::Error::Other(e.to_string()))?;

        Ok(res)
    }

    async fn query(
        &mut self,
        self_: Resource<postgres::Connection>,
        sql: String,
        params: Vec<postgres::Parameter>,
    ) -> Result<postgres::RowSet, postgres::Error> {
        let conn = self.table.get(&self_)?;

        let rows = conn.query(&sql, &params).await?;

        if rows.is_empty() {
            return Ok(postgres::RowSet {
                columns: vec![],
                rows: vec![],
            });
        }

        let columns = rows[0].columns().iter().map(Into::into).collect();

        let rows = rows
            .into_iter()
            .map(|row| {
                let mut result: Vec<postgres::Value> = Vec::with_capacity(row.len());
                for index in 0..row.len() {
                    result.push(row.try_get(index)?);
                }
                Ok(result)
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()
            .map_err(|e| postgres::Error::Query(e.to_string()))?;

        Ok(postgres::RowSet { columns, rows })
    }

    async fn execute(
        &mut self,
        self_: Resource<postgres::Connection>,
        sql: String,
        params: Vec<postgres::Parameter>,
    ) -> Result<u64, postgres::Error> {
        let conn = self.table.get(&self_)?;
        let affected = conn.execute(&sql, &params).await?;

        Ok(affected)
    }

    async fn drop(&mut self, rep: Resource<postgres::Connection>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl postgres::Host for PostgresCtxView<'_> {
    fn convert_error(&mut self, err: postgres::Error) -> wasmtime::Result<postgres::Error> {
        Ok(err)
    }
}
