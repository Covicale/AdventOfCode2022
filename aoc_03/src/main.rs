use std::fs;
use std::collections::HashSet;

fn char_to_val(ch: char) -> u32{
    if ch.is_uppercase() { return ch as u32 - 'A' as u32 + 27; }
    return ch as u32 - 'a' as u32 + 1;
}

fn part_two(input: &Vec<&str>) -> u32{
    let groups: Vec<_> = input.chunks(3).collect();
    let mut sum = 0;
    for group in groups{
        let mut badge: HashSet<char> = group[0].chars().collect();
        for i in 1..group.len() {
            let group_hash: HashSet<char> = group[i].chars().collect();
            badge = badge.intersection(&group_hash).copied().collect();
        }
        for x in badge { sum += char_to_val(x); }
    }
    return sum;
}

fn part_one(input: &Vec<&str>) -> u32{
    let mut sum = 0;
    for element in input{
        let mut values: HashSet<char> = HashSet::new();
        let rucksacks: [&str; 2] = [&element[..(element.len()/2)], &element[element.len()/2..]];
        for val in rucksacks[0].chars(){
            if values.get(&val) == None {
                for other_val in rucksacks[1].chars(){
                    if val == other_val {
                        values.insert(val);
                        break;
                    }
                }
            }
        }
        for val in values { sum += char_to_val(val); }
    }
    return sum;
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file: String = fs::read_to_string(&args[1]).expect("No se ha podido abrir el archivo.");
    let input: Vec<&str> = file.split("\n").collect();
    let part_one = part_one(&input);
    let part_two = part_two(&input);

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}