use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::time::Instant;

fn open_file(path: &str) -> File {
    File::open(path).expect(&format!("Error opening file at path, {}.", path))
}

fn string_to_uint(data: &[&str]) -> (u64, u64) {
    let p1 = data[0]
        .parse::<u64>()
        .expect(&format!("Unable to convert {} to u64.", data[0]));
    let p2 = data[1]
        .parse::<u64>()
        .expect(&format!("Unable to convert {} to u64.", data[1]));
    (p1, p2)
}

//sort join is actually not that slow. but an alt approach might be
//count while left == left (c1)
//count while left == right (c2)
//result += c1*c2*left
//left ++ til next
//right ++ til next
fn sort_join(arr1: &[u64], arr2: &[u64]) -> u64 {
    let mut result = 0;
    let mut right_index = 0;
    let arr2_len = arr2.len();

    for &data in arr1 {
        let mut accum = 0;
        let mut temp_index = right_index;

        while temp_index < arr2_len && arr2[temp_index] < data {
            temp_index += 1;
        }

        right_index = temp_index;

        while temp_index < arr2_len && arr2[temp_index] == data {
            accum += 1;
            temp_index += 1;
        }

        result += data * accum;
    }

    result
}

fn hash_join(arr1: &[u64], arr2: &[u64]) -> u64 {
    //data is already sorted tho lol
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(4_000_000);
    for data in arr2.iter() {
        let count = map.entry(*data).or_insert(0);
        *count += 1;
    }

    let mut result: u64 = 0;
    for data in arr1.iter() {
        if let Some(val) = map.get(data) {
            result += *val * data;
        }
    }

    result
}

fn main() {
    //http://0x0.st/XRnR.txt.7z
    let path = "/home/adam/repos/advent_of_code_2024/data/bigboy.txt";
    let f = open_file(path);
    let mut reader = BufReader::new(f);
    let mut buffer = String::with_capacity(70_000_000);

    let (mut arr1, mut arr2) = (Vec::with_capacity(4_000_000), Vec::with_capacity(4_000_000));
    while let Ok(read) = reader.read_line(&mut buffer) {
        if read == 0 {
            break;
        }
        let data: Vec<&str> = buffer.split("   ").map(|s| s.trim()).collect();
        let (p1, p2) = string_to_uint(&data);

        arr1.push(p1);arr2.push(p2);

        buffer.clear()
    }

    let sort_start = Instant::now();
    arr1.sort_unstable();
    arr2.sort_unstable();
    let sort_duration = sort_start.elapsed();

    let mut result: u64 = 0;
    for (p1, p2) in arr1.iter().zip(arr2.iter()) {
        result += u64::abs_diff(*p1, *p2);
    }

    println!("p1 result: {}", result);

    let mut start = Instant::now();
    let result_2 = hash_join(&arr1, &arr2);
    let mut duration = start.elapsed();
    println!("Hash join implementation took: {:?}", duration);

    start = Instant::now();
    let result_3 = sort_join(&arr1, &arr2);
    duration = start.elapsed();

    println!(
        "Sort join implementation took: {:?}",
        duration.add(sort_duration)
    );

    println!(
        "p2 result. hash join {}. p2 result. sort join {}",
        result_2, result_3
    )
}
