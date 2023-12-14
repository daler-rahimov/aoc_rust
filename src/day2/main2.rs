fn solve(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let games: Vec<&str> = line.split(":").collect();
        let all_shows: Vec<&str> = games[1].split(';').collect();
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;
        for s in all_shows {
            let shows: Vec<&str> = s.split(';').collect();
            for s2 in shows {
                let each_show: Vec<&str> = s2.split(',').collect();
                for s3 in each_show {
                    if s3.contains("red") {
                        let first = s3.trim().split(' ').next().unwrap();
                        let local_max_red = first.parse::<u32>().unwrap();
                        if local_max_red > max_red {
                            max_red = local_max_red;
                        }
                    } else if s3.contains("green") {
                        let first = s3.trim().split(' ').next().unwrap();
                        let local_max_green = first.parse::<u32>().unwrap();
                        if local_max_green > max_green {
                            max_green = local_max_green;
                        }
                    } else if s3.contains("blue") {
                        let first = s3.trim().split(' ').next().unwrap();
                        let local_max_blue = first.parse::<u32>().unwrap();
                        if local_max_blue > max_blue {
                            max_blue = local_max_blue;
                        }
                    } else {
                        panic!("Not red, blue or green: {}", s3)
                    }
                }
            }
        }
        sum += max_blue * max_green * max_red;
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
        // let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green "#;
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(solve(input), 2286);
    }
}
