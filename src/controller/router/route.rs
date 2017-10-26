use regex::Regex;
use url::percent_encoding::percent_decode;

use std::collections::HashMap;

lazy_static! {
    static ref REGEX_ROUTES: HashMap<&'static str, Regex> = hashmap! {
        "/db/{name}" => Regex::new("/db/[^/;\'\"`]+/?$").unwrap()
    };
}

pub trait Route
{
    fn is_match(&self, pattern: &str) -> bool;
}

impl Route for String
{
    fn is_match(&self, route: &str) -> bool
    {
        if let Some(regex) = REGEX_ROUTES.get(route) {
            if let Ok(decoded) = percent_decode(self.as_ref()).decode_utf8() {
                regex.is_match(decoded.as_ref())
            } else {
                println!("Can't decode uri: {}", self);
                false
            }
        } else {
            self == route
        }
    }
}


#[cfg(test)]
mod tests
{
    use super::Route;
    use url::percent_encoding::{percent_decode, percent_encode, USERINFO_ENCODE_SET};

    fn encode(string: &str) -> String
    {
        percent_encode(string.as_ref(), USERINFO_ENCODE_SET).to_string()
    }

    fn decode(string: &str) -> String
    {
        percent_decode(string.as_ref()).decode_utf8().unwrap().to_string()
    }

    #[test]
    fn is_match_db_name()
    {
        let samples = hashmap! {
            encode("/db/foo") => true,
            encode("/db/foo/") => true,
            encode("/db/foo_bar") => true,
            encode("/db/foo_bar/") => true,
            encode("/db/foo//") => false,
            encode("/db/foo/bar/") => false,
            encode("/db/foo;") => false,
            encode("/db/foo;bar") => false,
            encode("/db/foo\"") => false,
            encode("/db/foo\"bar") => false,
            encode("/db/foo\'") => false,
            encode("/db/foo\'bar") => false,
            encode("/db/foo`") => false,
            encode("/db/foo`bar") => false,
        };
        for (uri, is_match) in samples {
            assert_eq!(uri.is_match("/db/{name}"), is_match,
                       "uri: {}, decoded: {}, is_match: {}", uri, decode(&uri), is_match);
        }
    }
}