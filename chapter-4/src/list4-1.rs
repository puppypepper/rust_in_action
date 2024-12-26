#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_message(sat_id: i64) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = 0;
    let sat_b = 0;
    let sat_c = 0;

    let a_status = check_message(sat_a);
    let b_status = check_message(sat_b);
    let c_status = check_message(sat_c);
    println!("{:?}, {:?}, {:?}", a_status, b_status, c_status);

    let a_status = check_message(sat_a);
    let b_status = check_message(sat_b);
    let c_status = check_message(sat_c);
    println!("{:?}, {:?}, {:?}", a_status, b_status, c_status);
}
