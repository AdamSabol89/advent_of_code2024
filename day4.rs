use std::{fs::File, io::Read, iter, time::Instant};

type Direction = (i32,i32);
const Top: Direction = (-1,0);
const Down: Direction = (1, 0);
const Left: Direction = (0, -1);
const Right: Direction = (0, 1);
const TopRight: Direction = (-1, 1);
const TopLeft: Direction = (-1, -1);
const BottomRight: Direction = (1, 1);
const BottomLeft: Direction = (1, -1);

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

static DIRECTIONS: [Direction; 8] = [
    Top,
    Down,
    Left,
    Right,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
];

fn validate_indices(i: i32, j:i32, num_row: usize, num_col: usize ) -> Option<(usize, usize)>{
    let ui: usize = match i.try_into() {
        Ok(val) => val,
        Err(_) => return None,
    };

    let uj: usize = match j.try_into() {
        Ok(val) => val,
        Err(_) => return None,
    };

    if ui >= num_row || uj >= num_col {
        return None;
    };

    Some((ui, uj))
}

fn check_index(i: i32, j: i32, mat: &[&[u8]], remain: &str, direction: Direction) -> bool {
    if remain.chars().next().is_none(){
        return true
    };

    let (ui, uj) = match validate_indices(i, j, mat.len(), mat[0].len()) {
        Some(indices) => indices,
        None => return false,
    };

    let first_char = remain.as_bytes().iter().next().unwrap();
    if mat[ui][uj] == *first_char {
            let mut chars = remain.chars();
            chars.next();
            let (dy, dx) = direction;
            check_index(i + dy, j + dx, mat, chars.as_str(), direction)
    } else {
        false
    }
}

fn solve_silver(input: &str) -> u64 {
    let mat: Vec<&[u8]> = input.split_terminator('\n').map(|s| s.as_bytes()).collect();
    let mut total = 0;
    for i in 0..mat.len() {
        for j in 0..mat[i].len(){ 
            if mat[i][j] == 88 /*X*/ {
                for direction in DIRECTIONS.iter(){
                    if check_index(i.try_into().unwrap(),j.try_into().unwrap(), &mat, "XMAS", *direction){
                        total +=1;
                    };
                };
            };
        };
    };
    total
}

fn validate_opps(mat: &[&[u8]], i: usize, j: usize, direction_one: Direction, direction_two: Direction) -> bool {
    let (dy, dx) = direction_one;
    let i_new = i as i32 + dx;
    let j_new = j as i32 + dy;

    let (ui, uj) = match validate_indices(i_new, j_new, mat.len(), mat[0].len()) {
        Some(indices) => indices,
        None => return false,
    };

    if mat[ui][uj] == 77 /*M*/ {
        let (dy, dx) = direction_two;
        let i = i as i32 + dx;
        let j = j as i32 + dy;

        let (ui, uj) = match validate_indices(i, j, mat.len(), mat[0].len()) {
            Some(indices) => indices,
            None => return false,
        };

        return mat[ui][uj] == 83
    }

    if mat[ui][uj] == 83 /*S*/ {
        let (dy, dx) = direction_two;
        let i = i as i32 + dx;
        let j = j as i32 + dy;

        let (ui, uj) = match validate_indices(i, j, mat.len(), mat[0].len()) {
            Some(indices) => indices,
            None => return false,
        };

        return mat[ui][uj] == 77
    
    }

    false

}

fn solve_gold(input: &str) -> u64 {

    let mat: Vec<&[u8]> = input.split_terminator('\n').map(|s| s.as_bytes()).collect();
    let mut total = 0;

    for i in 0..mat.len() {
        for j in 0..mat[i].len(){ 
            if mat[i][j] == 65 /*A*/{
                let cond_one = validate_opps(&mat, i, j , BottomRight, TopLeft);
                let cond_two = validate_opps(&mat, i, j , BottomLeft, TopRight);
                if cond_one && cond_two{
                    total +=1
                }

            }
        }
    }
    total
}

fn main() {
    let path = "/home/adam/repos/advent_of_code_2024/data/bigboy4.txt";
    let mut file = open_file(path);
    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let start_silver = Instant::now();
    let result = solve_silver(&data);
    let silver_duration = start_silver.elapsed();

    let start_gold = Instant::now();
    let result2 = solve_gold(&data);
    let gold_duration = start_gold.elapsed();


    println!("total: {}. time: {:?}", result, silver_duration);
    println!("total: {}. time {:?}", result2, gold_duration);
}
