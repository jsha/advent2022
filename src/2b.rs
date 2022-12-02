use std::io::stdin;

fn main() {
    const rock: i32 = 1;
    const paper: i32 = 2;
    const scissors: i32 = 3;
    const lose: i32 = 0;
    const draw: i32 = 3;
    const win: i32 = 6;

    let sum = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|ref line| match line.as_str() {
            "A X" => scissors + lose,
            "A Y" => rock + draw,
            "A Z" => paper + win,
            "B X" => rock + lose,
            "B Y" => paper + draw,
            "B Z" => scissors + win,
            "C X" => paper + lose,
            "C Y" => scissors + draw,
            "C Z" => rock + win,
            _ => panic!("invalid input {}", line),
        })
        .sum::<i32>();
    println!("{}", sum);
}
