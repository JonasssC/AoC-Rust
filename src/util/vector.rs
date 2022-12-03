pub fn group_by_n<T: Clone>(v: Vec<T>, n: usize) -> Vec<Vec<T>> {
    if v.len() % n != 0 {
        panic!("Not groupable in groups of {}", n);
    }

    let mut res: Vec<Vec<T>> = Vec::new();

    for i in 0..v.len()/n {
        res.push(v[i*n..i*n+n].to_vec())
    }

    return res;
}