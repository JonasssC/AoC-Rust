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

pub fn regex_parse(s: &str, regex_s: &str) -> Vec<String> {
    let re = Regex::new(regex_s).expect("Invalid RegEx");
    let caps = re.captures(s).unwrap();

    let mut res: Vec<String> = Vec::new();

    for i in 1..caps.len() {
        res.push(caps.get(i).unwrap().as_str().to_string());
    }

    return res;
}

pub fn regex_parse_as_i32(s: &str, regex_s: &str) -> Vec<i32> {
    return regex_parse(s, regex_s).iter()
        .map(| m | m.parse::<i32>().unwrap())
        .collect();
}