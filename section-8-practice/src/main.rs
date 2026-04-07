fn main() {
    let mut v = vec![4, 2, 1, 5, 14, 67, 33];
    v.sort();
    println!("{:?}", v);
    let v_len = v.len();
    println!("Length of vector: {v_len}");
    let v_median = v[v_len / 2];

    println!("The median of vector v is: {v_median}");
}
