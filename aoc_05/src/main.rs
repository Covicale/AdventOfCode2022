use std::fs;

fn part_one(input: &Vec<&str>) {
    // Read Stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let number_stacks = if input[0].len() == 3 { 1 } else { (input[0].len() as f64 / 4.0).ceil() as usize };
    for _i in 0..number_stacks { stacks.push(Vec::new()); }

    let mut line = 0; // Linea que lee
    for i in 0..input.len() {
        if (input[i].as_bytes()[2] as char).is_digit(10) || input[i].len() == 0 { 
            line += 2;
            break; 
        }
        let mut line_to_vec = input[i].chars().collect::<Vec<char>>();
        if line_to_vec[0] != ' ' && line_to_vec[0] != '['  { line_to_vec.remove(0); }
        for j in 0..number_stacks {
            if line_to_vec[1+(j*4)] != ' ' { stacks[j].push(line_to_vec[1+(j*4)]); }
        } 
        line += 1;
    }

    // Do Instructions
    for i in line..input.len() {
        let instruction = input[i].split(" ").collect::<Vec<&str>>();
        let quantity: usize = instruction[1].parse().unwrap();
        let from: usize = instruction[3].parse().unwrap();
        let dest: usize = instruction[5].parse().unwrap();
        for _j in 0..quantity{
            let val: char = stacks[from - 1][0];
            stacks[from - 1].remove(0);
            stacks[dest - 1].insert(0, val);
        }
    }
    let mut result = String::new();
    for stack in stacks{
        result += &stack[0].to_string();
    }
    println!("{}", result);
}

fn part_two(input: &Vec<&str>) {
    // Read Stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let number_stacks = if input[0].len() == 3 { 1 } else { (input[0].len() as f64 / 4.0).ceil() as usize };
    for _i in 0..number_stacks { stacks.push(Vec::new()); }

    let mut line = 0; // Linea que lee
    for i in 0..input.len() {
        if (input[i].as_bytes()[2] as char).is_digit(10) || input[i].len() == 0 { 
            line += 2;
            break; 
        }
        let mut line_to_vec = input[i].chars().collect::<Vec<char>>();
        if line_to_vec[0] != ' ' && line_to_vec[0] != '['  { line_to_vec.remove(0); }
        for j in 0..number_stacks {
            if line_to_vec[1+(j*4)] != ' ' { stacks[j].push(line_to_vec[1+(j*4)]); }
        } 
        line += 1;
    }

    // Do Instructions
    for i in line..input.len() {
        let instruction = input[i].split(" ").collect::<Vec<&str>>();
        let quantity: usize = instruction[1].parse().unwrap();
        let from: usize = instruction[3].parse().unwrap();
        let dest: usize = instruction[5].parse().unwrap();
        for j in 0..quantity{
            let val: char = stacks[from - 1][0];
            stacks[from - 1].remove(0);
            stacks[dest - 1].insert(j, val);
        }
    }
    //println!("{:?}", stacks);
    let mut result = String::new();
    for stack in stacks{
        result += &stack[0].to_string();
    }
    println!("{}", result);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file: String = fs::read_to_string(&args[1]).expect("No se ha podido abrir el archivo.");
    let file_vec: Vec<&str> = file.split("\r").collect();

    part_one(&file_vec);
    part_two(&file_vec);
    //let part_two = part_two(&input);

    //println!("Part 1: {}", part_one);
    //println!("Part 2: {}", part_two);
}