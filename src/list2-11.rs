fn main() {
    let needle = 42;
    let haystack = [1, 2, 3, 4, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}
