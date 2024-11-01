fn main() {
    let one = [1, 2, 3];
    let two = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    for array in arrays {
        println!("{:?}", array);
        for number in array {
            println!("\t{} + 10 = {}", number, number + 10);
        }

        let mut sum = 0;
        for i in 0..array.len() {
            sum += array[i];
        }
        println!("\tSum = {}", sum);
    }
}
