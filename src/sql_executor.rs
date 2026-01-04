use crate::db::DbPool;
use crate::logger::log_error;
use log::{info, error};
use mysql::*;
use std::fs;
use std::path::Path;

pub struct SqlExecutor {
    pool: DbPool,
}

impl SqlExecutor {
    pub fn new(pool: DbPool) -> Self {
        SqlExecutor {
            pool,
        }
    }
    
    pub fn execute_file<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<Vec<Row>>, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let sql_content = fs::read_to_string(file_path)?;
        let cleaned_sql = Self::clean_sql(&sql_content);
        
        match self.pool.execute_query(&cleaned_sql) {
            Ok(results) => {
                info!("Successfully executed file: {}", file_path.display());
                Ok(results)
            },
            Err(e) => {
                log_error!(file_path.display(), "SQL Execution Error", &e.to_string());
                Err(e.into())
            }
        }
    }
    
    pub fn clean_sql(sql: &str) -> String {
        let mut cleaned = String::new();
        let mut in_multiline_comment = false;
        let mut chars = sql.chars().peekable();
        
        while let Some(c) = chars.next() {
            if in_multiline_comment {
                if c == '*' && chars.peek() == Some(&'/') {
                    in_multiline_comment = false;
                    chars.next();
                }
            } else if c == '/' && chars.peek() == Some(&'*') {
                in_multiline_comment = true;
                chars.next();
            } else if c == '-' && chars.peek() == Some(&'-') {
                while let Some(next_c) = chars.next() {
                    if next_c == '\n' {
                        cleaned.push(next_c);
                        break;
                    }
                }
            } else {
                cleaned.push(c);
            }
        }
        
        cleaned
    }
    

}
