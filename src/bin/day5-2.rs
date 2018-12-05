extern crate failure;
extern crate regex;

use std::io;
use std::io::prelude::*;

use failure::Error;

use regex::Regex;

fn main() -> Result<(), Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    input = input.trim().into();
    let min = (b'a'..=b'z')
        .map(|b| (b as char).to_string())
        .map(|c| (c.to_uppercase(), c))
        .map(|(s1, s2)| input.replace(&s1, "").replace(&s2, ""))
        .map(|s| react(&s).len())
        .min();

    println!("[part2] Minimum reaction is {} units", min.unwrap());
    Ok(())
}

fn react(s: &str) -> String {
    let pattern = Regex::new("(\
aA|Aa|bB|Bb|cC|Cc|dD|Dd|eE|Ee|fF|Ff|gG|Gg|hH|Hh|iI|Ii|jJ|Jj|kK|Kk|lL|Ll|mM|Mm|nN|Nn|oO|Oo|pP|Pp|\
qQ|Qq|rR|Rr|sS|Ss|tT|Tt|uU|Uu|vV|Vv|wW|Ww|xX|Xx|yY|Yy|zZ|Zz\
)").unwrap();

    let mut previous = String::new();
    let mut current = s.to_string();

    while previous != current {
        previous = current;
        current = pattern.replace(&previous, "").into();
    }

    current
}
