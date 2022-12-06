use std::fs;
use std::collections::HashSet;

fn problem(input: &String, consecutive_long: usize) -> usize {
    let str_to_vec: Vec<char> = input.chars().collect();
    for i in 0..str_to_vec.len(){
        let mut chars = HashSet::from([str_to_vec[i]]);
        for j in (i+1)..str_to_vec.len(){
            
            if !chars.contains(&str_to_vec[j]){ chars.insert(str_to_vec[j]); }
            else { break; }
            if chars.len() == consecutive_long { return j + 1; }
        }
    }
    return 0;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file: String = fs::read_to_string(&args[1]).expect("No se ha podido abrir el archivo.");
    let part_one = problem(&file, 4);
    let part_two = problem(&file, 14);
    println!("Part 1: {:?}", part_one);
    println!("Part 2: {:?}", part_two);
}