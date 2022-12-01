use std::io::stdin;

fn main() {
    let mut most = 0;
    let mut current = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        if line == "" {
            if current > most {
                most = current;
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    println!("{}", most);
}
