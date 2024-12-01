use std::{fs::File, io::BufRead, iter::zip};
use std::collections::BTreeMap;

fn main() -> Result<(), String> {
    let file = File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in reader.lines() {
        let parsed: Vec<i32> = line.unwrap().split_whitespace().map(|input | input.parse::<i32>().unwrap()).collect();
        left.push(parsed[0]);
        right.push(parsed[1]);
    }

    left.sort();
    right.sort();
    
    let mut total_distance: i32 = 0;
    let mut similarity = 0;
    let mut right_counts: BTreeMap<i32, i32> = std::collections::BTreeMap::new();
    for (l, r) in zip(left.clone(), right) {
        // Part 1
        total_distance += (l - r).abs();
        // Part 2
        match right_counts.contains_key(&r) {
            true =>  { right_counts.insert(r ,  right_counts[&r] + 1);},
            false => { right_counts.insert(r, 1);},
        }
    }

    for l in left {
        similarity += l * right_counts.get(&l).unwrap_or_else(|| &0);
    }
    println!("Part 1: total distance: {}", total_distance);
    println!("Part 2: Similarity:     {}", similarity);
    Ok(())
}