use std::fs;

use regex::Regex;

fn main() {
    println!("First problem mock {}", first_problem(mock_data()));
    println!(
        "First problem {}",
        first_problem(real_data("src/problem_input.txt"))
    );
    println!("Second problem mock {}", second_problem(mock_data_2()));
    println!(
        "Second problem {}",
        second_problem(real_data("src/problem_input.txt"))
    );
}

fn first_problem(memory: String) -> i32 {
    memory
        .split("mul(")
        .map(|partial_memory| check_split(partial_memory.to_string()))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn check_split(memory: String) -> i32 {
    let segment = memory
        .split(")")
        .flat_map(|x| x.split(","))
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .clone();
    if segment.len() > 1 && segment[0].len() <= 3 && segment[1].len() <= 3 {
        let first = match segment[0].parse() {
            Ok(x) => x,
            Err(_) => 0,
        };
        let second = match segment[1].parse() {
            Ok(x) => x,
            Err(_) => 0,
        };
        return first * second;
    }
    0
}

fn second_problem(memory: String) -> i32 {
    let exp = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let operations = exp.find_iter(memory.as_str()).map(|x| x.as_str());
    let mut flag = true;
    let digits_exp = Regex::new(r"[0-9]{1,3}").unwrap();
    let mut multiplication_array: Vec<i32> = vec![];
    for operation in operations {
        match operation {
            "do()" => flag = true,
            "don't()" => flag = false,
            _ => {
                if flag {
                    let product = digits_exp
                        .find_iter(operation)
                        .map(|x| x.as_str().parse::<i32>().unwrap())
                        .product::<i32>();
                    multiplication_array.push(product);
                }
            }
        }
    }

    multiplication_array.into_iter().sum()
}

fn mock_data() -> String {
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
}

fn mock_data_2() -> String {
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()
}

fn real_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Should have been able to read the file")
}
