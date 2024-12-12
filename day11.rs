use std::{collections::HashMap, fs::File, io::Read, iter};
use std::mem;

fn open_file(path: &str) -> File 
{
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

fn transform(var: &str) -> Vec<String>
{
    if var == "0" {
        vec!["1".to_string()]
    } else if var.len() % 2 == 0 {
        let (first_half, second_half) = var.split_at(var.len()/2);
        let first_val = first_half.parse::<u64>().expect("parse error").to_string();
        let second_val = second_half.parse::<u64>().expect("parse error").to_string();
        vec![first_val, second_val]
    } else {
        let as_int = var.parse::<u64>().expect("parse error");
        vec![(as_int * 2024).to_string()]
    }
} 

fn dfs(var: &str, depth: usize, memo: &mut HashMap<(String, usize), usize>) -> usize 
{
    if depth == 0 {
        return 1;
    }

    let key = (var.to_string(), depth);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let mut result = 0;
    for next_var in transform(var) {
        result += dfs(&next_var, depth - 1, memo);
    }

    memo.insert(key, result);
    result
}

fn solve_gold(input: &[&str])-> usize{ 
    let steps = 75;
    let mut memo = HashMap::new();
    let mut total = 0;
    for var in input {
        total += dfs(var, steps, &mut memo);
    }
    total
}

fn solve_silver(input: &[&str]) -> usize {
    let mut old_stack: Vec<String> = input.iter().map(|s| s.to_string()).collect();

    for i in 0..25 {
        let mut new_stack = Vec::new();
        println!("iteration {}", i);

        for var in &old_stack {
            if var == "0" {
                new_stack.push("1".to_string());
            } else if var.len() % 2 == 0 {
                let (first_half, second_half) = var.split_at(var.len()/2);
                let first_val = first_half.parse::<u64>().expect("parse error");
                let second_val = second_half.parse::<u64>().expect("parse error");
                new_stack.push(first_val.to_string());
                new_stack.push(second_val.to_string());
            } else {
                let as_int = var.parse::<u64>().expect("parse error");
                new_stack.push((as_int * 2024).to_string());
            }
        }

        let total_memory: usize = new_stack.iter()
            .map(|s| s.capacity()) // get the capacity of each string
            .sum();

        println!(
            "Total memory used by strings in new_stack: {} bytes",
            total_memory
        );

        old_stack = new_stack;
    }

    old_stack.len()
}

fn main() 
{
    let mut file = open_file("src/data.txt");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);
    let parsed: Vec<&str> = data.trim().split(' ').collect();
    let silver_result = solve_silver(&parsed);
    println!("result: {}", silver_result);

    let gold_result = solve_gold(&parsed);
    println!("result: {}", gold_result);
}

