use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

const SUM_TO_CHECK: i32 = 2020;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn part1(v: &Vec<i32>) -> i32 {
    let mut exit = false;
    let mut mul = 0;
    for i in 0..(v.len()-1) {
        for j in (i+1)..(v.len()) {
            let sum = v[i] + v[j];
            if sum == SUM_TO_CHECK {
                exit = true;
                println!("1: {}, 2: {}", v[i], v[j]);
                mul = v[i] * v[j];
                break;
            }
        }
        if exit {
            break;
        }
    }
    mul
}

fn part2(v: &Vec<i32>) -> usize {
    let mut sums = HashMap::new();
    for i in 0..(v.len()-1) {
        for j in (i+1)..(v.len()) {
            let sum = v[i] + v[j];
            sums.insert(sum, (v[i], v[j]));
        }
    }
    let mut ret: usize = 0;
    for i in 0..(v.len()) {
        let sum = SUM_TO_CHECK - v[i];
        if sums.contains_key(&sum) {
            let (a, b) = sums.get(&sum).unwrap();
            println!("a: {}, b: {}, c: {}", a, b, v[i]);
            ret = (*a as usize) * (*b as usize) * (v[i] as usize);
            break;
        }
    }
    ret
}

fn main() -> io::Result<()>{
    let mut v : Vec<i32> = Vec::new();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line: i32 = line?.parse().expect("Could not parse line");
        v.push(line);
    }
    println!("-----------PART 1-----------");
    println!("Product: {}", part1(&v));
    println!("-----------PART 2-----------");
    println!("Product: {}", part2(&v));
    Ok(())
}
