use std::ffi::OsStr;

use super::super::{Args,ArgsError};

/// A trait designed to bind associated convenience methods to a struct.
pub trait HasArgs: Send {
    // Associated methods
    /// Returns a mutable references to an associated `Args` struct.
    fn args() -> Args where Self: Sized;

    // Defaulted associated methods
    /// Generates a combination of the short and verbose usage messages.
    fn full_usage() -> String where Self: Sized {
        format!("{}\n\n{}", Self::short_usage(), Self::usage())
    }

    /// Acts as a convenience method for calling the associated `Args` implementation.
    fn parse<C: IntoIterator>(raw_args: C) -> Result<(), ArgsError> where C::Item: AsRef<OsStr>, Self: Sized {
        Self::args().parse(raw_args)
    }

    /// Acts as a convenience method for calling the associated `Args` implementation.
    fn parse_from_cli() -> Result<(), ArgsError> where Self: Sized {
        Self::args().parse_from_cli()
    }

    /// Acts as a convenience method for calling the associated `Args` implementation.
    fn short_usage() -> String where Self: Sized {
        Self::args().short_usage()
    }

    /// Acts as a convenience method for calling the associated `Args` implementation.
    fn usage() -> String where Self: Sized {
        Self::args().usage()
    }
}

