// Copyright 2016 Matthew Fornaciari <mattforni@gmail.com>
//!
//! A dead simple implementation of command line argument parsing and validation
//! built on top of the [getopts](https://crates.io/crates/getopts) crate.
//!
//! In order to use the `args` crate simply create an `Args` object and begin
//! registering possible command line options via the `flag(...)` and `option(...)`
//! methods. Once all options have been registered, parse arguments directly from the
//! command line, or provide a vector of your own arguments.
//!
//! If any errors are encountered during parsing the method will panic, otherwise,
//! arguments can be retrieved from the `args` instance by calling `value_of(...)`
//! or `validated_value_of(...)`.
//!
//! That's it!
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/args) and can be
//! used by adding `args` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! args = "0.1"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate args;
//! ```
//!
//! # Example
//!
//! The following example shows simple command line parsing for an application that
//! requires a number of iterations between zero *(0)* and ten *(10)* to be specified,
//! accepts an optional log file name and responds to the help flag.
//!
//! ```{.rust}
//! extern crate args;
//! extern crate getopts;
//!
//! use args::{Args,Order,OrderValidation};
//! use getopts::Occur;
//!
//! const PROGRAM_NAME: &'static str = "program";
//!
//! fn main() {
//!     let mut args = Args::new(PROGRAM_NAME);
//!     args.flag("h", "help", "Print the usage menu");
//!     args.option("i",
//!         "iter",
//!         "The number of times to run this program",
//!         "TIMES",
//!         Occur::Req,
//!         None);
//!     args.option("l",
//!         "log_file",
//!         "The name of the log file",
//!         "NAME",
//!         Occur::Optional,
//!         None);
//!
//!     args.parse(vec!("-i", "15"));
//!
//!     match args.value_of("help") {
//!         Ok(help) => {
//!             if help {
//!                 args.full_usage(&format!("How to use {}", PROGRAM_NAME));
//!                 return;
//!             }
//!         },
//!         Err(error) => {
//!             println!("{}", error);
//!             return;
//!         }
//!     }
//!
//!     let gt_0 = Box::new(OrderValidation::new(Order::GreaterThan, 0u32));
//!     let lt_10 = Box::new(OrderValidation::new(Order::LessThanOrEqual, 10u32));

//!     match args.validated_value_of("iter", &[gt_0, lt_10]) {
//!         Ok(iterations) => {
//!             for _ in 0..iterations {
//!                 println!("Doing work ...");
//!             }
//!
//!             println!("All done.");
//!         },
//!         Err(error) => {
//!             println!("{}", error.to_string());
//!         }
//!     }
//! }
//! ```
//!

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://doc.rust-lang.org/")]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

#[macro_use] extern crate log;
extern crate getopts;

use getopts::{Fail,HasArg,Occur,Options};
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::iter::IntoIterator;
use std::str::FromStr;

pub use self::errors::ArgsError;
pub use self::validations::{Order,Validation,OrderValidation};
pub use self::traits::HasArgs;

use self::options::Opt;

const SCOPE_PARSE: &'static str = "parse";

mod errors;
mod options;
mod traits;
mod validations;

#[cfg(test)] mod tst;

/// A dead simple implementation of command line argument parsing and validation.
pub struct Args {
    options: Options,
    opts: HashMap<String, Opt>,
    opt_names: Vec<String>,
    program_name: String,
    values: HashMap<String, String>
}

impl Args {
    // Public associated methods
    /// Creates an empty set of command line options.
    pub fn new(program_name: &str) -> Args {
        debug!("Creating new args object for '{}'", program_name);

        Args {
            options: Options::new(),
            opts: HashMap::new(),
            opt_names: Vec::new(),
            program_name: program_name.to_string(),
            values: HashMap::new()
        }
    }

    // Public instance methods
    /// Registers an optional flag argument that does not take an argument and defaults to false.
    ///
    /// * `short_name` - e.g. `"h"` for a `-h` option, or `""` for none
    /// * `long_name` - e.g. `"help"` for a `--help` option, or `""` for none
    /// * `desc` - A description of the flag for the usage message
    pub fn flag(&mut self,
            short_name: &str,
            long_name: &str,
            desc: &str) -> &mut Args {
        self.register_opt(
            Opt::new(short_name,
                long_name,
                desc,
                "",
                HasArg::No,
                Occur::Optional,
                None
            )
        );

        self
    }

    /// Generates a combination of the short and verbose usage messages.
    pub fn full_usage(&self, brief: &str) -> String {
        format!("{}\n\n{}", self.short_usage(), self.usage(brief))
    }

    /// Returns a `bool` indicating whether or not a argument is present.
    pub fn has_value(&self, opt_name: &str) -> bool {
        self.values.get(opt_name).is_some()
    }

