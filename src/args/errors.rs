use std::error::Error;
use std::fmt;
use std::fmt::{Display,Formatter};

#[derive(Debug)]
pub struct ArgsError {
    pub desc: String
}

impl ArgsError {
    pub fn new(scope: &str, msg: &str) -> ArgsError {
        Self::new_with_usage(scope, msg, "")
    }

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

