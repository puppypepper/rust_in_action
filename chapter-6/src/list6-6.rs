fn main() {
    let a = 42;
    let b = Box::new(36);

    println!("{} + {} = {}", a, *b, a + *b);
}
