/// Re-export slog
///
/// Users of this library can, but don't have to use slog to build their own loggers
#[macro_use]
pub extern crate slog;
extern crate slog_stdlog;

use slog::DrainExt;

#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

extern crate reqwest;
use reqwest::Url;

extern crate select;
use select::document::Document;
use select::predicate::Name;

use std::io::Read;

pub struct ADS {
    logger: slog::Logger,
}

/// Test whether a string is a valid ADS bibliographic code
///
/// Requirements documented on the
/// [ADS website](http://doc.adsabs.harvard.edu/abs_doc/help_pages/data.html#bibcodes).
///
/// # Examples
///
/// ```
/// assert!(libads::validate_bib_code("2017arXiv170503937B"))
/// ```
pub fn validate_bib_code(code: &str) -> bool {
    // Use lazy_static to ensure that regexes are compiled only once
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            r"^[[:digit:]]{4}[[:alnum:].]{5}[[:digit:].]{4}[ELPQ-Z[:digit:]]{1}[[:digit:].]{4}[[:alpha:]]{1}$").unwrap();
    }

    REGEX.is_match(code)
}

#[derive(Debug,PartialEq)]
pub struct BibCode<'a> {
    bibcode: &'a str,
}

/// Create BibCode from &str
///
/// Returns a `Result<Self, ()>` as this can fail.
/// In future I may also implement `std::convert::TryFrom`, currently a [nightly only
/// feature](https://github.com/rust-lang/rust/issues/33417).
///
/// # Examples
///
/// ```
/// extern crate libads;
/// libads::BibCode::new("2015MNRAS.452.2597X");
/// ```
impl<'a> BibCode<'a> {
    pub fn new(s: &'a str) -> Result<Self, ()> {
        match validate_bib_code(s) {
            true => Ok(BibCode { bibcode: s }),
            false => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
