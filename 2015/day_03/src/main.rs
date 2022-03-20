use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let r: Vec<char> = fs::read_to_string("../data/input")?.chars().collect();
    println!("{}", first(r.clone()));
    println!("{}", second(r.clone()));
    Ok(())
}

fn first(r: Vec<char>) -> usize {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut coords: Vec<(i16, i16)> = Vec::new();
    for i in r {
        match i {
            '>' => {x += 1},
            '<' => {x -= 1},
            'v' => {y -= 1},
            '^' => {y += 1},
            _ => println!("else"),
        }
        coords.push((x, y));
    }
    coords.sort();
    coords.dedup();
    if coords.contains(&(0, 0)) {
        return coords.len()
    }
    coords.len() + 1
}

fn second(r: Vec<char>) -> usize {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut x_bot: i16 = 0;
    let mut y_bot: i16 = 0;

    let mut coords: Vec<(i16, i16)> = Vec::new();

    for (i, value) in r.iter().enumerate() {
        if i % 2 == 0 {
            match value {
                '>' => {x += 1},
                '<' => {x -= 1},
                'v' => {y -= 1},
                '^' => {y += 1},
                _ => println!("else"),
            }
            coords.push((x, y));
        } else {
            match value {
                '>' => {x_bot += 1},
                '<' => {x_bot -= 1},
                'v' => {y_bot -= 1},
                '^' => {y_bot += 1},
                _ => println!("else"),
            }
            coords.push((x_bot, y_bot));
        }
    }
    coords.sort();
    coords.dedup();

    if coords.contains(&(0, 0)) {
        return coords.len()
    }
    coords.len() + 1

}
