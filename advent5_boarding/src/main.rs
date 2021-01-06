use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const ROW_BITS: usize = 7;
const ROW_ZERO: char = 'F';
const COLUMN_BITS: usize = 3;
const COLUMN_ZERO: char = 'L';

const ID_MUL: i32 = 8;

fn bin_string_to_int(s: &str, zero_char: char, length: usize) -> i32 {
    let mut out = 0;
    for i in 0..length {
        if s.chars().nth(i).unwrap() != zero_char {
            let x: i32 = 2;
            out = out + x.pow((length as u32) - (i as u32) - 1);
        }
    }
    out
}

fn get_id(row: i32, column: i32) -> i32 {
    row * ID_MUL + column
}

fn main() -> io::Result<()> {
    let mut v : Vec<String> = Vec::new();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        v.push(line?);
    }
    let mut max = 0;
    for entry in v {
        //let row = &entry[0..7];
        let row = &entry[0..ROW_BITS];
        let column = &entry[ROW_BITS..(ROW_BITS + COLUMN_BITS)];
        let rnum = bin_string_to_int(&row, ROW_ZERO, ROW_BITS);
        let cnum = bin_string_to_int(&column, COLUMN_ZERO, COLUMN_BITS);
        let id = get_id(rnum, cnum);
        if id > max {
            max = id;
        }
    }
    println!("Max id: {}", max);
    Ok(())
}
