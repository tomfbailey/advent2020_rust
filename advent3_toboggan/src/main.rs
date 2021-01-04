use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_trees(terrain: &Vec<Vec<char>>, right: i32, down: i32) -> usize {
    let len = terrain[0].len();
    let mut linenum = (down as usize);
    let mut righttot = (right as usize);
    let mut out = 0;
    while linenum < terrain.len() {
        if terrain[linenum][righttot] == '#' {
            out = out + 1;
            println!("ln: {}, r: {}", linenum, righttot);
        }
        linenum = linenum + (down as usize);
        righttot = (righttot + (right as usize)) % (len);
    }
    out
}

fn muliple_gets(t: &Vec<Vec<char>>) -> usize {
    get_trees(t, 1, 1)
    * get_trees(t, 3, 1)
    * get_trees(t, 5, 1)
    * get_trees(t, 7, 1)
    * get_trees(t, 1, 2)
}

fn main() -> io::Result<()> {
    let mut v : Vec<Vec<char>> = Vec::new();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut chars : Vec<char> = Vec::new();
        for c in line.expect("lines failed").chars() {
            chars.push(c);
        }
        v.push(chars);
    }
    println!("-----------PART 1-----------");
    println!("Trees encountered: {}", get_trees(&v, 3, 1));
    println!("-----------PART 2-----------");
    println!("Mulplicative total: {}", muliple_gets(&v));
    Ok(())
}
