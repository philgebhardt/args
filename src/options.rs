use getopts::{HasArg,Matches,Occur,Options};
use std::fmt::{self,Debug,Display,Error,Formatter};

macro_rules! unsupported {
    ( $str:expr ) => ( panic!("{} is not supported yet", $str) );
}

pub struct Opt {
    short_name: String,
    long_name: String,
    desc: String,
    hint: String,
    has_arg: HasArg,
    occur: Occur,
    default: Option<String>
}

impl Opt {
    pub fn new(short_name: &str,
            long_name: &str,
            desc: &str,
            hint: &str,
            has_arg: HasArg,
            occur: Occur,
            default: Option<String>) -> Opt {
        if has_arg == HasArg::Maybe { unsupported!("HasArg::Maybe"); }
        if occur == Occur::Multi { unsupported!("Occur::Multi"); }

        // If there is a default occurence becomes optional
        let occur = if default.is_some() { Occur::Optional } else { occur };
        Opt {
            short_name: short_name.to_string(),
            long_name: long_name.to_string(),
            desc: desc.to_string(),
            hint: hint.to_string(),
            has_arg: has_arg,
            occur: occur,
            default: default
        }
    }

    pub fn is_required(&self) -> bool {
        self.occur == Occur::Req
    }

    pub fn name(&self) -> &String {
        &self.long_name
    }

    pub fn parse(&self, matches: &Matches) -> Option<String> {
        // If the option does not have an argument, return presence
        if self.has_arg == HasArg::No {
            return Some(matches.opt_present(&self.long_name).to_string());
        }

        // If the option does have an arugment, parse it or get the default
        matches.opt_str(&self.long_name).or_else(|| {
            // Return the default if it is defined and there is no match
            if self.default.is_some() { return self.default.clone(); }
            None
        })
    }

    pub fn register_option(&self, options: &mut Options) {
        options.opt(&self.short_name,
            &self.long_name,
            &self.desc,
            &self.hint,
            self.has_arg,
            self.occur);
    }
}

impl Display for Opt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "option '-{} --{}'", self.short_name, self.long_name)
    }
}

impl Debug for Opt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "option '-{} --{}'", self.short_name, self.long_name)
    }
}

