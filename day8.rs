use std::{collections::{HashMap, HashSet}, fs::File, io::Read, isize};
use itertools::Itertools;

fn validate_indices(i: isize, j:isize, num_row: usize, num_col: usize ) -> Option<(usize, usize)>{
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

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

fn solve_silver(input: &[&str]) -> usize
{
    let rows = input.len(); 
    let cols = input[0].len();
    let mut seen = vec![false; rows*cols];
    let mut found_indices: HashMap<char,Vec<(usize,usize)>> = HashMap::new();

    for (i,col) in input.iter().enumerate(){
        for (j, c) in col.chars().enumerate(){
            if !['.','#'].contains(&c) {
                found_indices.entry(c).or_default().push((i, j));
                println!("thinks there node at {}, {}", i, j ); 
            }
        }
    }

    for c in found_indices.keys() {
        if let Some(indices) = found_indices.get(c) {
            for (p1, p2) in indices.iter().tuple_combinations() {
                let (x1, y1) = *p1; 
                let (x2, y2) = *p2; 
                let index = x1 * rows + y1;
                seen[index] = true;
                let index = x2 * rows + y2;
                seen[index] = true;

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                let mut x1 = x1 as isize - dx;
                let mut x2 = x2 as isize + dx; 
                let mut y1 = y1 as isize - dy; 
                let mut y2 = y2 as isize + dy;

                while let Some((nx1,ny1)) = validate_indices(x1,y1,rows,cols){
                    x1 = nx1 as isize - dx;
                    y1 = ny1 as isize - dy;
                    let index = nx1 * rows + ny1;
                    seen[index] = true;

                }

                while let Some((nx2,ny2)) = validate_indices(x2,y2,rows,cols){
                    x2 = nx2 as isize + dx;
                    y2 = ny2 as isize + dy;
                    let index = nx2 * rows + ny2;
                    seen[index] = true;

                }

            }
        }
    }

    let mut result = 0;
    for antinode in seen.iter(){
        if *antinode{
            result +=1;
        }
    }
    result
}

fn main() {
    let path = "src/data.txt";
    let mut file = open_file(path);
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);
    let parsed: Vec<&str> = data.split_terminator('\n').collect();
    let result = solve_silver(&parsed);

    println!("{}", result);
    //println!("modified direction {:?}", change_direction(start_dir));
}
