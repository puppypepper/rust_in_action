fn main() {
    println!("Hello, world!");

    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("{}", e);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
