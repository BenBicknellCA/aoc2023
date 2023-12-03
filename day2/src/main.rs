use std::collections::HashMap;
use std::fs;

fn main() {}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn part_1() {
    let mut game_num_count: usize = 0;
    'outer: for line in read_input().lines() {
        let mut split_game: Vec<&str> = line.split([':', ';']).map(|strng| strng.trim()).collect();

        let game_num = &split_game[0]
            .trim_start_matches("Game ")
            .parse::<usize>()
            .unwrap();

        split_game.remove(0);
        for set in split_game.iter() {
            let pair: Vec<_> = set
                .split(',')
                .map(|pair| pair.split_whitespace().collect::<Vec<_>>())
                .collect();
            for i in pair.iter() {
                let colour = &i[1];
                let num = i[0].parse::<usize>().unwrap();
                let test = match *colour {
                    "blue" => num <= 14,
                    "green" => num <= 13,
                    "red" => num <= 12,
                    _ => panic!(),
                };
                if !test {
                    continue 'outer;
                }
            }
        }
        game_num_count += game_num;
    }
}

pub fn try_insert(map: &mut HashMap<&str, usize>, key: &str, val_to_try: &mut usize) {
    if let Some(x) = map.get_mut(key) {
        if x < val_to_try || (*x == 0) {
            *x = *val_to_try
        }
    }
}

fn part_2() -> usize {
    let mut game_num_count = 0;
    for line in read_input().lines() {
        let mut map: HashMap<&str, usize> = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
        let mut split_game: Vec<&str> = line.split([':', ';']).map(|strng| strng.trim()).collect();
        split_game.remove(0);

        for set in split_game.iter() {
            let pair: Vec<_> = set
                .split(',')
                .map(|pair| pair.split_whitespace().collect::<Vec<_>>())
                .collect();

            for i in pair.iter() {
                let colour = &i[1];
                let mut num = i[0].parse::<usize>().unwrap();
                try_insert(&mut map, colour, &mut num);
            }
        }
        let total = map["red"] * map["blue"] * map["green"];
        game_num_count += total;
    }
    game_num_count
}
