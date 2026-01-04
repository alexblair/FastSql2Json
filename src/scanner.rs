use walkdir::{WalkDir, DirEntry};
use std::path::Path;

pub fn scan_sql_files<P: AsRef<Path>>(start_dir: P) -> Vec<std::path::PathBuf> {
    let mut sql_files = Vec::new();
    
    for entry in WalkDir::new(start_dir).into_iter().filter_map(|e| e.ok()) {
        if is_sql_file(&entry) {
            sql_files.push(entry.path().to_path_buf());
        }
    }
    
    sql_files
}

fn is_sql_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && 
    entry.path().extension().map_or(false, |ext| ext == "sql")
}
