use std::fs;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>>  {
    // let r: Vec<char> = fs::read_to_string("data/input")?.chars().collect();
    // let r: String = fs::read_to_string("data/input")?;
    // let r: std::str::Lines = fs::read_to_string("data/input")?;
    let r: String = fs::read_to_string("data/input")?;
    println!("{:?}", first(r.clone()));
    println!("{:?}", first(r.clone()));
    Ok(())
}

fn first(r: String) -> u32 {
    fn vowel_count(s: &str) -> u8 {
        s.matches(&['a', 'e', 'i', 'o', 'u'][..]).count().try_into().unwrap()
    }
    fn double_chars(s: &str) -> bool {
        let mut c: char = '!';
        for i in s.chars() {
            if c==i {
                return true
            }
            c = i;
        }
        return false
    }
    fn disallowed_str(s: &str) -> bool {
        let disallowed = ["ab", "cd", "pq", "xy"];
        let mut any_disallowed: bool = false;
        for i in disallowed.iter() {
            if s.contains(i) {
                any_disallowed = true;
                break;
            }
        }
        any_disallowed
    }
    let r = r.lines();
    let mut count: u32 = 0;

    let mut vowels: u8;
    let mut doubles: bool;
    let mut disllwds: bool;

    for i in r {
        vowels = vowel_count(&i);
        doubles = double_chars(&i);
        disllwds = disallowed_str(&i);
        if (vowels >= 3) & doubles & !disllwds {
            count += 1;
        }
    }
    count
}
