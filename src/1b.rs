use std::io::stdin;

fn main() {
    let mut elves = stdin()
        .lines()
        .map(Result::unwrap)
        .fold(vec![0], |mut v, line| {
            if line.is_empty() {
                v.push(0);
            } else {
                *v.last_mut().unwrap() += line.parse::<i64>().unwrap();
            }
            v
        });
    elves.sort();
    println!("{}", elves.iter().rev().take(3).sum::<i64>());
}
