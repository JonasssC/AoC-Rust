use std::fmt::Debug;
use std::str::FromStr;
use regex::Regex;

#[macro_export]
macro_rules! common_chars {
    ($s1:expr, $($args:expr), +) => {{
        let mut res: Vec<char> = $s1.chars().collect();
        $(
            let mut common: Vec<char> = Vec::new();
            for c1 in res.iter() {
                if $args.contains(*c1) && !common.contains(c1) {
                    common.push(*c1);
                }
            }
            res = common;
        )*
        res
    }}
}

pub fn regex_parse<T: FromStr>(s: &str, regex_s: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    let re = Regex::new(regex_s).expect("Invalid RegEx");
    let caps = re.captures(s).unwrap();

    let mut res: Vec<T> = Vec::new();

    for i in 1..caps.len() {
        res.push(caps.get(i).unwrap().as_str().parse::<T>().unwrap());
    }

    return res;
}