// one, two, three, four, five, six, seven, eight, nine
fn get_number(number: &str) -> Option<char> {
    match number {
        n if n.starts_with("one") => Some('1'),
        n if n.starts_with("two") => Some('2'),
        n if n.starts_with("three") => Some('3'),
        n if n.starts_with("four") => Some('4'),
        n if n.starts_with("five") => Some('5'),
        n if n.starts_with("six") => Some('6'),
        n if n.starts_with("seven") => Some('7'),
        n if n.starts_with("eight") => Some('8'),
        n if n.starts_with("nine") => Some('9'),
        _ => None,
    }
}

fn solve(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut last = ' ';
        let mut first = ' ';
        let mut is_first = true;
        for i in 0..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c.is_digit(10) {
                if is_first {
                    first = c;
                    last = c;
                    is_first = false;
                } else {
                    last = c;
                }
            } else {
                let num = get_number(&line[i..]);
                if num.is_some() {
                    if is_first {
                        first = num.unwrap();
                        last = num.unwrap();
                        is_first = false;
                    } else {
                        last = num.unwrap();
                    }
                }
            }
        }
        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    }
    sum
}

fn main() {
    let input = include_str!("input2.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solver() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(solve(input), 281);
    }
}
