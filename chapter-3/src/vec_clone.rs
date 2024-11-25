fn main() {
    let mut v1 = vec![1, 2, 3];
    let v2 = v1.clone();

    v1.push(4);

    println!("{:?}", v1);
    println!("{:?}", v2);
}