    /// Registers an option explicitly.
    ///
    /// * `short_name` - e.g. `"h"` for a `-h` option, or `""` for none
    /// * `long_name` - e.g. `"help"` for a `--help` option, or `""` for none
    /// * `desc` - A description of the flag for the usage message
    /// * `hint` - A hint to be used in place of the argument in the usage message,
    /// e.g. `"FILE"` for a `-o FILE` option
    /// * `occur` - An enum representing whether the option is required or not
    /// * `default` - The default value for this option if there should be one
    pub fn option(&mut self,
            short_name: &str,
            long_name: &str,
            desc: &str,
            hint: &str,
            occur: Occur,
            default: Option<String>) -> &mut Args {
        self.register_opt(
            Opt::new(short_name,
                long_name,
                desc,
                hint,
                HasArg::Yes,
                occur,
                default
            )
        );

        self
    }

    /// Parses arguments according to the registered options.
    ///
    /// # Failures
    /// Fails if any errors are encountered during parsing.
    pub fn parse<C: IntoIterator>(&mut self, raw_args: C) -> Result<(), ArgsError> where C::Item: AsRef<OsStr> {
        debug!("Parsing args for '{}'", self.program_name);

        // Get matches and return an error if there is a problem parsing
        let matches = match self.options.parse(raw_args) {
            Ok(matches) => { matches },
            Err(error) => { return Err(ArgsError::new(SCOPE_PARSE, &error.to_string())) }
        };

        // Find matches and store the values (or a default)
        for opt_name in &self.opt_names {
            let option = self.opts.get(opt_name);
            if option.is_none() {
                return Err(ArgsError::new(SCOPE_PARSE, &Fail::UnrecognizedOption(opt_name.to_string()).to_string()));
            }

            let opt = option.unwrap();
            let value = opt.parse(&matches).unwrap_or("".to_string());
            if !value.is_empty() {
                self.values.insert(opt_name.to_string(), value);
            } else {
                if opt.is_required() {
                    return Err(ArgsError::new(SCOPE_PARSE, &Fail::ArgumentMissing(opt_name.to_string()).to_string()));
                }
            }
        }

        debug!("Args: {:?}", self.values);
        Ok(())
    }

    /// Parses arguments directly from the command line according to the registered options.
    ///
    /// # Failures
    /// Fails if any errors are encountered during parsing.
    pub fn parse_from_cli(&mut self) -> Result<(), ArgsError> {
        // Retrieve the cli args and throw out the program name
        let mut raw_args: Vec<String> = env::args().collect();
        if !raw_args.is_empty() { raw_args.remove(0); }

        self.parse(&mut raw_args)
    }

    /// Generates a one-line usage summary from the registered options.
    pub fn short_usage(&self) -> String {
        self.options.short_usage(&self.program_name)
    }

    /// Generates a verbose usage summary from the registered options.
    pub fn usage(&self, brief: &str) -> String {
        self.options.usage(brief)
    }

    /// Retrieves the value of the `Opt` identified by `opt_name`, casts it to
    /// the type specified by `T` and then runs all provided `Validation`s.
    ///
    /// # Failures
    ///
    /// Returns `Err(ArgsError)` if no `Opt` correspond to `opt_name`, if the value cannot
    /// be cast to type `T` or if any validation is considered invalid.
    pub fn validated_value_of<T: FromStr>(
            &self, opt_name: &str, validations: &[Box<Validation<T=T>>]) -> Result<T, ArgsError> {
        // If the value does not have an error, run validations
        self.value_of::<T>(opt_name).and_then(|value| {
            for validation in validations {
                // If any validations fail, break the loop and return the error
                if validation.is_invalid(&value) { return Err(validation.error(&value)); }
            }

            Ok(value)
        })
    }

    /// Retrieves the value of the `Opt` identified by `opt_name` and casts it to
    /// the type specified by `T`.
    ///
    /// # Failures
    ///
    /// Returns `Err(ArgsError)` if no `Opt` corresponds to `opt_name` or if the
    /// value cannot be cast to type `T`.
    pub fn value_of<T: FromStr>(&self, opt_name: &str) -> Result<T, ArgsError> {
        self.values.get(opt_name).ok_or(
            ArgsError::new(opt_name, "does not have a value")
        ).and_then(|value_string| {
            T::from_str(value_string).or(
                Err(ArgsError::new(opt_name, &format!("unable to parse '{}'", value_string)))
            )
        })
    }

    // Private instance methods
    fn register_opt(&mut self, opt: Opt) {
        debug!("Registering {}", opt);
        opt.register_option(&mut self.options);
        self.opt_names.push(opt.name().to_string());
        self.opts.insert(opt.name().to_string(), opt);
    }
}


