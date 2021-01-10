use mysql::prelude::Queryable;
use mysql::{Pool, PooledConn};
use std::env;

pub struct MySQLConnection {
    pub pool: Pool,
}

impl MySQLConnection {
    pub fn new() -> MySQLConnection {
        let url = env::var("MYSQL_DB_CONN_STR").expect("No MySQL_DB_CONN_STR env var!");
        let pool = Pool::new(&url).unwrap();
        MySQLConnection { pool }
    }

    pub async fn execute_update(&self, statement: &str) -> Result<(), mysql::Error> {
        let conn: &mut PooledConn = &mut self.pool.get_conn().unwrap();
        conn.query_drop(statement)
    }

    pub async fn execute_query(&self, statement: &str) -> Vec<String> {
        let conn: &mut PooledConn = &mut self.pool.get_conn().unwrap();
        conn.query(&statement).unwrap()
    }
}

impl Default for MySQLConnection {
    fn default() -> Self {
        Self::new()
    }
}
