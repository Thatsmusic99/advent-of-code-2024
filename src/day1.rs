use crate::Day;
use std::{collections::HashMap, fs};

fn read_input() -> (Vec<u32>, Vec<u32>) {

    let file_contents = fs::read_to_string("inputs/day1.txt").expect("smh");

    let mut first_list = vec![];
    let mut second_list = vec![];

    for line in file_contents.lines() {
        if line.is_empty() { break; }
        let numbers = line.split("   ");
        let mut numbers_iter = numbers.into_iter();

        for index in 0..=1 {
            let number_str = numbers_iter.next().expect("Number was not found.");
            let number = String::from(number_str).parse::<u32>().expect("Failed to parse int");
            if index == 0 {
                first_list.push(number);
            } else {
                second_list.push(number);
            }
        }
    }

    (first_list, second_list)
}

fn calculate_pairs(mut x: Vec<u32>, mut y: Vec<u32>) -> Vec<(u32, u32)> {
    x.sort();
    y.sort();

    let mut pairs = vec![];

    for i in 0..x.len() {
        pairs.push((x[i], y[i]));
    }

    pairs
}

fn calculate_total_distance(pairs: Vec<(u32, u32)>) -> u32 {

    let mut total = 0;
    for pair in pairs {
        total += pair.0.abs_diff(pair.1);
    }

    total
}

fn calculate_similarity_score(x: Vec<u32>, y: Vec<u32>) -> u32 {

    // Scores used to track similarity scores of a certain value
    let mut total = 0;
    let mut scores: HashMap<u32, u32> = HashMap::new();
    for num in x {

        let similarity_score_option = scores.get(&num);
        if let Some(score) = similarity_score_option {
            total += score;
        } else {

            let mut count = 0;
            for y_num in &y {
                if *y_num == num {
                    count += 1;
                }
            }

            let similarity_score = count * num;
            total += similarity_score;
            scores.insert(num, similarity_score);

        }
    }

    total
}

pub struct Day1 {}

impl Day<u32> for Day1 {
    fn get_answer_part_1() -> u32 {
        
        let lists = read_input();
        let pairs = calculate_pairs(lists.0, lists.1);
        let distance = calculate_total_distance(pairs);

        distance
    }

    fn get_answer_part_2() -> u32 {
        
        let lists = read_input();
        let score = calculate_similarity_score(lists.0, lists.1);

        score
    }
}