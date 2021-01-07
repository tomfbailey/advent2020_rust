use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn unique_yes_count(this_group: &str) -> usize {
    let mut group_map = HashMap::new();
    for i in 0..this_group.len() {
        let this_char = this_group.chars().nth(i).unwrap();
        if this_char != '\n' && !group_map.contains_key(&this_char) {
            group_map.insert(this_char, 1);
            //println!("Inserted {} | ", this_char);
        }
    }
    group_map.len()
}

fn main() {
    let file = File::open("input.txt");
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).expect("Can't read the file");
    let groups: Vec<&str> = contents.split("\n\n").collect();
    let mut sum = 0;
    for group in groups {
        //println!("Group: {}", group);
        sum = sum + unique_yes_count(group);
    }
    println!("Sum total: {}", sum);
}
