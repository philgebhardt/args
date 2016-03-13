use std::error::Error;
use std::fmt;
use std::fmt::{Debug,Display,Formatter};

/// An implementation of `Error` which may or may not include a scope
/// (e.g. arg name, program name, etc.) and/or usage message.
pub struct ArgsError {
    desc: String
}

impl ArgsError {
    /// Creates a new `ArgsError` with the provided `scope` and `msg`.
    /// If `scope` is an empty string (i.e. `""`) it will be ignored.
    pub fn new(scope: &str, msg: &str) -> ArgsError {
        Self::new_with_usage(scope, msg, "")
    }

    /// Creates a new `ArgsError` with the provided `scope`, `msg` and `usage` message.
    /// If either `scope` or `usage` are an empty string (i.e. `""`) they will be ignored.
    pub fn new_with_usage(scope: &str, msg: &str, usage: &str) -> ArgsError {
        // If there is a scope, append it to the front
        let mut desc = if scope.to_string().is_empty() {
            String::new()
        } else {
            format!("{}: ", scope)
        };

        // Append the error message
        desc.push_str(msg);

        // Append the usage message, if it exists
        if !usage.to_string().is_empty() { desc.push_str(&format!("\n\n{}", usage)); }

        ArgsError { desc: desc }
    }
}

impl Debug for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl Error for ArgsError {
    fn description(&self) -> &str {
        &self.desc
    }
}

