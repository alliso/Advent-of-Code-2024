use std::fs;

fn main() {
    println!("First problem result {}", first_problem());
    println!("Second problem result {}", second_problem());
}

fn first_problem() -> i32 {
    let ids = read_file("src/first_problem_input.txt");
    let mut left_list: Vec<i32> = ids.0;
    let mut right_list: Vec<i32> = ids.1;
    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;
    for i in 0..left_list.len() {
        let point_distance = (left_list[i] - right_list[i]).abs();
        total_distance += point_distance;
    }

    total_distance
}

fn second_problem() -> i32 {
    let ids = read_file("src/first_problem_input.txt");
    let left_list: Vec<i32> = ids.0;
    let right_list: Vec<i32> = ids.1;

    let mut total_distance: i32 = 0;
    for i in 0..left_list.len() {
        let count = right_list.iter().filter(|x| **x == left_list[i]).count() as i32;
        total_distance += count * left_list[i];
    }

    total_distance
}

fn read_file(file_name: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    for line in contents.lines() {
        let ids: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        left_list.push(ids[0]);
        right_list.push(ids[1]);
    }

    (left_list, right_list)
}
