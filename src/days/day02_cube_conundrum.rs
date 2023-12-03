use std::collections::HashMap;

pub fn part_one(input: &Vec<String>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if line
                .split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .all(|round| round.split(", ").all(is_cube_count_possible))
            { i as u32 + 1 } else { 0 }
        }).sum()
}

pub fn part_two(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            compute_game_power(&line.split_once(": ").unwrap().1.split("; ").collect())
    }).sum()
}

fn is_cube_count_possible(cube: &str) -> bool {
    let (count, colour) = cube.split_once(" ").unwrap();
    let count = count.parse::<u32>().unwrap();
    match colour {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => false,
    }
}

fn compute_game_power(rounds: &Vec<&str>) -> u32 {
    let mut colour_mapping = HashMap::from([
         ("red", 0),
         ("green", 0),
         ("blue", 0),
    ]);

    for round in  rounds {
        for cube in round.split(", ") {
            let (count, colour) = cube.split_once(" ").unwrap();
            let count = count.parse::<u32>().unwrap();

            if count > colour_mapping[colour] {
               colour_mapping.insert(colour, count);
            }
        }

    }

    let mut return_value : u32 = 1;
    colour_mapping.values().for_each(|x| return_value *= x);
    return return_value;
}
