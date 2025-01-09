#[derive(Clone)]
struct CubeSat {
    id: u64,
}

#[derive(Debug, Clone)]
enum StatusMessage {
    Ok,
}

fn check_status(sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a.clone());
    println!("{:?}", a_status.clone());

    let a_status = check_status(sat_a);
    println!("{:?}", a_status.clone());
}
