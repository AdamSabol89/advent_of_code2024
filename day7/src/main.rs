use std::{collections::HashSet, fs::File, io::Read};

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

fn parser(input: &str) -> (Vec<u64>, Vec<Vec<u64>>){
    let mut data = input
        .split_terminator("\n")
        .map(|s| s
            .split([':', ' '].as_ref())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().expect(&format!("Error parsing integer at '{}'.", s))).collect::<Vec<u64>>()
        );
    let mut target = Vec::new();
    let mut series = Vec::new(); 

    for line in data{
        let mut data = line.iter(); 
        target.push(*data.next().unwrap());
        series.push(data.cloned().collect())
    }

    (target, series)
}

fn execute_opp(opp: char, previous: u64, index: usize, series: &[u64]) -> u64 {
    if index == 1 {
        return match opp{
            '*'=>{series[0]*series[1]}
            '+'=>{series[0]+series[1]}
            _ => panic!("bad opp")

        }
    }
    match opp {
        '*'=>{series[index]*previous}
        '+'=>{series[index]+previous}
        _ => panic!("bad opp")
    }
}

fn recur(previous_result: Option<u64>, curr_index: usize, series: &[u64], target: u64) -> Option<u64>{
    let previous_result = previous_result?;

    if (curr_index >= series.len()) && (previous_result == target) {
        return Some(previous_result)
    }else if curr_index >= series.len(){
        return None
    }

    let mut temp = execute_opp('*', previous_result, curr_index, series);
    if temp <= target {
        let maybe_result = recur(Some(temp), curr_index + 1, series, target);

        if maybe_result.is_some(){
            return maybe_result
        }
    }

    temp = execute_opp('+', previous_result, curr_index, series);
    if temp <= target {
        return recur(Some(temp), curr_index + 1, series, target);
    }

    None
}

fn solve_silver(targets: &[u64], series: &[Vec<u64>]) -> u64 { 
    let mut result = 0;
    let input = targets.iter().zip(series);

    for (target, series ) in input {
        let maybe_result = recur(Some(0), 1, &series.clone(), *target);
        if let Some(worked) = maybe_result{
            result +=worked;
        }
    }


   result
}

fn main() {
    let path = "src/data.txt";
    let mut file = open_file(path);
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);
    let (parsed_target, parsed_inputs) = parser(&data);
    let result = solve_silver(&parsed_target, &parsed_inputs);
    //let parsed: Vec<&str> = data.split_terminator('\n').collect();
    println!("{}", result);
}
