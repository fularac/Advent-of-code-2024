use std::{fs::File, io::BufRead};

fn main() {
    let file = File::open("test_input.txt").unwrap();
    // let file = File::open("part_1_input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut safe_count = 0;
    let mut safe_count2 = 0;

    for line in reader.lines().map(|s| s.unwrap()) {
        let temps: Vec<isize> = line
            .clone()
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        if still_safe(temps[0], &temps[1..], None) {
            //println!("{} is safe!", line);
            safe_count += 1;
        }
        if still_safe_part2(temps[0], &temps[1..], None, false) {
            println!("{} is safe (part2)!", line);
            safe_count2 += 1;
        } else {
            println!("{} is unsafe!", line);
        }
    }
    println!("Part 1 total safe: {}", safe_count);
    println!("Part 2 total safe: {}", safe_count2);
}

fn still_safe(last_temp: isize, remaining: &[isize], assending: Option<bool>) -> bool {
    if remaining.is_empty() {
        return true;
    }
    let new_temp = remaining[0];
    let diff = new_temp - last_temp;
    if diff.abs() == 0 || diff.abs() > 3 {
        println!(
            "Unsafe! {} -> {}: diff({}) 0 or >3 ",
            last_temp, new_temp, diff
        );
        return false;
    }
    if assending.is_none() {
        return still_safe(new_temp, &remaining[1..], Some(diff > 0));
    } else {
        if assending.unwrap() && diff > 0 || !assending.unwrap() && diff < 0 {
            return still_safe(new_temp, &remaining[1..], assending);
        } else {
            let direction = if assending.unwrap() {
                "assending"
            } else {
                "decending"
            };
            println!(
                "Unsafe! {} -> {}: change in temperature direction to {}",
                last_temp, new_temp, direction
            );
            return false;
        }
    }
}

fn still_safe_part2(
    last_temp: isize,
    remaining: &[isize],
    assending: Option<bool>,
    already_failed: bool,
) -> bool {
    false
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn safe_part1() {
        let lines: Vec<Vec<isize>> = "7 6 4 2 1
1 3 6 7 9"
            .split_terminator("\n")
            .map(|s| {
                s.split_whitespace()
                    .map(|line| line.parse::<isize>().unwrap())
                    .collect()
            })
            .collect();
        for line in lines {
            println!("Testing: {:?}", line);
            assert!(still_safe(line[0], &line[1..], None))
        }
    }
    #[test]
    fn unsafe_part1() {
        let lines: Vec<Vec<isize>> = "1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1"
            .split_terminator("\n")
            .map(|s| {
                s.split_whitespace()
                    .map(|line| line.parse::<isize>().unwrap())
                    .collect()
            })
            .collect();
        for line in lines {
            println!("Testing: {:?}", line);
            assert!(!still_safe(line[0], &line[1..], None))
        }
    }
}
