use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let r: Vec<char> = fs::read_to_string("../data/input")?.chars().collect();
    println!("{}", first(r.clone()));
    println!("{}", second(r.clone()));
    Ok(())
}

fn first(r: Vec<char>) -> i16 {
    let mut floor: i16 = 0;
    for i in r {
        if i == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }
    floor
}

fn second(r: Vec<char>) -> usize {
    let mut floor: i16 = 0;
    for (num, i) in r.iter().enumerate() {
        // println!(i);
        if i.to_owned() == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            return num + 1
        }
    }
    return 1
}
