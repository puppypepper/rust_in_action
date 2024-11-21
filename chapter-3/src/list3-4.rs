#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn main() {
    let f3 = File::new("3.txt");

    let f3_name = &f3.name;
    let f3_length = f3.data.len();

    println!("{}", f3_name);
    println!("{}", f3_length);
}
