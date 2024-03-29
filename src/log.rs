use std::process::exit;

/// Print a successfull message
pub fn success<T: ToString>(text: T) {
    eprintln!("[\x1B[92m+\x1B[0m] {}", text.to_string());
}

/// Print an error message, and *exit* the program
pub fn error<T: ToString>(text: T) {
    eprintln!("[\x1B[91mFAIL\x1B[0m] {}", text.to_string());
    exit(1);
}

/// Print a warning message
pub fn warn<T: ToString>(text: T) {
    eprintln!("[\x1B[93m!\x1B[0m] {}", text.to_string());
}

/// Print an informational message
pub fn info<T: ToString>(text: T) {
    eprintln!("[\x1B[94m*\x1B[0m] {}", text.to_string());
}

/// Print a progress message
pub fn progress<T: ToString>(text: T) {
    eprintln!("[\x1B[96m~\x1B[0m] {}", text.to_string());
}

/// Print a debug message
pub fn debug<T: ToString>(text: T) {
    eprintln!("[\x1B[90mDEBUG\x1B[0m] {}", text.to_string());
}
