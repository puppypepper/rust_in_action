use rand::random;

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

fn main() {
    let f = File;
    let mut buffer = Vec::new();

    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            println!("An error has occurred!");
        }
    }
}