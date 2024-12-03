use std::{fs::File, io::Read};
use std::time::Instant;

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

fn solve_silver(data: String) -> usize {
    let rows: Vec<&str> = data.trim().split('\n').collect();
    let mut num_safe = rows.len();

    for row in rows {
        let points: Vec<usize> = row
            .split_whitespace()
            .map(|s| {
                s.parse::<usize>()
                    .expect(&format!("Unable to convert {} to usize.", s))
            })
            .collect();

        if points.len() < 2 {
            continue;
        }

        let asc = points[1] > points[0];

        for i in 1..points.len() {
            if points[i].abs_diff(points[i - 1]) > 3 {
                num_safe -= 1;
                break;
            }

            if points[i] == points[i - 1] {
                num_safe -= 1;
                break;
            }

            if asc && points[i] < points[i - 1] {
                num_safe -= 1;
                break;
            }

            if !asc && points[i] > points[i - 1] {
                num_safe -= 1;
                break;
            }
            if i == points.len() - 1 {
                //println!("{}", row)
            }
        }
    }

    num_safe
}

fn check_points(p1: usize, p2: usize, ascending: &mut bool, descending: &mut bool) ->  bool {
    *ascending = *ascending || (p1 < p2);
    *descending = *descending || (p1 > p2);

    if *ascending && *descending {
        return false 
    }

    if p1 == p2 { 
        return false 
    }

    if p1.abs_diff(p2) > 3 {
        return false 
    }

    true

}

fn check_safe(points: &[usize], check_again: bool) ->  bool {
    let mut ascending = false; 
    let mut descending = false;

    for i  in 1..points.len(){

        if points.len() < 2 {
            return true
        }
        let safe = check_points(points[i-1], points[i], &mut ascending, &mut descending);

        if !safe && check_again { 
            let mut removed_points = points.to_vec();
            removed_points.remove(i);


            let safe_inner = check_safe(&removed_points, false);
            if safe_inner {
                return safe_inner 
            }
            if i < 3 {
                removed_points = points.to_vec();
                removed_points.remove(0);
                let safe_zero = check_safe(&removed_points, false);
                if safe_zero{
                    return safe_zero
                }
            }

            removed_points = points.to_vec();
            removed_points.remove(i-1);
            return check_safe(&removed_points, false);
        } 

        if !safe && !check_again{
            return false
        }

    }
    true
}

fn solve_gold(data: String) -> usize {
    let rows = data.split_terminator("\n");

    let mut num_unsafe = 0;
    for row in rows {
        let points: Vec<_> = row 
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<usize>().expect(&format!("Unable to convert {} to usize.", s)))
            .collect();
        if check_safe(&points, true){
            num_unsafe +=1;
        }else{
        }
    }
    num_unsafe
}

fn main() {
    //todays was a pain in the ass because of edge cases and state tracking on the gold problem
    //example set was not good. anyway prob a bunch of optimizations here.
    //maybe especially multiple allocations. someday ill clean up but for now this is good enough
    let mut file = open_file("data/bigboy2.txt");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);
    let result = solve_silver(data.clone());
    println!("{}", result);

    let start = Instant::now();
    let result2 = solve_gold(data);
    let duration = start.elapsed();
    println!(
        "Gold took {:?}",
        duration
    );
    println!("{}", result2);
}
