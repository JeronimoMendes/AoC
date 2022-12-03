use std::fs;
use std::char;

fn calculate_points(opp: char, me: char) -> i32 {
    if opp == 'A' && me == 'X' {
        return 3;
    } else if opp == 'A' && me == 'Z' {
        return 2 + 6;
    } else if opp == 'A' && me == 'Y' {
        return 1 + 3;
    } else if opp == 'B' && me == 'Z' {
        return 6 + 3;
    } else if opp == 'B' && me == 'Y' {
        return 2 + 3;
    } else if opp == 'B' && me == 'X' {
        return 1;
    } else if opp == 'C' && me == 'Y' {
        return 3 + 3;
    } else if opp == 'C' && me == 'X' {
        return 2;
    } else {
        return 6 + 1;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum: i32 = 0;

    for line in input.lines() {
        sum += calculate_points(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
    }

    println!("{}", sum);
    
}
