#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

/// Test whether a string is a valid ADS bibliographic code
///
/// Requirements documented on the
/// [ADS website](http://doc.adsabs.harvard.edu/abs_doc/help_pages/data.html#bibcodes).
///
/// # Examples
///
/// ```
/// assert!(ads::validate_bibliographic_code("2017arXiv170503937B"))
/// ```
pub fn validate_bibliographic_code(code: &str) -> bool {
    // Use lazy_static to ensure that regexes are compiled only once
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            r"^[[:digit:]]{4}[[:alnum:].]{5}[[:digit:].]{4}[ELPQ-Z[:digit:]]{1}[[:digit:].]{4}[[:alpha:]]{1}$").unwrap();
    }

    REGEX.is_match(code)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
