use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const SUM_TO_CHECK: i32 = 2020;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> io::Result<()>{
    let mut v : Vec<i32> = Vec::new();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line: i32 = line?.parse().expect("Could not parse line");
        v.push(line);
    }

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
    println!("Answer is {}", mul);
    Ok(())
    /*
    let lines : &str = read_to_string("input.txt")
                .expect("File not found!")
                .lines()
                .collect();
    print_type_of(&lines);
    for line in lines {
        print_type_of(&line);
        //let line = line.parse::<u32>().expect("Not a number!");
        v.push(line);
    }*/

    /*
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
    println!("Answer is {}", mul);
    */
}
