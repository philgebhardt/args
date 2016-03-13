use std::ffi::OsStr;
use std::str::FromStr;

use super::{Args,ArgsError,Validation};

/// A trait designed to bind arguments and convenience methods to a struct.
pub trait HasArgs: Send {
    // Instance methods
    /// Returns a mutable references to an `Args` struct.
    fn args(&self) -> &mut Args;

    // Default instance methods
    /// Acts as a convenience method for calling the `Args` implementation.
    fn full_usage(&self, brief: &str) -> String {
        self.args().full_usage(brief)
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn has_value(&self, opt_name: &str) -> bool {
        self.args().has_value(opt_name)
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn parse<C: IntoIterator>(&mut self, raw_args: C) -> Result<(), ArgsError> where C::Item: AsRef<OsStr> {
        self.args().parse(raw_args)
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn parse_from_cli(&mut self) -> Result<(), ArgsError> {
        self.args().parse_from_cli()
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn short_usage(&self) -> String {
        self.args().short_usage()
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn usage(&self, brief: &str) -> String {
        self.args().usage(brief)
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn validated_value_of<T: FromStr>(&self, opt_name: &str, validations: &[Box<Validation<T=T>>]) -> Result<T, ArgsError> {
        self.args().validated_value_of::<T>(opt_name, validations)
    }

    /// Acts as a convenience method for calling the `Args` implementation.
    fn value_of<T: FromStr>(&self, opt_name: &str) -> Result<T, ArgsError> {
        self.args().value_of::<T>(opt_name)
    }
}

