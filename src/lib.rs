pub mod io {
    use std::fs::read_to_string;

    pub fn read_string(year: i32, day: i32) -> String {
        let path = format!("src/y{}/d{:0>2}/input.txt", year, day);
        return read_to_string(path)
            .expect("Couldn't read file")
            .trim().to_string();
    }

    pub fn read_lines(year: i32, day: i32) -> Vec<String> {
        return read_string(year, day)
            .split(&['\n', '\r'])
            .map(| l | l.trim().to_string())
            .collect();
    }

    pub fn read_lines_as_i32(year: i32, day: i32) -> Vec<i32> {
        return read_lines(year, day)
            .into_iter()
            .map(| l | l.parse::<i32>().unwrap())
            .collect()
    }
}