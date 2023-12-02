use std::str::FromStr;

use crate::day02::cube_set::CubeSet;
use crate::day02::game::Game;

pub fn solve(input: &str) -> usize {
    let games = input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .collect();

    let bag = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    get_sum_possible_game_ids(games, &bag)
}

fn get_sum_possible_game_ids(games: Vec<Game>, bag: &CubeSet) -> usize {
    games
        .iter()
        .filter(|game| game.is_possible(bag))
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(8, solve(input))
    }
}
