#[macro_export]
macro_rules! common_chars {
    ($s1:expr, $($args:expr), *) => {{
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