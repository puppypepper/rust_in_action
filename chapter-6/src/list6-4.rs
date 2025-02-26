fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;

    println!("{}, {:p}", a, a_ptr);
}
