use std::collections::HashSet;

// #[derive(Debug)]
// struct GameSet {
//     game_id: u32,
//     rounds: Vec<(u32, u32, u32)>, // (r, g, b)
// }

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn solve(input: &str) -> u32 {
    // let mut all_games: Vec<GameSet>; // (r, g, b)
    let mut possible_game_id: HashSet<u32> = HashSet::new();
    for line in input.lines() {
        let games: Vec<&str> = line.split(":").collect();
        let second = games[0].trim().split(" ").nth(1).unwrap();
        let id: u32 = second.parse::<u32>().unwrap();
        let all_shows: Vec<&str> = games[1].split(";").collect();
        // let mut game_set: GameSet = GameSet {
        //     game_id: id,
        //     rounds: Vec::new(),
        // };
        let mut is_possible: bool = true;
        for s in all_shows {
            let shows: Vec<&str> = s.split(";").collect();
            let mut show_tuple: (u32, u32, u32) = (0, 0, 0);
            for s2 in shows {
                let each_show: Vec<&str> = s2.split(",").collect();
                for s3 in each_show {
                    if s3.contains("red") {
                        let first = s3.trim().split(" ").next().unwrap();
                        show_tuple.0 = first.parse::<u32>().unwrap();
                    } else if s3.contains("green") {
                        let first = s3.trim().split(" ").next().unwrap();
                        show_tuple.1 = first.parse::<u32>().unwrap();
                    } else if s3.contains("blue") {
                        let first = s3.trim().split(" ").next().unwrap();
                        show_tuple.2 = first.parse::<u32>().unwrap();
                    } else {
                        panic!("Not red, blue or green: {}", s3)
                    }
                }
            }
            if show_tuple.0 > MAX_RED || show_tuple.1 > MAX_GREEN || show_tuple.2 > MAX_BLUE {
                is_possible = false;
            }
            // game_set.rounds.push(show_tuple);
        }
        if is_possible {
            possible_game_id.insert(id);
        }
        // all_games.push(game_set);
    }
    let mut sum: u32 = 0;
    for i in possible_game_id {
        sum += i;
    }
    sum
}

fn main() {
    let input = include_str!("input1.txt");
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
        assert_eq!(solve(input), 8);
    }
}
