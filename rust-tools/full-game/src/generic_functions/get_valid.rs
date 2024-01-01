fn get_valid_int() -> i32 {
    let mut input = String::new();
    loop {
        println!("Please enter a number: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}