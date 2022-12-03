#[macro_export]
macro_rules! common_chars {
    ($s1:expr, $($args:expr), *) => {{
        let mut res: String = $s1.to_string();
        $(
            let mut common: String = String::from("");
            for c1 in res.chars() {
                for c2 in $args.chars() {
                    if c1 == c2 && !common.contains(c1) {
                        common.push(c1);
                    }
                }
            }
            res = common;
        )*
        res
    }}
}