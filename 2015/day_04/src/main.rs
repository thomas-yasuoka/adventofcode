use md5;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let key = "iwrupvqb";
    println!("{}", first(&key, 5));
    println!("{}", second(&key, 6));

    Ok(())
}

fn first(key: &str, zeros: usize) -> i32 {
    let mut n = 0;
    loop {
        let hash = md5::compute(format!("{}{}", key, n));
        if &format!("{:x}", hash)[..zeros] == "0".repeat(zeros) {
            break;
        }
        n = n + 1;
    }
    n
}

fn second(key: &str, zeros: usize) -> i32 {
    first(key, zeros)
}
