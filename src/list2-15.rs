fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let a = 10;
    let b = 20;

    let res = add(a, b);
    println!("{}", res);
}
