use regex::Regex;
use std::collections::HashMap;

pub fn run_a(input: &Vec<String>) -> i64 {
    let (num_players, last_marble) = parse_input(input);

    println!("{} players, {} last marble", num_players, last_marble);

    play_game(num_players, last_marble)
}

pub fn run_b(input: &Vec<String>, multiplier: i32) -> i64 {
    let (num_players, last_marble) = parse_input(input);

    println!("{} players, {} last marble, {} multiplier", num_players, last_marble, multiplier);

    play_game(num_players, last_marble * multiplier)
}

fn parse_input(input: &Vec<String>) -> (i32, i32) {
    let pattern = Regex::new("(\\d+) players; last marble is worth (\\d+) points").unwrap();

    let line = input.first().unwrap();

    if !pattern.is_match(line) {
        panic!("Line doesn't match pattern");
    }

    let captures = pattern.captures(line).unwrap();

    let num_players = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let last_marble = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

    (num_players, last_marble)
}

fn play_game(num_players: i32, last_marble: i32) -> i64 {
    let mut circle = Vec::with_capacity(last_marble as usize);
    circle.push(0);
    circle.push(2);
    circle.push(1);

    let mut player_scores: HashMap<i32, i64> = HashMap::new();

    let mut current_player = 2;
    let mut current_marble_index : usize = 1;

    for i in 3..last_marble + 1 {
        if i % 23 == 0 {
            let player_score = player_scores.entry(current_player).or_insert(0);
            *player_score += i as i64;

            let mut remove_index : i32 = current_marble_index as i32 - 7;
            if remove_index < 0 {
                remove_index += circle.len() as i32;
            }

            let remove_index = remove_index as usize;

            let removed_marble : i32 = *circle.get(remove_index).unwrap();

            *player_score += removed_marble as i64;
            circle.remove(remove_index as usize);

            current_marble_index = remove_index;

            debug!("Removed marble {} from index {}", removed_marble, remove_index)

        } else {
            let circle_len = circle.len();

            let mut next_marble_index = current_marble_index + 2;

            if next_marble_index == circle_len {
                circle.push(i);
            } else {
                next_marble_index = next_marble_index % circle_len;
                circle.insert(next_marble_index, i);
            }

            current_marble_index = next_marble_index;
        }

        current_player = (current_player + 1) % num_players;
        debug!("Changing to player {} for marble {}", current_player, i + 1);
    }

    let mut winning_player = -1;
    let mut max_score = 0;

    for (player, score) in player_scores.iter() {
        let player = *player;
        let score = *score;

        if score > max_score {
            winning_player = player;
            max_score = score;
        }
    }

    println!("Player {} won with a score of {}", winning_player + 1, max_score);

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![String::from("9 players; last marble is worth 25 points")];

        assert_eq!(32, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![String::from("10 players; last marble is worth 1618 points")];

        assert_eq!(8317, run_a(&input));
    }

    #[test]
    pub fn sample_input_2_a() {
        let input = vec![String::from("13 players; last marble is worth 7999 points")];

        assert_eq!(146373, run_a(&input));
    }

    #[test]
    pub fn sample_input_3_a() {
        let input = vec![String::from("17 players; last marble is worth 1104 points")];

        assert_eq!(2764, run_a(&input));
    }

    #[test]
    pub fn sample_input_4_a() {
        let input = vec![String::from("21 players; last marble is worth 6111 points")];

        assert_eq!(54718, run_a(&input));
    }

    #[test]
    pub fn sample_input_5_a() {
        let input = vec![String::from("30 players; last marble is worth 5807 points")];

        assert_eq!(37305, run_a(&input));
    }
}