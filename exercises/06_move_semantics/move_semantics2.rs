fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    // let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
    // vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let mut vec0 = Vec::new(); 
    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {} and content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} and content `{:?}`", "vec1", vec1.len(), vec1);
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66];

        let mut vec1 = fill_vec($mut vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
