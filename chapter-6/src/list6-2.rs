static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: usize = 42;

    let b: &[u8; 10] = &B;

    let c: Box<[u8]> = Box::new(C);

    println!("a (符号のない整数):");
    println!("{:p}", &a);
    println!("{:?}", size_of::<usize>());
    println!("{:?}", a);
    println!();

    println!("b (Bへの参照)");
    println!("{:p}", &b);
    println!("{:?}", size_of::<&[u8; 10]>());
    println!("{:?}", b);
    println!();

    println!("c (Cを入れたBox)");
    println!("{:p}", &c);
    println!("{:?}", size_of::<Box<[u8]>>());
    println!("{:?}", c);
    println!();

    println!("B (10バイトの配列)");
    println!("{:p}", &B);
    println!("{:?}", size_of::<[u8; 10]>());
    println!("{:?}", B);
    println!();

    println!("C (11バイトの配列)");
    println!("{:p}", &C);
    println!("{:?}", size_of::<[u8; 11]>());
    println!("{:?}", C);
    println!();
}
