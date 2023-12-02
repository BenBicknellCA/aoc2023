use std::fs;

fn main() {}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn part_1() -> usize {
    let mut total: usize = 0;
    let input = read_input();
    for i in input.lines() {
        let nums: Vec<u32> = i
            .chars()
            .filter_map(|x| x.to_digit(10))
            .collect::<Vec<u32>>();
        let first = nums.first().unwrap().to_string();
        let last = nums.last().unwrap().to_string();
        let final_answer = first + &last;
        total += final_answer.parse::<usize>().unwrap();
    }
    total
}

fn part_2() -> usize {
    use aho_corasick::AhoCorasick;

    let input = read_input();
    let patterns = &[
        r"0", r"1", r"2", r"3", r"4", r"5", r"6", r"7", r"8", r"9", r"one", r"two", r"three",
        r"four", r"five", r"six", r"seven", r"eight", r"nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    let mut total = 0;
    for i in input.lines() {
        let mut number = String::new();
        for mat in ac.find_overlapping_iter(i) {
            let get_match = match mat.pattern().as_usize() {
                1..=9 => Some(mat.pattern().as_usize().to_string()),
                10..=19 => Some(patterns[mat.pattern().as_usize() - 9].to_string()),
                _ => panic!(),
            };
            number += get_match.unwrap().as_str();
        }
        let last = number.chars().last().unwrap().to_string();
        let first = number.chars().next().unwrap().to_string();
        let first_last = first + &last;
        total += first_last.parse::<usize>().unwrap();
    }
    total
}
