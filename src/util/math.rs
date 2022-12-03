pub fn get_max_n<T: Clone + Ord>(elements: Vec<T>, n: usize) -> Vec<T> {
    let mut clone = elements.clone();
    clone.sort();
    clone.reverse();
    clone.truncate(n);
    clone
}