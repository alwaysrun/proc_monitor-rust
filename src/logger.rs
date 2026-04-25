use chrono::Local;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write as IoWrite;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub struct Logger {
    log_file: Option<File>,
    to_logging: bool,
}

impl Logger {
    pub fn new(to_log: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let log_file = if to_log {
            if let Ok(exe_path) = env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    let log_path = exe_dir.join("proc_monitor.log");
                    let file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(log_path)?;
                    Some(file)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(Logger {
            log_file,
            to_logging: to_log,
        })
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let level_str = match level {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        };
        let log_message = format!("[{}] [{}] {}\n", timestamp, level_str, message);

        if self.to_logging {
            if let Some(file) = &mut self.log_file {
                if let Err(e) = file.write_all(log_message.as_bytes()) {
                    eprintln!("Failed to write to log file: {}", e);
                }
                let _ = file.flush();
            }
        } else {
            match level {
                LogLevel::Info => println!("{}", &log_message[..log_message.len() - 1]),
                LogLevel::Warning => println!("{}", &log_message[..log_message.len() - 1]),
                LogLevel::Error => eprintln!("{}", &log_message[..log_message.len() - 1]),
            }
        }
    }

    pub fn info(&mut self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&mut self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&mut self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}
