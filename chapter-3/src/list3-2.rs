#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

struct Sample {
    data: String,
    data2: String,
}

fn main() {
    let mut f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{:?}", f1_name);
    println!("{:?}", f1_length);

    {
        let f1_data = &f1.data;
        println!("{:?}", f1_data);
    }

    f1.data = vec![114, 117, 115, 116];

    println!("{:?}", f1);
    println!("{:?}", f1_name);
    println!("{:?}", f1_length);

    {
        let f1_data = &f1.data;
        println!("{:?}", f1_data);
    }

    let x = Sample {
        data: String::from("hello"),
        data2: String::from("world"),
    };
    let y = &x.data;
    let z = &(x.data);
    // let w = (&x).data;
    let v = x.data;
    // let u = x.data;
    // let a = x;
    let t = x.data2;
    // let b = x;
}
