pub mod logger;
pub mod templates;
pub mod errors;
pub mod config;
pub mod progress;
pub mod args;
pub mod interactive;

pub const VERSION: &str = "0.1.0";
pub const NAME: &str = "cli_core";

pub use logger::{log_info, log_warn, log_error, log_success, log_debug};
pub use templates::{get_template, add_template, remove_template};
pub use errors::{CliError, config_error, auth_error, network_error, unknown_error};
pub use config::Config;
pub use progress::{create_progress_bar, update_progress, finish_progress};

// C FFI
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn cli_log_info(message: *const c_char) {
    let c_str = unsafe {
        if message.is_null() {
            return;
        }
        CStr::from_ptr(message)
    };
    
    if let Ok(message_str) = c_str.to_str() {
        log_info(message_str);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_log_warn(message: *const c_char) {
    let c_str = unsafe {
        if message.is_null() {
            return;
        }
        CStr::from_ptr(message)
    };
    
    if let Ok(message_str) = c_str.to_str() {
        log_warn(message_str);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_log_error(message: *const c_char) {
    let c_str = unsafe {
        if message.is_null() {
            return;
        }
        CStr::from_ptr(message)
    };
    
    if let Ok(message_str) = c_str.to_str() {
        log_error(message_str);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_log_success(message: *const c_char) {
    let c_str = unsafe {
        if message.is_null() {
            return;
        }
        CStr::from_ptr(message)
    };
    
    if let Ok(message_str) = c_str.to_str() {
        log_success(message_str);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_get_template(key: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        if key.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(key)
    };
    
    match c_str.to_str() {
        Ok(key_str) => {
            if let Some(template) = get_template(key_str) {
                match CString::new(template) {
                    Ok(c_template) => c_template.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            } else {
                std::ptr::null_mut()
            }
        },
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_free_string(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_load_config(path: *const c_char) -> bool {
    let c_str = unsafe {
        if path.is_null() {
            return false;
        }
        CStr::from_ptr(path)
    };
    
    match c_str.to_str() {
        Ok(path_str) => {
            match config::Config::load(path_str) {
                Ok(_loaded_config) => {
                    true
                },
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_create_progress_bar(total: u64) -> usize {
    progress::create_progress_bar(total)
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_update_progress(id: usize, current: u64, message: *const c_char) -> bool {
    let message_str = if message.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(message) }.to_str() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    };

    progress::update_progress(id, current, message_str)
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_finish_progress(id: usize, message: *const c_char) -> bool {
    let message_str = if message.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(message) }.to_str() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    };

    progress::finish_progress(id, message_str)
}