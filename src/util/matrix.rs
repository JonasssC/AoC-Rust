pub fn is_in_field(height: usize, width: usize, row: i32, col: i32) -> bool {
    0 <= row && row < height as i32
        && 0 <= col && col < width as i32
}