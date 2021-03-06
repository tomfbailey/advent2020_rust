use std::fs::File;
use std::io::{self, prelude::*};
use regex::Regex;

struct LineInfo {
    lower_bound: i32,
    upper_bound: i32,
    character: char,
    password: String,
}

fn create_line_entry(lb: i32, ub: i32, chr: char, pwd: String) -> LineInfo {
    LineInfo {
        lower_bound: lb,
        upper_bound: ub,
        character: chr,
        password: pwd,
    }
}

fn num_in_range(v: &Vec<LineInfo>) -> i32 {
    let mut total = 0;
    let mut count = 0;
    for line_entry in v {
        for chr in line_entry.password.chars() {
            if chr == line_entry.character {
                count = count + 1;
            }
        }
        if count >= line_entry.lower_bound && count <= line_entry.upper_bound {
            total = total + 1;
        }
        count = 0;
    }
    total
}

fn num_at_positions(v: &Vec<LineInfo>) -> i32 {
    let mut total = 0;
    let mut count = 0;
    let mut pos = 1;
    for line_entry in v {
        for chr in line_entry.password.chars() {
            if pos == line_entry.lower_bound || pos == line_entry.upper_bound {
                if chr == line_entry.character {
                    count = count + 1;
                }
            }
            pos = pos + 1;
        }
        if count == 1 {
            total = total + 1;
        }
        count = 0;
        pos = 1;
    }
    total
}

fn main() -> io::Result<()> {
    let mut v : Vec<LineInfo> = Vec::new();
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read the file");
    let r = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s([a-z]+)").unwrap();
    for caps in r.captures_iter(&contents) {
        let lb = caps.get(1).unwrap().as_str().parse().expect("Could not parse lower bound");
        let ub = caps.get(2).unwrap().as_str().parse().expect("Could not parse upper bound");
        let chr = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let pwd = caps.get(4).unwrap().as_str();
        let entry = create_line_entry(lb, ub, chr, pwd.to_string());
        v.push(entry);
    }
    println!("-----------PART 1-----------");
    println!("Total valid passwords: {}", num_in_range(&v));
    println!("-----------PART 2-----------");
    println!("Total valid passwords: {}", num_at_positions(&v));

    Ok(())
}
