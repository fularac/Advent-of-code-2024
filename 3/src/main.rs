use regex::Regex;
use std::io::Read;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("test2.txt").unwrap();

    let mut result = multiply(input.as_str());
    println!("Part 1 result: {}", result);


    let mut active = true;
    let conditionals = Regex::new(r"(do\(\))|(don't\(\))").unwrap();
    let mut index= 0;
    result = 0;
    for m in conditionals.find_iter(input.as_str()) {
        println!("{} {:?} {}", active, m, index);
        if active && m.as_str() == "do()" || !active && m.as_str() == "don't()" {
            continue
        } else if active {
            result += multiply(&input[index..m.start()]);
            active = false;
        } else {
            active = true;
            index=m.end();
        }
    }
    if active {
        result += multiply(&input[index..]);
    }
    println!("Part 2 result: {}", result);
}

fn multiply(input: &str) -> isize {
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    for (_, [x, y]) in re.captures_iter(input).map(|x| x.extract()) {
        result += x.parse::<isize>().unwrap() * y.parse::<isize>().unwrap();
    }
    return result;
}