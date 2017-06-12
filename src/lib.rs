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

use std::io::Read;

pub struct Api {
    logger: slog::Logger,
}

impl Api {
    /// Initialize API
    ///
    /// Either provide a custom slog::Logger or default to the standard `log`
    /// crate.
    ///
    /// # Examples
    /// ```
    /// libads::Api::init(None);
    /// ```
    pub fn init(logger: Option<slog::Logger>) -> Self {
        Api {
            logger: logger.unwrap_or_else(|| slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    /// Fetch BibTeX entries from ADS
    ///
    /// # Examples
    ///
    /// ```
    /// let ads = libads::Api::init(None);
    ///
    /// println!(
    ///     "{}",
    ///     ads.fetch_bibtex_with_key(
    ///         libads::BibCode::new("2015MNRAS.452.2597X").expect("Good bibcode")
    ///     ).expect("ADS record exists")
    /// );
    /// ```
    pub fn fetch_bibtex_with_key(&self, key: BibCode) -> Option<String> {

        let mut api_url: Url = Url::parse("http://adsabs.harvard.edu")
            .expect("Unable to parse API URL")
            .join("cgi-bin/")
            .expect("Static and parseable")
            .join("nph-bib_query/")
            .expect("Static and parseable");
        api_url
            .query_pairs_mut()
            .append_pair("data_type", "BIBTEX")
            .append_pair("bibcode", &key.bibcode);

        debug!(self.logger, "Querying ADS API";
               "URL" => api_url.to_string());
        let mut response = reqwest::get(api_url).expect("Failed to send get request");
        debug!(self.logger, "GET request completed";
               "HTTP response status" => response.status().to_string());

        let mut data = String::new();
        response
            .read_to_string(&mut data)
            .expect("Failed to read response.");

        if let Some(entry) = data.split("\n@").nth(1) {
            Some(entry.to_string())
        } else {
            None
        }

    }
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
///
/// ```
/// assert!(libads::validate_bib_code("2015MNRAS.452.2597X"))
/// ```
pub fn validate_bib_code(code: &str) -> bool {
    // Use lazy_static to ensure that regexes are compiled only once
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            r"^[[:digit:]]{4}[[:alnum:]\.]{5}[[:digit:]\.]{4}[ELPQ-Z[:digit:]\.]{1}[[:digit:]\.]{4}[[:alpha:]]{1}$").unwrap();
    }

    REGEX.is_match(code)
}

#[derive(Debug,PartialEq)]
pub struct BibCode<'a> {
    pub bibcode: &'a str,
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
