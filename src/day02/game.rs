use std::str::FromStr;

use crate::day02::cube_set::CubeSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: usize,
    pub handfuls: Vec<CubeSet>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameParseErr {
    NotIdHandfulsPair,
    InvalidGameId,
    IncorrectHandfuls,
}

impl FromStr for Game {
    type Err = GameParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((game_id, handfuls_str)) => Ok(Self {
                id: match game_id.split_once(' ') {
                    Some((_, id)) => match id.parse() {
                        Ok(value) => value,
                        Err(_) => return Err(GameParseErr::InvalidGameId),
                    },
                    None => return Err(GameParseErr::InvalidGameId),
                },
                handfuls: match handfuls_str
                    .split(';')
                    .map(CubeSet::from_str)
                    .collect::<Result<Vec<_>, _>>()
                {
                    Ok(val) => val,
                    Err(_) => return Err(GameParseErr::IncorrectHandfuls),
                },
            }),
            None => Err(GameParseErr::NotIdHandfulsPair),
        }
    }
}

impl Game {
    pub fn is_possible(&self, bag: &CubeSet) -> bool {
        self.handfuls.iter().all(|handful| {
            handful.red <= bag.red && handful.green <= bag.green && handful.blue <= bag.blue
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_str_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: 1,
            handfuls: vec![
                CubeSet {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                CubeSet {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                CubeSet {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };
        let actual = Game::from_str(input).unwrap();
        assert_eq!(expected, actual);

        let bag = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert!(actual.is_possible(&bag));
    }

    #[test]
    fn test_game_from_str_2() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = Game {
            id: 2,
            handfuls: vec![
                CubeSet {
                    red: 0,
                    green: 2,
                    blue: 1,
                },
                CubeSet {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                CubeSet {
                    red: 0,
                    green: 1,
                    blue: 1,
                },
            ],
        };
        let actual = Game::from_str(input).unwrap();
        assert_eq!(expected, actual);

        let bag = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert!(actual.is_possible(&bag));
    }

    #[test]
    fn test_game_from_str_3() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let expected = Game {
            id: 3,
            handfuls: vec![
                CubeSet {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                CubeSet {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                CubeSet {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ],
        };
        let actual = Game::from_str(input).unwrap();
        assert_eq!(expected, actual);

        let bag = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert!(!actual.is_possible(&bag));
    }

    #[test]
    fn test_game_from_str_4() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let expected = Game {
            id: 4,
            handfuls: vec![
                CubeSet {
                    red: 3,
                    green: 1,
                    blue: 6,
                },
                CubeSet {
                    red: 6,
                    green: 3,
                    blue: 0,
                },
                CubeSet {
                    red: 14,
                    green: 3,
                    blue: 15,
                },
            ],
        };
        let actual = Game::from_str(input).unwrap();
        assert_eq!(expected, actual);

        let bag = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert!(!actual.is_possible(&bag));
    }

    #[test]
    fn test_game_from_str_5() {
        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = Game {
            id: 5,
            handfuls: vec![
                CubeSet {
                    red: 6,
                    green: 3,
                    blue: 1,
                },
                CubeSet {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ],
        };
        let actual = Game::from_str(input).unwrap();
        assert_eq!(expected, actual);

        let bag = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert!(actual.is_possible(&bag));
    }

    #[test]
    fn test_game_from_str_not_id_handfuls_pair() {
        let input = "Game 6 - 1 red, 2 green, 3 blue";
        let result = Game::from_str(input);
        assert_eq!(result.unwrap_err(), GameParseErr::NotIdHandfulsPair);
    }

    #[test]
    fn test_game_from_str_invalid_game_id_format() {
        let input = "Game7: 1 red, 2 green, 3 blue";
        let result = Game::from_str(input);
        assert_eq!(result.unwrap_err(), GameParseErr::InvalidGameId);
    }

    #[test]
    fn test_game_from_str_invalid_game_id() {
        let input = "Game 8b: 1 red, 2 green, 3 blue";
        let result = Game::from_str(input);
        assert_eq!(result.unwrap_err(), GameParseErr::InvalidGameId);
    }

    #[test]
    fn test_game_from_str_incorrect_handfuls() {
        let input = "Game 9: 1 red, 2 green, 3 purple";
        let result = Game::from_str(input);
        assert_eq!(result.unwrap_err(), GameParseErr::IncorrectHandfuls);
    }
}
