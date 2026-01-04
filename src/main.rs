use std::sync::Arc;
use tokio::task::JoinSet;
use std::time::Duration;
use clap::Parser;
use log::LevelFilter;

mod config;
mod logger;
mod db;
mod scanner;
mod sql_executor;
mod json_generator;
mod file_handler;
mod tests;

use crate::config::Config;
use crate::db::DbPool;
use crate::scanner::scan_sql_files;
use crate::sql_executor::SqlExecutor;
use crate::json_generator::JsonGenerator;
use crate::file_handler::FileHandler;
use crate::logger::init_logger;

/// FastSQL2Json - Convert SQL results to JSON files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file (default: config.toml in current directory)
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    /// Disable all output
    #[arg(short, long)]
    quiet: bool,
    /// Only output errors
    #[arg(short, long)]
    error_only: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析命令行参数
    let args = Args::parse();
    
    // 设置日志级别
    let log_level = if args.quiet {
        LevelFilter::Off
    } else if args.error_only {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    };
    
    // 初始化日志
    init_logger(log_level)?;
    
    // 读取配置文件
    let config = Config::from_file(&args.config)?;
    log::info!("Loaded configuration from {}", args.config);
    
    // 创建数据库连接池
    let db_pool = DbPool::new(
        &config.database,
        10, // 最大连接数
        Duration::from_secs(30) // 超时时间
    )?;
    log::info!("Connected to MySQL database: {}", config.database.database);
    
    // 获取MySQL版本
    let mysql_version = db_pool.get_version()?;
    log::info!("MySQL version: {}", mysql_version);
    
    // 创建各个组件
    let sql_executor = SqlExecutor::new(db_pool);
    let json_generator = JsonGenerator::new(mysql_version);
    let file_handler = FileHandler::new();
    
    // 扫描SQL文件
    let sql_files = scan_sql_files(&config.app.start_dir);
    log::info!("Found {} SQL files in directory: {}", sql_files.len(), config.app.start_dir);
    
    // 并发处理SQL文件
    let sql_executor = Arc::new(sql_executor);
    let json_generator = Arc::new(json_generator);
    let file_handler = Arc::new(file_handler);
    let config = Arc::new(config);
    
    let mut tasks = JoinSet::new();
    let max_concurrent = 5;
    
    for sql_file in sql_files {
        let sql_executor = sql_executor.clone();
        let json_generator = json_generator.clone();
        let file_handler = file_handler.clone();
        let config = config.clone();
        
        tasks.spawn(async move {
            if let Err(e) = process_sql_file(
                &sql_file,
                &sql_executor,
                &json_generator,
                &file_handler,
                &config
            ).await {
                log::error!("Failed to process file {}: {}", sql_file.display(), e);
            }
        });
        
        // 限制并发任务数
        if tasks.len() >= max_concurrent {
            if let Some(res) = tasks.join_next().await {
                if let Err(e) = res {
                    log::error!("Task failed: {}", e);
                }
            }
        }
    }
    
    // 等待所有任务完成
    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            log::error!("Task failed: {}", e);
        }
    }
    
    log::info!("All SQL files processed successfully");
    Ok(())
}

async fn process_sql_file(
    sql_file: &std::path::Path,
    sql_executor: &Arc<SqlExecutor>,
    json_generator: &Arc<JsonGenerator>,
    file_handler: &Arc<FileHandler>,
    config: &Arc<Config>
) -> Result<(), Box<dyn std::error::Error>> {
    // 获取相对路径，用于配置文件中的间隔设置
    let sql_file_str = sql_file.to_string_lossy().to_string();
    let interval = config.get_interval(&sql_file_str);
    
    // 检查是否需要更新
    if !file_handler.should_update(sql_file, interval)? {
        log::debug!("Skipping file {} (not due for update)", sql_file.display());
        return Ok(());
    }
    
    // 获取文件锁，防止并发处理
    let _lock = file_handler.lock_file(sql_file)?;
    
    // 执行SQL文件
    let results = sql_executor.execute_file(sql_file)?;
    
    // 生成JSON结果
    let json_str = json_generator.generate_json(&results, sql_file)?;
    
    // 原子写入JSON文件
    let json_path = file_handler.sql_to_json_path(sql_file);
    file_handler.write_json_atomic(&json_path, &json_str)?;
    
    log::info!("Generated JSON file: {}", json_path.display());
    Ok(())
}
