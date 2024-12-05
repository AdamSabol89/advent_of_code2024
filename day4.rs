use std::{fs::File, io::Read, iter, time::Instant};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Top((i32, i32)),
    Down((i32, i32)),
    Left((i32, i32)),
    Right((i32, i32)),
    TopRight((i32, i32)),
    TopLeft((i32, i32)),
    BottomRight((i32, i32)),
    BottomLeft((i32, i32)),
}

//this ends up being quite bad i think because we have to run time follow a pointer. also extra
//space less cache. silver is actually heavily parralizable
let Direction = (i32,i32);

impl Direction {
    fn as_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Top(tuple) => *tuple,
            Direction::Down(tuple) => *tuple,
            Direction::Left(tuple) => *tuple,
            Direction::Right(tuple) => *tuple,
            Direction::TopRight(tuple) => *tuple,
            Direction::TopLeft(tuple) => *tuple,
            Direction::BottomRight(tuple) => *tuple,
            Direction::BottomLeft(tuple) => *tuple,
        }
    }

    const fn top() -> Self {
        Direction::Top((-1, 0))
    }

    const fn down() -> Self {
        Direction::Down((1, 0))
    }

    const fn left() -> Self {
        Direction::Left((0, -1))
    }

    const fn right() -> Self {
        Direction::Right((0, 1))
    }

    const fn top_right() -> Self {
        Direction::TopRight((-1, 1))
    }

    const fn top_left() -> Self {
        Direction::TopLeft((-1, -1))
    }

    const fn bottom_right() -> Self {
        Direction::BottomRight((1, 1))
    }

    const fn bottom_left() -> Self {
        Direction::BottomLeft((1, -1))
    }
}

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

static DIRECTIONS: [Direction; 8] = [
    Direction::top(),
    Direction::down(),
    Direction::left(),
    Direction::right(),
    Direction::top_right(),
    Direction::top_left(),
    Direction::bottom_right(),
    Direction::bottom_left(),
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
            let (dy, dx) = direction.as_tuple();
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
    let (dy, dx) = direction_one.as_tuple();
    let i_new = <i32 as std::convert::TryFrom<_>>::try_from(i).unwrap() + dx;
    let j_new = <i32 as std::convert::TryFrom<_>>::try_from(j).unwrap() + dy;

    let (ui, uj) = match validate_indices(i_new, j_new, mat.len(), mat[0].len()) {
        Some(indices) => indices,
        None => return false,
    };

    if mat[ui][uj] == 77 /*M*/ {
        let (dy, dx) = direction_two.as_tuple();
        let i = <i32 as std::convert::TryFrom<_>>::try_from(i).unwrap() + dx;
        let j = <i32 as std::convert::TryFrom<_>>::try_from(j).unwrap() + dy;

        let (ui, uj) = match validate_indices(i, j, mat.len(), mat[0].len()) {
            Some(indices) => indices,
            None => return false,
        };

        return mat[ui][uj] == 83
    }

    if mat[ui][uj] == 83 /*S*/ {
        let (dy, dx) = direction_two.as_tuple();
        let i = <i32 as std::convert::TryFrom<_>>::try_from(i).unwrap() + dx;
        let j = <i32 as std::convert::TryFrom<_>>::try_from(j).unwrap() + dy;

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
                let cond_one = validate_opps(&mat, i, j , Direction::bottom_right(), Direction::top_left());
                let cond_two = validate_opps(&mat, i, j , Direction::bottom_left(), Direction::top_right());
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
