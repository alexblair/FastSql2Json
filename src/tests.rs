#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;
    
    #[test]
    fn test_config_from_file() {
        // 创建临时配置文件
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        let config_content = r#"
[database]
host = "localhost"
port = 3306
user = "test_user"
password = "test_pass"
database = "test_db"

[app]
start_dir = "./test_sql"

[file_intervals]
"./test_sql/query1.sql" = 60
"#;
        
        fs::write(&config_path, config_content).unwrap();
        
        // 测试配置文件读取
        let config = Config::from_file(&config_path).unwrap();
        
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 3306);
        assert_eq!(config.app.start_dir, "./test_sql");
        assert_eq!(config.get_interval("./test_sql/query1.sql"), Some(60));
        assert_eq!(config.get_interval("./test_sql/query2.sql"), None);
    }
    
    #[test]
    fn test_scan_sql_files() {
        // 创建临时目录结构
        let temp_dir = tempdir().unwrap();
        let sql_dir = temp_dir.path().join("sql_files");
        let subdir = sql_dir.join("subdir");
        
        fs::create_dir_all(&subdir).unwrap();
        
        // 创建测试文件
        fs::write(sql_dir.join("test1.sql"), "SELECT 1;").unwrap();
        fs::write(sql_dir.join("test2.sql"), "SELECT 2;").unwrap();
        fs::write(subdir.join("test3.sql"), "SELECT 3;").unwrap();
        fs::write(sql_dir.join("test.txt"), "Not SQL").unwrap();
        
        // 测试扫描功能
        let sql_files = scan_sql_files(&sql_dir);
        
        assert_eq!(sql_files.len(), 3);
        assert!(sql_files.iter().any(|p| p.file_name().unwrap() == "test1.sql"));
        assert!(sql_files.iter().any(|p| p.file_name().unwrap() == "test2.sql"));
        assert!(sql_files.iter().any(|p| p.file_name().unwrap() == "test3.sql"));
    }
    
    #[test]
    fn test_sql_cleaning() {
        let sql_with_comments = r#"-- This is a single line comment
SELECT * FROM users WHERE id = 1; /* This is a
multiline comment */
-- Another single line comment
WITH cte AS (SELECT * FROM orders) SELECT * FROM cte;
"#;
        
        let cleaned_sql = sql_executor::SqlExecutor::clean_sql(sql_with_comments);
        
        // 检查注释是否被正确移除
        assert!(!cleaned_sql.contains("-- This is a single line comment"));
        assert!(!cleaned_sql.contains("/* This is a"));
        assert!(!cleaned_sql.contains("multiline comment */"));
        assert!(!cleaned_sql.contains("-- Another single line comment"));
        
        // 检查SQL语句是否保留完整
        assert!(cleaned_sql.contains("SELECT * FROM users WHERE id = 1;"));
        assert!(cleaned_sql.contains("WITH cte AS (SELECT * FROM orders) SELECT * FROM cte;"));
    }
    
    #[test]
    fn test_file_handler() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let content = "{\"test\": \"value\"}";
        
        let file_handler = file_handler::FileHandler::new();
        
        // 测试写入功能
        file_handler.write_json_atomic(&file_path, content).unwrap();
        
        // 验证文件内容
        let read_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(read_content, content);
        
        // 测试最后修改时间
        let modified_time = file_handler.get_last_modified_time(&file_path).unwrap();
        assert!(modified_time.is_some());
        
        // 测试SQL到JSON路径转换
        let sql_path = Path::new("test.sql");
        let json_path = file_handler.sql_to_json_path(sql_path);
        assert_eq!(json_path.file_name().unwrap(), "test.json");
        
        // 测试should_update功能
        let should_update1 = file_handler.should_update(&sql_path, None).unwrap();
        assert_eq!(should_update1, true);
        
        // 测试带有间隔的should_update
        let should_update2 = file_handler.should_update(&sql_path, Some(60)).unwrap();
        assert_eq!(should_update2, true);
    }
    
    #[test]
    fn test_json_generator() {
        let json_generator = json_generator::JsonGenerator::new("8.0.30".to_string());
        
        // 创建模拟的结果
        let mock_results: Vec<Vec<mysql::Row>> = Vec::new();
        
        // 测试生成JSON
        let json_str = json_generator.generate_json(&mock_results).unwrap();
        
        // 验证JSON格式
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(json_value.get("metadata").is_some());
        assert!(json_value.get("results").is_some());
    }
}
