use std::{collections::HashMap, fs};
#[derive(Debug)]
struct Input {
    left: Vec<String>,
    right: Vec<String>,
}

fn main() {
    v1();
    v2()
}

fn v1() {
    let text = fs::read_to_string("../input.txt").unwrap();
    let mut input = Input {
        left: Vec::new(),
        right: Vec::new(),
    };

    let _ = text.split("\n").for_each(|line| {
        let slices: Vec<&str> = line.split("   ").collect();
        if slices.len() == 2 {
            input.left.push(slices[0].to_string());
            input.right.push(slices[1].to_string());
        }
    });

    let distance = calc_distance(input);

    println!("V1: {}", distance)
}

fn calc_distance(mut input: Input) -> i32 {
    let mut distance = 0;

    input.left.sort();
    input.right.sort();

    for i in 0..input.left.len() {
        let left_number: i32 = input.left[i].parse().unwrap();
        let right_number: i32 = input.right[i].parse().unwrap();

        distance += (left_number - right_number).abs()
    }

    distance
}

fn v2() {
    let text = fs::read_to_string("../input.txt").unwrap();
    let mut left: HashMap<i32, i32> = HashMap::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    let _ = text.split("\n").for_each(|line| {
        let slices: Vec<i32> = line
            .split("   ")
            .map(|number| number.parse::<i32>().unwrap_or_default())
            .collect();
        if slices.len() == 2 {
            if let Some(value) = left.get_mut(&slices[0]) {
                *value += 1
            } else {
                left.insert(slices[0], 1);
            }
            if let Some(value) = right.get_mut(&slices[1]) {
                *value += 1
            } else {
                right.insert(slices[1], 1);
            }
        }
    });

    let mut similarity = 0;

    left.keys().for_each(|key| {
        let left_key = left.get(key).unwrap();
        let right_key = match right.get(key) {
            Some(value) => value,
            None => return (),
        };

        similarity += key * left_key * right_key;
    });

    println!("V2: {}", similarity)
}
