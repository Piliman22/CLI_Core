// example
use cli_core::{log_info, log_error, log_success, get_template};

fn main() {
    log_info("Starting application...");
    
    if let Some(msg) = get_template("not_found") {
        log_error(&msg);
    }
    
    log_success("success");
}