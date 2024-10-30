fn add_with_lifetimes<'a, 'b>(a: &'a i32, b: &'b i32) -> i32 {
    *a + *b
}

fn main() {
    let a = 10;
    let b = 20;

    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);
}
