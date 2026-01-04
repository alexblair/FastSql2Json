use log::LevelFilter;
use std::io::Write;
use std::time::SystemTime;

pub fn init_logger(level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .format(|buf, record| {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            writeln!(buf, "[{}] [{}] - {}", timestamp, record.level(), record.args())
        })
        .filter(None, level)
        .init();
    Ok(())
}

macro_rules! log_error {
    ($file_path:expr, $error:expr) => {
        error!("Error in file {}: {}", $file_path, $error);
    };
    ($file_path:expr, $error_type:expr, $details:expr) => {
        error!("Error in file {} - {}: {}", $file_path, $error_type, $details);
    };
}

pub(crate) use log_error;
