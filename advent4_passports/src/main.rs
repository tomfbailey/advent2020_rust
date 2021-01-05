use std::fs::File;
use std::io::{self, prelude::*};

fn contains_consts(this: &str) -> bool {
    this.contains("byr") &&
    this.contains("iyr") &&
    this.contains("eyr") &&
    this.contains("hgt") &&
    this.contains("hcl") &&
    this.contains("ecl") &&
    this.contains("pid")
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read the file");
    let tokens: Vec<&str> = contents.split("\n\n").collect();
    let mut num_present = 0;
    let mut all_fields_present: Vec<&str> = Vec::new();
    for i in 0..(tokens.len()) {
        if contains_consts(&tokens[i]) {
            num_present = num_present + 1;
            all_fields_present.push(&tokens[i]);
        }
    }
    println!("NUMBER OF PASSPORTS with all fields present: {}", num_present);
    println!("-----------PART 2-----------");
    println!("NUMBER OF VALID PASSPORTS: ");

    Ok(())
}
