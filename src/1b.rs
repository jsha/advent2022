use std::io::stdin;

fn main() {
    let mut elves = vec![];
    let mut current = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        if line == "" {
            elves.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    if current != 0 {
        elves.push(current);
    }
    elves.sort();
    let sum = elves.iter().rev().take(3).fold(0, |c, n| c + n);
    println!("{}", sum);
}
