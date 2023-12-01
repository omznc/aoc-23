fn main() {
    println!("{}", include_str!("../../input.txt")
        .split("\n\n")
        .map(|line| line.split_whitespace()
            .map(|word| {
                let digits: Vec<_> = word.chars().filter(|c| c.is_digit(10)).collect();
                match digits.len() {
                    0 | 1 => format!("{0}{0}", digits[0]).parse::<i32>().unwrap(),
                    _ => format!("{}{}", digits[0], digits[digits.len() - 1]).parse::<i32>().unwrap(),
                }
            }).sum::<i32>())
        .sum::<i32>());
}