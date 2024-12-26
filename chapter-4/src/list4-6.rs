fn use_value(_v: Demo) {}

struct Demo {
    value: i32,
}

fn main() {
    let a = Demo { value: 0 };

    use_value(a);

    // println!("{}", a);
}
