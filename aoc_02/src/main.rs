use std::fs;
use std::collections::HashMap;

fn points_accum(input: &Vec<&str>, rules: &HashMap<char, HashMap<char, u32>>) -> u32{
    let mut sum = 0;
    for element in input{ sum += rules[&element.chars().nth(0).unwrap()][&element.chars().nth(2).unwrap()]; }
    return sum
}

fn guess_turn(input: &Vec<&str>, 
              rules: &HashMap<char, HashMap<char, u32>>,
              prediction: &HashMap<char, HashMap<char, char>>) -> u32{
    let mut sum = 0;
    for element in input{
        let player_one: &char = &element.chars().nth(0).unwrap();
        let player_two: &char = &element.chars().nth(2).unwrap();
        sum += rules[player_one][&prediction[player_one][player_two]];
    }
    return sum
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file: String = fs::read_to_string(&args[1]).expect("No se ha podido abrir el archivo.");
    let input: Vec<&str> = file.split("\n").collect();

    // Rules and Predictions
    let rules: HashMap<char, HashMap<char, u32>> = HashMap::from([
        ('A', HashMap::from([
            ('X', 1 + 3),
            ('Y', 2 + 6),
            ('Z', 3 + 0)
        ])),
        ('B', HashMap::from([
            ('X', 1 + 0),
            ('Y', 2 + 3),
            ('Z', 3 + 6)
        ])),
        ('C', HashMap::from([
            ('X', 1 + 6),
            ('Y', 2 + 0),
            ('Z', 3 + 3)
        ]))
    ]);
    let prediction: HashMap<char, HashMap<char, char>> = HashMap::from([
        ('A', HashMap::from([
            ('X', 'Z'),
            ('Y', 'X'),
            ('Z', 'Y')
        ])),
        ('B', HashMap::from([
            ('X', 'X'),
            ('Y', 'Y'),
            ('Z', 'Z')
        ])),
        ('C', HashMap::from([
            ('X', 'Y'),
            ('Y', 'Z'),
            ('Z', 'X')
        ]))
    ]);

    let result = points_accum(&input, &rules);
    let result_two = guess_turn(&input, &rules, &prediction);

    println!("Part 1: {}", result);
    println!("Part 2: {}", result_two);
}
