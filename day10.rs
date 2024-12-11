use std::{collections::HashMap, fs::File, io::Read, iter};

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

fn validate_indices(i: isize, j: isize, num_row: usize, num_col: usize ) -> Option<(usize, usize)>{
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

fn dfs_from(seen_map: &mut Vec<bool>, topo_map: &[Vec<u64>], i: usize, j: usize, target: usize, result: &mut usize) { 
    let ii = i as isize; 
    let ij = j as isize; 

    if topo_map[i][j] != target as u64{
        return 
    }


    //silver
    //if topo_map[i][j] == 9 && !seen_map[(topo_map.len() * i + j)]{
    //    *result +=1;
    //    seen_map[topo_map.len()* i + j] = true;
    //    return true
    //}

    if topo_map[i][j] == 9{
        *result +=1;
        return
    }

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut found = false;

    for (di, dj) in directions {
        if let Some((ni, nj)) = validate_indices(ii + di, ij + dj, topo_map.len(), topo_map[0].len())
        {
            dfs_from(seen_map, topo_map, ni, nj, target + 1, result)
        }
    }

}

fn solve_silver(topo_map: &[Vec<u64>]) -> usize {
    let mut result = 0;
    for (i, row) in topo_map.iter().enumerate(){
        for (j, start_pos) in row.iter().enumerate(){ 
            if *start_pos == 0 {

                let mut seen_map = vec![false; row.len()*topo_map.len()];
                dfs_from(&mut seen_map, topo_map, i,j, 0, & mut result);
            }
        }
    }

    result
}

//heres day 10
//i hated/still hate day9 so no solution yet
fn main() {
    let mut file = open_file("src/data.txt");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);
    let parsed: Vec<Vec<u64>> = data
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect(&format!("Unable to convert {} to u64.", c)) as u64)
                .collect()
        })
        .collect();

    let result = solve_silver(&parsed);
    println!("result: {}", result);
}
