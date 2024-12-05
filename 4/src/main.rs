use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let word_search = parse_input("input.txt");
    let word: Vec<char> = "XMAS".chars().collect();
    let count = find_all_matches(&word, &word_search);
    println!("Part 1: {}", count);
}

fn parse_input(path: &str) -> Vec<Vec<char>> {
    let mut word_search: Vec<Vec<char>> = vec![];
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines().map(|s| s.unwrap()) {
        word_search.push(line.chars().collect());
    }
    word_search
}

fn find_all_matches(word: &Vec<char>, word_search: &Vec<Vec<char>>) -> usize {
    let mut count: usize = 0;
    let max_x = word_search.len();
    let max_y = word_search[0].len();
    for x in 0..max_x {
        for y in 0..max_y {
            if word_search[x][y] != word[0] {
                continue;
            }
            let ix: isize = x.try_into().unwrap();
            let iy: isize = y.try_into().unwrap();
            count += search(0, ix, iy, 0, 1, word, word_search)
                + search(0, ix, iy, 1, 1, word, word_search)
                + search(0, ix, iy, 1, 0, word, word_search)
                + search(0, ix, iy, 0, -1, word, word_search)
                + search(0, ix, iy, -1, -1, word, word_search)
                + search(0, ix, iy, -1, 0, word, word_search)
                + search(0, ix, iy, 1, -1, word, word_search)
                + search(0, ix, iy, -1, 1, word, word_search);
        }
    }
    count
}

fn search(
    i: usize,
    x: isize,
    y: isize,
    inc_x: isize,
    inc_y: isize,
    word: &Vec<char>,
    word_search: &Vec<Vec<char>>,
) -> usize {
    let max_x: isize = word_search.len().try_into().unwrap();
    let max_y: isize = word_search[0].len().try_into().unwrap();
    if max_x == x || max_y == y || x == -1 || y == -1 {
        return 0;
    }

    let ux: usize = x.try_into().unwrap();
    let uy: usize = y.try_into().unwrap();
    if word[i] != word_search[ux][uy] {
        return 0;
    }
    if i + 1 == word.len() {
        return 1;
    }
    return search(i + 1, x + inc_x, y + inc_y, inc_x, inc_y, word, word_search);
}
