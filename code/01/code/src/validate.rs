use std::io;

pub fn validate() {
    println!("Please enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let input_bytes = input.as_bytes();
    if validate_theorem(input_bytes) {
        println!("Yes, {} is a valid MU system theorem", input);
    } else {
        println!("No, {} is not a valid MU system theorem", input);
    }
}

fn validate_theorem(theorem: &[u8]) -> bool {
    if theorem[0] != b'M' { return false; }
    #[allow(non_snake_case)]
    let mut I_counter: usize = 0;
    for character in &theorem[1..] {
        match character {
            b'I' => I_counter += 1,
            b'U' => (),
            _ => return false,
        }
    }
    return I_counter % 3 != 0;
}
