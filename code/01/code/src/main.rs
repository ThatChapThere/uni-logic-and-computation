use std::io;

#[derive(Debug)]
enum Menu {
    Generate,
    Test,
}

fn main() {
    let choice: Menu;
    println!("Please choose an option:");
    println!("1. Generate MU system theorems");
    println!("2. Test if a string is part of the MU system");

    loop {
        //let mut input = String::new();
        let mut input = String::from("1");
        //io::stdin().read_line(&mut input).expect("Failed to read line");
        choice = match input.trim().parse() {
            Ok(1) => Menu::Generate,
            Ok(2) => Menu::Test,
            _ => continue,
        };
        break;
    }

    match choice {
        Menu::Generate => generate(),
        Menu::Test => test(),
    }
}

fn generate() {
    let start_string: [u8;2] = *b"MU";
    let amount: u32;
    println!("How many MU system theorems would you like to generate?");

    loop {
        //let mut input = String::new();
        let mut input = String::from("10");
        //io::stdin().read_line(&mut input).expect("Failed to read line");
        amount = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        break;
    }

    let mut counter: u32 = 0;

    while counter < amount {

        println!("{}", counter);
        counter += 1;
    }
}

// Add a U at the end if there's an I at the end
fn get_I_add_Us(input: &[u8]) -> Vec<Vec<u8>> {
    if input[input.len()-1] != b'I' { return vec![]; }
    let mut output = input.to_vec();
    output.push(b'U');
    return vec![output];
}

// Double everything after an M
fn get_doubles(input: &[u8]) -> Vec<Vec<u8>> {
    if input[0] != b'M' { return vec![]; }
    let mut output = input.to_vec();
    output.extend_from_slice(&input[1..]);
    return vec![output];
}

// Replace any III with U
fn get_bangs(input: &[u8]) -> Vec<Vec<u8>> {
    return get_replaced_slice_results(input, b"III", b"U");
}

// Replace any UU with nothing
fn get_pops(input: &[u8]) -> Vec<Vec<u8>> {
    return get_replaced_slice_results(input, b"UU", b"");
}

fn get_replaced_slice_results(input: &[u8], replacee: &[u8], replacer: &[u8]) -> Vec<Vec<u8>> {
    if input.len() < replacee.len() { return vec![]; }
    let mut outputs: Vec<Vec<u8>> = vec![];
    for i in 0..=input.len() - replacee.len() {
        if input[i..i+replacee.len()].to_vec().eq(&replacee.to_vec()) {
            let mut output = input.to_vec();
            output.splice(i..i+replacee.len(), replacer.to_vec());
            outputs.push(output);
        }
    }
    outputs
}

fn test() {
}
