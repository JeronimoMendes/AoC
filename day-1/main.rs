use std::fs;

fn main() {
    let mut sum: i32 = 0;
    let mut elves: Vec<i32> = Vec::new();

    let input = fs::read_to_string("input.txt").unwrap();

    for line in input.lines() {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    elves.push(sum);

    // get top 3 values
    // sort reverse order
    elves.sort_by(|a, b| b.cmp(a));
    let top3 = elves[0..3].to_vec();

    println!("{}", top3.iter().sum::<i32>());
}
