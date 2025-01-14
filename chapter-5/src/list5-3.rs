fn main() {
    let sixty5_535: u16 = 0b1111_1111_1111_1111;
    println!("65535: {:016b} {}", sixty5_535, sixty5_535);

    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            print!("\n")
        }
    }
}
