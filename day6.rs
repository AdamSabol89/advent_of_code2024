use std::{collections::HashSet, fs::File, io::Read};

type Direction = (i32, i32);
const Up: Direction = (-1, 0);
const Down: Direction = (1, 0);
const Left: Direction = (0, -1);
const Right: Direction = (0, 1);

//static DIRECTIONS: [(i32,i32); 4]= [(1,0), (0,1), (-1,0), (0,-1)];

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


fn change_direction(dir: Direction) -> Direction {
    match dir {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
        _ => {
            panic!("ABSOLUTELY ILLEGAL: BAD COMPILER")
        }
    }
}

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

fn find_start(board: &[&str]) -> Option<(usize, usize, Direction)> {
    for (i, row) in board.iter().enumerate() {
        for (j, guard) in row.chars().enumerate() {
            if ['^', 'v', '>', '<'].contains(&guard) {
                let dir: Direction;
                match guard {
                    '>' => dir = Right,
                    '^' => dir = Up,
                    'v' => dir = Down,
                    '<' => dir = Left,
                    _ => {
                        panic!("ABSOLUTELY ILLEGAL: BAD COMPILER")
                    }
                }

                return Some((i, j, dir));
            }
        }
    }
    None
}

//todo can refactor pretty good i think
//sparse matrix instead of clone 
//parr_iter over solved matrices
fn solve(board: &[&str]) -> (usize, usize) {
    let mut result = 1;
    let mut gold_result = 0;
    let (mut i, mut j, mut dir) = find_start(board).unwrap();
    let (origin_i, origin_j, origin_dir ) = (i,j, dir);
    let mut seen_map: HashSet<(usize,usize)> = HashSet::new();
    let (mut dx, mut dy) = dir;

    seen_map.insert((i,j));

    let row_len = board.len();
    let col_len = board[0].len();

    while let Some((ui, uj))= validate_indices( (i as i32 + dx), (j as i32 + dy), row_len, col_len){
        if board[ui].as_bytes()[uj] == b'#'{
            dir = change_direction(dir);
            (dx, dy) = dir;
            continue
        }
        i = ui;
        j = uj;

        if !seen_map.contains(&(ui,uj)){
            seen_map.insert((ui,uj));
            result += 1;
        }

    }

    for pos in seen_map.iter(){
        let (obs_i ,obs_j) = pos;
        let mut mod_map: Vec<String> = board.iter().map(ToString::to_string).collect();
        let mut seen = vec![false; row_len*col_len*4];

        unsafe {
            mod_map.get_unchecked_mut(*obs_i).as_bytes_mut()[*obs_j] = b'#';
        }

        dir = origin_dir;
        (dx, dy) = dir;
        i = origin_i;
        j = origin_j;

        while let Some((ui, uj))= validate_indices( (i as i32 + dx), (j as i32 + dy), row_len, col_len){
            if mod_map[ui].as_bytes()[uj] == b'#'{
                dir = change_direction(dir);
                (dx, dy) = dir;
                continue
            }
            i = ui;
            j = uj;

            let offset = match dir {
                Down => {0}
                Up => {1}
                Left => {2}
                Right => {3}
                _ => panic!("ABSOLUTELY ILLEGAL: BAD COMPILER")
            };

            let index = ((ui * col_len) + uj) * 4 + offset;

            if seen[index]{
                gold_result += 1;
                break
            }else{
                seen[index] = true;
            }
        }
    }

    (result, gold_result)
}

fn main() {
    let path = "/home/adam/repos/advent_of_code_2024/data/bigboy6.txt";
    let mut file = open_file(path);
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);
    let parsed: Vec<&str> = data.split_terminator('\n').collect();
    let (result, gold_result) = solve(&parsed);

    println!("silver result {}", result);
    println!("gold result {}", gold_result);

    //println!("Start direction {:?}", start_dir);
    //println!("modified direction {:?}", change_direction(start_dir));
}
