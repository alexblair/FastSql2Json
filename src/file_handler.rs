use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use fs2::FileExt;

pub struct FileHandler {
    // 可以添加配置参数，如最大重试次数等
}

impl FileHandler {
    pub fn new() -> Self {
        FileHandler {}
    }
    
    pub fn write_json_atomic<P: AsRef<Path>>(&self, file_path: P, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 创建临时文件，与目标文件在同一目录下
        let temp_file = NamedTempFile::new_in(file_path.parent().unwrap_or_else(|| Path::new(".")))?;
        let temp_path = temp_file.path();
        
        // 写入内容到临时文件
        let mut file = File::create(temp_path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        
        // 验证写入内容的完整性
        let written_content = fs::read_to_string(temp_path)?;
        if written_content != content {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to verify written content"
            )));
        }
        
        // 原子性替换目标文件
        fs::rename(temp_path, file_path)?;
        
        Ok(())
    }
    
    pub fn get_last_modified_time<P: AsRef<Path>>(&self, file_path: P) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        if !file_path.exists() {
            return Ok(None);
        }
        
        let metadata = file_path.metadata()?;
        let modified_time = metadata.modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        Ok(Some(modified_time))
    }
    
    pub fn should_update<P: AsRef<Path>>(&self, file_path: P, interval: Option<u64>) -> Result<bool, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let json_path = self.sql_to_json_path(file_path);
        
        if !json_path.exists() {
            return Ok(true);
        }
        
        if let Some(interval) = interval {
            let last_modified = self.get_last_modified_time(&json_path)?
                .unwrap_or(0);
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            
            Ok(now - last_modified > interval * 60)
        } else {
            Ok(true)
        }
    }
    
    pub fn sql_to_json_path<P: AsRef<Path>>(&self, sql_path: P) -> PathBuf {
        let sql_path = sql_path.as_ref();
        let mut json_path = sql_path.to_path_buf();
        json_path.set_extension("json");
        json_path
    }
    
    pub fn lock_file<P: AsRef<Path>>(&self, file_path: P) -> Result<File, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let lock_path = format!("{}.lock", file_path.display());
        
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&lock_path)?;
        
        // 尝试获取独占锁
        file.try_lock_exclusive()?;
        
        Ok(file)
    }
}
