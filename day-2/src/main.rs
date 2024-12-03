use std::fs;

fn main() {
    println!("First problem {}", first_problem());
    println!("Second problem {}", second_problem());
}

fn first_problem() -> i32 {
    let report = read_file("src/first_problem_input.txt");
    report
        .iter()
        .map(|level| check_level(level.clone()))
        .filter(|is_safe| *is_safe)
        .count() as i32
}

fn check_level(level: Vec<i32>) -> bool {
    let is_increasing = level[0] < level[1];
    for i in 1..level.len() {
        match is_increasing {
            true => {
                let diff = level[i] - level[i - 1];
                if (diff.is_positive() && diff.abs() <= 3) == false {
                    return false;
                }
            }
            false => {
                let diff = level[i] - level[i - 1];
                if (diff.is_negative() && diff.abs() <= 3) == false {
                    return false;
                }
            }
        }
    }

    true
}

fn second_problem() -> i32 {
    let report = read_file("src/first_problem_input.txt");

    report
        .iter()
        .map(|level| check_level_second(level.clone(), true))
        .filter(|is_safe| *is_safe)
        .count() as i32
}

fn check_level_second(level: Vec<i32>, shield: bool) -> bool {
    let is_increasing = level[0] < level[1];
    for i in 1..level.len() {
        match is_increasing {
            true => {
                let diff = level[i] - level[i - 1];
                if (diff.is_positive() && diff.abs() <= 3) == false {
                    return call_to_unsafe(level, shield, i);
                }
            }
            false => {
                let diff = level[i] - level[i - 1];
                if (diff.is_negative() && diff.abs() <= 3) == false {
                    return call_to_unsafe(level, shield, i);
                }
            }
        }
    }

    true
}

fn call_to_unsafe(mut level: Vec<i32>, shield: bool, i: usize) -> bool {
    let mut is_first_valid = false;
    if i == 2 {
        let mut first = level.clone();
        first.remove(i - 2);
        is_first_valid = check_level_second(first, false);
    }
    let mut current = level.clone();
    level.remove(i - 1);
    current.remove(i);
    return shield
        && (check_level_second(level, false)
            || check_level_second(current, false)
            || is_first_valid);
}

fn read_file(file_name: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(file_name)
        .expect("Should have been able to read the file")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}
