struct CubeSat {
    id: i64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_message(sat: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let a_status = check_message(&sat_a);
    let b_status = check_message(&sat_b);
    let c_status = check_message(&sat_c);
    println!("{:?}, {:?}, {:?}", a_status, b_status, c_status);

    let a_status = check_message(&sat_a);
    let b_status = check_message(&sat_b);
    let c_status = check_message(&sat_c);
    println!("{:?}, {:?}, {:?}", a_status, b_status, c_status);
}
