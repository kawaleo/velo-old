#![allow(dead_code)]
pub const ERROR_INDICATOR: &str = "\x1b[1m[\x1b[0m\x1b[1;31merror\x1b[0m\x1b[1m]:\x1b[0m";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VeloError {
    pub line: usize,
    pub message: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    ParseError,
    RuntimeError,
}

impl VeloError {
    pub fn error(line: usize, message: &str, error_type: ErrorType) -> Self {
        Self {
            line,
            message: message.to_string(),
            error_type,
        }
    }
}
