use std::fs;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", first()?);
    println!("{}", second()?);
    Ok(())
}

fn first() -> Result<i32, Box<dyn Error>> {
    let lines = io::BufReader::new(fs::File::open("../data/input")?).lines();
    let mut sf: i32 = 0;
    for line in lines {
        if let Ok(s) = line {
            let mut t: Vec<i32> = s.split("x").map(|s| s.parse::<i32>().unwrap()).collect();
            t.sort();
            sf += 3 * t[0] * t[1] + 2 * t[1] * t[2] + 2 * t[0] * t[2];
        }
    }
    Ok(sf)
}
fn second() -> Result<i32, Box<dyn Error>> {
    let lines = io::BufReader::new(fs::File::open("../data/input")?).lines();
    let mut sf: i32 = 0;
    for line in lines {
        if let Ok(s) = line {
            let mut t: Vec<i32> = s.split("x").map(|s| s.parse::<i32>().unwrap()).collect();
            t.sort();
            sf += t[0] * 2 + t[1] * 2 + t.iter().product::<i32>();
            // println!("{} - {:?}", tmp, t);
        }
    }
    Ok(sf)
}
