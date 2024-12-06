use std::{collections::{HashMap, HashSet}, fs::File, io::Read};
use std::cmp::Ordering::{Less, Greater};

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

fn parser(input: &str) -> (Vec<Vec<u64>>, Vec<Vec<u64>>){
    let mut data = input.split("\n\n");

    let p1: Vec<Vec<u64>> = data
        .next()
        .unwrap()
        .split_terminator('\n')
        .map(|line| {
            line.split('|')
                .map(|s| s.parse::<u64>().expect(&format!("Unable to convert {} to u64.", s)))
                .collect::<Vec<u64>>()
        })
        .collect();

    let p2: Vec<Vec<u64>> = data
        .next()
        .unwrap()
        .split_terminator('\n')
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u64>().expect(&format!("Unable to convert {} to u64.", s)))
                .collect::<Vec<u64>>()
        })
        .collect();

    (p1,p2)
}

fn create_rule_map(rules: &[Vec<u64>]) -> HashMap<u64, Vec<u64>> {
    let mut map: HashMap<u64, Vec<u64>> = HashMap::new();

    for rule in rules.iter() {
        if let Some(r) = map.get_mut(&rule[0]) {
            r.push(rule[1]);
        }else{
            map.insert(rule[0], vec![rule[1]]);
        }
    }
    map
}

fn bubble_sort(rule_map: &HashMap<u64, Vec<u64>>, row: &mut [u64]) {
    row.sort_by(|a, b| {
        if let Some(neighbors) = rule_map.get(b) {
            if neighbors.contains(a) {
                return Greater;
            }
        }
        Less
    });
}

fn solve(rules: &[Vec<u64>], input: &mut [Vec<u64>]) -> (u64,u64) {
    let rule_map = create_rule_map(rules);
    let mut silver_sum = 0;
    let mut gold_sum = 0;
    let mut seen_map: HashSet<u64> = HashSet::new();

    for row in input.iter_mut() {
        let mut invalid = false;

        '_row_loop: for num in row.iter() {
            if let Some(rules) = rule_map.get(num) {
                for rule in rules.iter() {
                    if seen_map.contains(rule) {
                        invalid = true;
                        let mut sorted = row.clone();
                        bubble_sort(&rule_map, &mut sorted);
                        gold_sum += sorted[sorted.len()/2];
                        break '_row_loop
                    }
                }
            }
            seen_map.insert(*num);
        }
        if !invalid {
            silver_sum += row[row.len() / 2];
        }
        seen_map.clear();
    }

    (silver_sum, gold_sum)
}

//todays problem was pretty bad 
//sorting theory isnt really my thing
//apparently you use topological sort, which the bultin
//rust comparator supports if you define greater and less? 
//idk i tried other peoples supposed working solutions and they
//gave wrong answers on my input, this was for like 3-4 seperate solutions.
//i think the problem was just not well defined, like i produced valid 
//transforms of the inpuit to one which satisfied rules but still rejected input.
fn main() {
    let path: &str = "src/data.txt";
    let mut file = open_file(path);
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);

    let (rules, mut input) = parser(&data);
    let (silver_result, gold_result) = solve(&rules, &mut input);
    println!("silver: {}. gold {}", silver_result, gold_result);
}


