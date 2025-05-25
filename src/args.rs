use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::errors::CliError;

#[derive(Debug, Clone)]
pub struct ArgParser {
    program_name: String,
    args: HashMap<String, String>,
    flags: Vec<String>,
    positional: Vec<String>,
    description: String,
}

impl ArgParser {
    pub fn new(program_name: &str) -> Self {
        ArgParser {
            program_name: program_name.to_string(),
            args: HashMap::new(),
            flags: Vec::new(),
            positional: Vec::new(),
            description: String::new(),
        }
    }

    // Add a description to the parser
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    // Arguments
    pub fn parse<I, S>(&mut self,args:I) -> Result<(), CliError>
    where
        I:IntoIterator<Item=S>,
        S: AsRef<str>,
    {
        let mut args_iter = args.into_iter();
        let _ = args_iter.next();

        let mut current_key: Option<String> = None;

        for arg in args_iter {
            let arg = arg.as_ref();

            if arg == "--help" || arg == "-h" {
                self.print_help();
                return Err(crate::errors::unknown_error("show help"));
            }

            if arg.starts_with("--") && arg.contains('=') {
                let parts: Vec<&str> = arg.splitn(2, '=').collect();
                let key = parts[0].trim_start_matches("--").to_string();
                let value = parts[1].to_string();
                self.args.insert(key, value);
                continue;
            }

            if arg.starts_with("--") {
                let key = arg.trim_start_matches("--").to_string();
                if let Some(prev_key) = current_key.take() {
                    self.flags.push(prev_key);
                }
                current_key = Some(key);
            } else if arg.starts_with('-') && arg.len() > 1 {
                let key = arg.trim_start_matches('-').to_string();
                if let Some(prev_key) = current_key.take() {
                    self.flags.push(prev_key);
                }
                current_key = Some(key);
            } else if let Some(key) = current_key.take() {
                self.args.insert(key, arg.to_string());
            } else {
                self.positional.push(arg.to_string());
            }
        }

        if let Some(key) = current_key {
            self.flags.push(key);
        }

        Ok(())  
    }

    pub fn print_help(&self) {
        println!("-- {} --", self.program_name);
        if !self.description.is_empty() {
            println!("{}", self.description);
        }
        println!("\n使い方:");
        println!("  {} [オプション] [引数...]", self.program_name);
        println!("\nオプション:");
        println!("  -h, --help    このヘルプメッセージを表示して終了");
    }
    // get args
    pub fn get(&self, key: &str) -> Option<&String> {
        self.args.get(key)
    }

    // has flag
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(&flag.to_string())
    }

    // get positional
    pub fn get_positional(&self, index: usize) -> Option<&String> {
        self.positional.get(index)
    }

    pub fn positional_count(&self) -> usize {
        self.positional.len()
    }

    pub fn get_all_positional(&self) -> &[String] {
        &self.positional
    }
}

// C interface
#[unsafe(no_mangle)]
pub extern "C" fn cli_create_arg_parser(program_name: *const c_char) -> *mut ArgParser {
    let c_name = unsafe {
        if program_name.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(program_name)
    };
    
    match c_name.to_str() {
        Ok(name) => {
            let parser = ArgParser::new(name);
            Box::into_raw(Box::new(parser))
        },
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_set_parser_description(parser: *mut ArgParser, description: *const c_char) {
    if parser.is_null() || description.is_null() {
        return;
    }
    
    let c_desc = unsafe { CStr::from_ptr(description) };
    
    if let Ok(desc) = c_desc.to_str() {
        unsafe {
            (*parser).description = desc.to_string();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_parse_args(parser: *mut ArgParser, argc: i32, argv: *const *const c_char) -> bool {
    if parser.is_null() || argv.is_null() || argc <= 0 {
        return false;
    }
    
    let mut args: Vec<String> = Vec::new();
    
    for i in 0..argc {
        let arg_ptr = unsafe { *argv.offset(i as isize) };
        if arg_ptr.is_null() {
            continue;
        }
        
        let c_arg = unsafe { CStr::from_ptr(arg_ptr) };
        if let Ok(arg) = c_arg.to_str() {
            args.push(arg.to_string());
        }
    }
    
    let result = unsafe {
        (*parser).parse(args)
    };
    
    result.is_ok()
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_arg_parser_get(parser: *const ArgParser, key: *const c_char) -> *mut c_char {
    if parser.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }
    
    let c_key = unsafe { CStr::from_ptr(key) };
    if let Ok(key_str) = c_key.to_str() {
        let value = unsafe { (*parser).get(key_str) };
        
        if let Some(val) = value {
            if let Ok(c_value) = CString::new(val.clone()) {
                return c_value.into_raw();
            }
        }
    }
    
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_arg_parser_has_flag(parser: *const ArgParser, flag: *const c_char) -> bool {
    if parser.is_null() || flag.is_null() {
        return false;
    }
    
    let c_flag = unsafe { CStr::from_ptr(flag) };
    if let Ok(flag_str) = c_flag.to_str() {
        unsafe { (*parser).has_flag(flag_str) }
    } else {
        false
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_arg_parser_print_help(parser: *const ArgParser) {
    if !parser.is_null() {
        unsafe {
            (*parser).print_help();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cli_arg_parser_free(parser: *mut ArgParser) {
    if !parser.is_null() {
        unsafe {
            let _ = Box::from_raw(parser);
        }
    }
}