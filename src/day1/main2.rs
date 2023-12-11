// one, two, three, four, five, six, seven, eight, nine
fn get_number(number: &str) -> char {
    match number {
        n if n.starts_with("one") => '1',
        n if n.starts_with("two") => '2',
        n if n.starts_with("three") => '3',
        n if n.starts_with("four") => '4',
        n if n.starts_with("five") => '5',
        n if n.starts_with("six") => '6',
        n if n.starts_with("seven") => '7',
        n if n.starts_with("eight") => '8',
        n if n.starts_with("nine") => '9',
        _ => '0',
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
                if num != '0' {
                    if is_first {
                        first = num;
                        last = num;
                        is_first = false;
                    } else {
                        last = num;
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
