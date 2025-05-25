use std::io::{self, Write, stdin, stdout, Read};
use colored::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn prompt(message: &str) -> Result<String, io::Error> {
    print!("{} ", message.bright_cyan());
    stdout().flush()?;
    
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}

pub fn confirm(message: &str, default: bool) -> Result<bool, io::Error> {
    let prompt_message = if default {
        format!("{} [Y/n]: ", message)
    } else {
        format!("{} [y/N]: ", message)
    };
    
    let input = prompt(&prompt_message)?;
    
    if input.is_empty() {
        return Ok(default);
    }
    
    match input.to_lowercase().as_str() {
        "y" | "yes" | "はい" => Ok(true),
        "n" | "no" | "いいえ" => Ok(false),
        _ => {
            println!("{} Enter y or n", "⚠️".yellow());
            confirm(message, default)
        }
    }
}

pub fn select_option(message: &str, options: &[&str]) -> Result<usize, io::Error> {
    if options.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "選択肢が存在しません。"));
    }
    
    println!("{}", message.bright_cyan());
    
    for (i, option) in options.iter().enumerate() {
        println!("  {}. {}", (i + 1).to_string().green(), option);
    }
    
    loop {
        let input = prompt("数字をいれてください (1-x): ")?;
        
        if let Ok(num) = input.parse::<usize>() {
            if num >= 1 && num <= options.len() {
                return Ok(num - 1);
            }
        }
        
        println!("{}有効な数字をいれてください。(1-{})", "⚠️ ".yellow(), options.len());
    }
}

// password
pub fn read_password(prompt_message: &str) -> Result<String, io::Error> {
    print!("{} ", prompt_message.bright_cyan());
    stdout().flush()?;
    
    let password = rpassword::read_password()?;
    println!();
    
    Ok(password)
}

pub fn read_multiline(prompt_message: &str) -> Result<String, io::Error> {
    println!("{} (入力が終了したらCtrl+Dを押してください)", prompt_message.bright_cyan());
    println!("{}", "---------- 入力開始 ----------".bright_black());
    
    let mut result = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut result)?;
    
    println!("{}", "---------- 入力終了 ----------".bright_black());
    
    Ok(result)
}

pub fn animated_wait(message: &str, seconds: u64) -> Result<(), io::Error> {
    print!("{}", message);
    stdout().flush()?;
    
    for _ in 0..seconds*2 {
        print!(".");
        stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    
    println!("完了");
    Ok(())
}

// C interface

#[unsafe(no_mangle)]
pub extern "C" fn cli_prompt(message: *const c_char) -> *mut c_char {
    let c_msg = unsafe {
        if message.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(message)
    };
    
    match c_msg.to_str() {
        Ok(msg) => {
            match prompt(msg) {
                Ok(input) => {
                    match CString::new(input) {
                        Ok(c_input) => c_input.into_raw(),
                        Err(_) => std::ptr::null_mut(),
                    }
                },
                Err(_) => std::ptr::null_mut(),
            }
        },
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_confirm(message: *const c_char, default: bool) -> bool {
    let c_msg = unsafe {
        if message.is_null() {
            return default;
        }
        CStr::from_ptr(message)
    };
    
    match c_msg.to_str() {
        Ok(msg) => {
            match confirm(msg, default) {
                Ok(result) => result,
                Err(_) => default,
            }
        },
        Err(_) => default,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_select_option(message: *const c_char, 
                                   options: *const *const c_char,
                                   options_count: usize) -> i32 {
    if message.is_null() || options.is_null() || options_count == 0 {
        return -1;
    }
    
    let c_msg = unsafe { CStr::from_ptr(message) };
    let msg = match c_msg.to_str() {
        Ok(m) => m,
        Err(_) => return -1,
    };
    
    let mut rust_options: Vec<&str> = Vec::with_capacity(options_count);
    
    for i in 0..options_count {
        let opt_ptr = unsafe { *options.offset(i as isize) };
        if opt_ptr.is_null() {
            continue;
        }
        
        let c_opt = unsafe { CStr::from_ptr(opt_ptr) };
        if let Ok(opt) = c_opt.to_str() {
            rust_options.push(opt);
        }
    }
    
    match select_option(msg, &rust_options) {
        Ok(index) => index as i32,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_read_password(prompt_message: *const c_char) -> *mut c_char {
    let c_msg = unsafe {
        if prompt_message.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(prompt_message)
    };
    
    match c_msg.to_str() {
        Ok(msg) => {
            match read_password(msg) {
                Ok(password) => {
                    match CString::new(password) {
                        Ok(c_password) => c_password.into_raw(),
                        Err(_) => std::ptr::null_mut(),
                    }
                },
                Err(_) => std::ptr::null_mut(),
            }
        },
        Err(_) => std::ptr::null_mut(),
    }
}