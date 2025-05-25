use chrono::Local;
use colored::*;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Success,
    Warning,
    Error,
}

fn log(level: LogLevel, message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let prefix = match level {
        LogLevel::Debug => "[DEBUG]".bright_black(),
        LogLevel::Info => "[INFO]".bright_blue(),
        LogLevel::Success => "[SUCCESS]".bright_green(),
        LogLevel::Warning => "[WARN]".bright_yellow(),
        LogLevel::Error => "[ERROR]".bright_red(),
    };
    
    let log_message = format!("{} {} {}", timestamp.bright_black(), prefix, message);
    
    let mut stdout = io::stdout();
    let _ = writeln!(stdout, "{}", log_message);
    let _ = stdout.flush();
}

pub fn log_info(message: &str) {
    log(LogLevel::Info, message);
}

pub fn log_success(message: &str) {
    log(LogLevel::Success, message);
}

pub fn log_warn(message: &str) {
    log(LogLevel::Warning, message);
}

pub fn log_error(message: &str) {
    log(LogLevel::Error, message);
}

pub fn log_debug(message: &str) {
    log(LogLevel::Debug, message);
}