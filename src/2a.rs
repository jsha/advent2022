use std::io::stdin;

fn main() {
    let sum = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|ref line| match line.as_str() {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => panic!("invalid input {}", line),
        })
        .sum::<i32>();
    println!("{}", sum);
}
