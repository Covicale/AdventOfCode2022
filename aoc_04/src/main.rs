use std::fs;

fn part_two(input: &Vec<[[u32; 2]; 2]>) -> u32 {
    let mut sum = 0;
    for pair in input {
        if ( pair[0][0] >= pair[1][0] && pair[0][0] <= pair[1][1] )
            || ( pair[1][0] >= pair[0][0] && pair[1][0] <= pair[0][1] ) { sum += 1; }
    }
    return sum;
}

fn part_one(input: &Vec<[[u32; 2]; 2]>) -> u32 {
    let mut sum = 0;
    for pair in input {
        if ( pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1] )
            || ( pair[1][0] >= pair[0][0] && pair[1][1] <= pair[0][1] ) { sum += 1; }
    }
    return sum;
}

fn get_vec(fs: &String) -> Vec<[[u32; 2]; 2]>{
    let mut input: Vec<[[u32; 2]; 2]> = Vec::new();
    let base_input: Vec<&str> = fs.split("\n").collect();
    for element in base_input{
        let mut values: [[u32; 2]; 2] = [[0, 0], [0, 0]];
        let sections: Vec<&str> = element.split(",").collect(); 
        for i in 0..sections.len(){
            let pairs: Vec<&str> = sections[i].split("-").collect(); 
            for j in 0..pairs.len(){
                println!("{}", pairs[j]);
                values[i][j] = pairs[j].parse().unwrap();
            }      
        }
        input.push(values.clone());
    }
    return input;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file: String = fs::read_to_string(&args[1]).expect("No se ha podido abrir el archivo.");
    let input: Vec<[[u32; 2]; 2]> = get_vec(&file);
    let part_one = part_one(&input);
    let part_two = part_two(&input);

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}