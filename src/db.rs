use crate::config::DatabaseConfig;
use mysql::*;
use mysql::prelude::*;
use std::time::Duration;

pub struct DbPool {
    pool: Pool,
}

impl DbPool {
    pub fn new(config: &DatabaseConfig, _max_connections: usize, _timeout: Duration) -> Result<Self, Box<dyn std::error::Error>> {
        let opts_builder = OptsBuilder::new()
            .ip_or_hostname(Some(config.host.clone()))
            .tcp_port(config.port)
            .user(Some(config.user.clone()))
            .pass(Some(config.password.clone()))
            .db_name(Some(config.database.clone()))
            .tcp_connect_timeout(Some(Duration::from_secs(10)))
            .read_timeout(Some(Duration::from_secs(10)))
            .write_timeout(Some(Duration::from_secs(10)));
        
        let pool = Pool::new(opts_builder)?;
        
        Ok(DbPool {
            pool,
        })
    }
    
    pub fn execute_query(&self, query: &str) -> Result<Vec<Vec<Row>>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        let results = conn.query(query)?;
        
        Ok(vec![results])
    }
    
    pub fn get_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        let version: String = conn.query_first("SELECT VERSION()")?.unwrap_or_else(|| "Unknown".to_string());
        Ok(version)
    }
}
