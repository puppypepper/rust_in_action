fn use_value(_v: i32) {}

fn main() {
    let a = 0;

    use_value(a);

    println!("{}", a);
}
