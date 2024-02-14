use std::io;
use std::str;

pub fn generate() {
    let start_string: [u8;2] = *b"MI";
    let amount: usize;
    loop {
        println!("How many MU system theorems would you like to generate?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        amount = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        break;
    }
    let mut strings: Vec<Vec<u8>> = vec![start_string.to_vec()];
    for counter in 0..amount {
        println!("{}", str::from_utf8(&strings[counter]).unwrap());
        strings.append(&mut get_IaddUs(&strings[counter]));
        strings.append(&mut get_doubles(&strings[counter]));
        strings.append(&mut get_bangs(&strings[counter]));
        strings.append(&mut get_pops(&strings[counter]));
    }
}

// Add a U at the end if there's an I at the end
#[allow(non_snake_case)]
fn get_IaddUs(premise: &[u8]) -> Vec<Vec<u8>> {
    if premise[premise.len()-1] != b'I' { return vec![]; }
    let mut conclusion = premise.to_vec();
    conclusion.push(b'U');
    return vec![conclusion];
}

// Double everything after an M
fn get_doubles(premise: &[u8]) -> Vec<Vec<u8>> {
    if premise[0] != b'M' { return vec![]; }
    let mut conclusion = premise.to_vec();
    conclusion.extend_from_slice(&premise[1..]);
    return vec![conclusion];
}

// Replace any III with U
fn get_bangs(premise: &[u8]) -> Vec<Vec<u8>> {
    return get_replaced_slice_results(premise, b"III", b"U");
}

// Replace any UU with nothing
fn get_pops(premise: &[u8]) -> Vec<Vec<u8>> {
    return get_replaced_slice_results(premise, b"UU", b"");
}

fn get_replaced_slice_results(
    premise: &[u8],
    replacee: &[u8],
    replacer: &[u8]
) -> Vec<Vec<u8>> {
    if premise.len() < replacee.len() { return vec![]; }
    let mut conclusions: Vec<Vec<u8>> = vec![];
    for i in 0..=premise.len() - replacee.len() {
        if premise[i..i+replacee.len()].to_vec() == replacee.to_vec() {
            let mut conclusion = premise.to_vec();
            conclusion.splice(i..i+replacee.len(), replacer.iter().cloned());
            conclusions.push(conclusion);
        }
    }
    return conclusions;
}
