fn solve(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut last = ' ';
        let mut first = ' ';
        let mut is_first = true;
        for c in line.chars() {
            if c.is_digit(10) {
                if is_first {
                    first = c;
                    last = c;
                    is_first = false;
                } else {
                    last = c;
                }
            }
        }
        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    }
    sum
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solver() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(solve(input), 142);
    }
}
