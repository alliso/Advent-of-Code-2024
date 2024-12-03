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

fn check_level_second(mut level: Vec<i32>, shield: bool) -> bool {
    let is_increasing = level[0] < level[1];
    for i in 1..level.len() {
        match is_increasing {
            true => {
                let diff = level[i] - level[i - 1];
                if (diff.is_positive() && diff.abs() <= 3) == false {
                    level.remove(i - i);
                    return shield && check_level_second(level, false);
                }
            }
            false => {
                let diff = level[i] - level[i - 1];
                if (diff.is_negative() && diff.abs() <= 3) == false {
                    level.remove(i - i);
                    return shield && check_level_second(level, false);
                }
            }
        }
    }

    true
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
