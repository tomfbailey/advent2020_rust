use std::fs::File;
use std::io::{self, prelude::*};
use regex::Regex;

fn contains_consts(this: &str) -> bool {
    this.contains("byr") &&
    this.contains("iyr") &&
    this.contains("eyr") &&
    this.contains("hgt") &&
    this.contains("hcl") &&
    this.contains("ecl") &&
    this.contains("pid")
}

fn valid_passport(this: &str) -> bool {
    let mut out = true;
    let r = Regex::new(r"byr:(\d+)").unwrap();
    let mut byr = 0;
    match r.captures(this) {
        Some(x) => { byr = r.captures(this).unwrap().get(1).unwrap().as_str().parse().expect("Could not parse");
                     println!("byr::{}", byr)}
        None    => return false,
    }
    let r = Regex::new(r"iyr:(\d+)").unwrap();
    let mut iyr = 0;
    match r.captures(this) {
        Some(x) => { iyr = r.captures(this).unwrap().get(1).unwrap().as_str().parse().expect("Could not parse");
                     println!("iyr::{}", iyr)}
        None    => return false,
    }
    let r = Regex::new(r"eyr:(\d+)").unwrap();
    let mut eyr = 0;
    match r.captures(this) {
        Some(x) => { eyr = r.captures(this).unwrap().get(1).unwrap().as_str().parse().expect("Could not parse");
                     println!("eyr::{}", eyr)}
        None    => return false,
    }
    let r = Regex::new(r"hgt:(\d+)cm").unwrap();
    let mut iscm = true;
    let mut hgtcm = 0;
    match r.captures(this) {
        Some(x) => { hgtcm = r.captures(this).unwrap().get(1).unwrap().as_str().parse().expect("Could not parse");
                     println!("hgt::{}cm", hgtcm)}
        None    => iscm = false,
    }
    let mut hgtin = 0;
    if !iscm {
        let r = Regex::new(r"hgt:(\d+)in").unwrap();
        match r.captures(this) {
            Some(x) => { hgtin = r.captures(this).unwrap().get(1).unwrap().as_str().parse().expect("Could not parse");
                         println!("hgt::{}in", hgtin)}
            None    => return false,
        }
    }
    let r = Regex::new(r"hcl:#([0-9a-f]{6}$|[0-9a-f]{6}\s)").unwrap();
    let mut hcl = String::new();
    match r.captures(this) {
        Some(x) => { hcl = String::from(r.captures(this).unwrap().get(1).unwrap().as_str());
                           println!("hcl::#{}", hcl)}
        None    => return false,
    }
    let r = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let mut ecl = String::new();
    match r.captures(this) {
        Some(x) => { ecl = String::from(r.captures(this).unwrap().get(1).unwrap().as_str());
                           println!("ecl::{}", ecl)}
        None    => return false,
    }
    let r = Regex::new(r"pid:(\d{9}$|\d{9}\s)").unwrap();
    let mut pid = String::new();
    match r.captures(this) {
        Some(x) => { pid = String::from(r.captures(this).unwrap().get(1).unwrap().as_str());
                     println!("pid::{}", pid)}
        None    => return false,
    }
    if byr > 2002 || byr < 1920 { out = false; }
    if iyr > 2020 || iyr < 2010 { out = false; }
    if eyr > 2030 || eyr < 2020 { out = false; }
    if iscm {
        if hgtcm > 193 || hgtcm < 150 { out = false; }
    } else {
        if hgtin > 76 || byr < 59 { out = false; }
    }
    return out
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
    let mut num_valid = 0;
    for entry in all_fields_present {
        if valid_passport(&entry) {
            num_valid = num_valid + 1;
            println!("^^VALID");
        }
        println!("-------");
    }
    println!("NUMBER OF VALID PASSPORTS: {}", num_valid);

    Ok(())
}
