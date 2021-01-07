use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn unique_yes_count(this_group: &str) -> HashMap<char, i32> {
    let mut group_map = HashMap::new();
    for i in 0..this_group.len() {
        let this_char = this_group.chars().nth(i).unwrap();
        if this_char != '\n' && !group_map.contains_key(&this_char) {
            group_map.insert(this_char, 1);
        }
    }
    group_map
}

fn all_lines_same(this_group: &str) -> usize {
    let mut lines = this_group.split("\n");
    let mut maps: Vec<HashMap<char, i32>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.len() > 0 {
            maps.push(unique_yes_count(line));
        }
    }
    let mut intersection_map = HashMap::new();
    for (key, _) in &maps[0] {
        intersection_map.insert(key, 1);
    }
    for i in 1..maps.len() {
        let mut to_remove: Vec<char> = Vec::new();
        for (&key, _) in &intersection_map {
            if !maps[i].contains_key(key) {
                to_remove.push(*key);
            }
        }
        for r in to_remove {
            intersection_map.remove(&r);
        }
    }
    intersection_map.len()
}

fn main() {
    let file = File::open("input.txt");
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).expect("Can't read the file");
    let groups: Vec<&str> = contents.split("\n\n").collect();
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for group in groups {
        part1_sum = part1_sum + unique_yes_count(group).len();
        part2_sum = part2_sum + all_lines_same(group);
    }
    println!("Sum total (I): {}", part1_sum);
    println!("-----------PART 2-----------");
    println!("Sum total (II): {}", part2_sum);
}
