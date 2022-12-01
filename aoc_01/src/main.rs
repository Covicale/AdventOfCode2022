use std::fs::File;
use std::io::{BufRead, BufReader};

fn top_three(input: &Vec<u32>) -> u32{
    let mut top = [0; 3];
    for element in input {
        let min_val = top.iter().min().unwrap(); 
        if element > min_val { top[top.iter().position(|&x| x == *min_val).unwrap()] = *element; }
    }
    let mut sum: u32 = 0;
    for val in top { sum += val; } 
    return sum;
}

fn max_calories(input: &Vec<u32>) -> u32{
    let mut max: u32 = 0;
    for element in input {
        if max < *element { max = *element; }
    }
    return max;
}

fn get_vec_from_input(f: &String) -> Vec<u32> {
    let mut vec_inputs: Vec<u32> = Vec::new();
    let file = File::open(f).expect("No se ha podido abrir el archivo.");
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() > 0 {
            let value: u32 = line.parse().unwrap(); 
            sum += value; 
        }
        else { 
            vec_inputs.push(sum);
            sum = 0;
        }
    }
    return vec_inputs;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: Vec<u32> = get_vec_from_input(&args[1]);
    let max = max_calories(&input);
    let max_three = top_three(&input);
    
    println!("Max: {}\nMax 3: {}", max, max_three);
}
