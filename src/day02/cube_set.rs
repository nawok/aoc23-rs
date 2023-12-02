use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct CubeSet {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl CubeSet {
    fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CubeSetParseErr {
    NotAmountColorPair,
    InvalidAmount,
    UnknownColor,
}

impl FromStr for CubeSet {
    type Err = CubeSetParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = CubeSet::empty();
        let parts = s.split(',');
        for part in parts {
            match part.trim().split_once(' ') {
                Some((amount_str, color)) => {
                    let amount = match amount_str.parse() {
                        Ok(value) => value,
                        Err(_) => return Err(CubeSetParseErr::InvalidAmount),
                    };
                    match color {
                        "red" => cube_set.red = amount,
                        "green" => cube_set.green = amount,
                        "blue" => cube_set.blue = amount,
                        _ => return Err(CubeSetParseErr::UnknownColor),
                    }
                }
                None => return Err(CubeSetParseErr::NotAmountColorPair),
            };
        }
        Ok(cube_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_set_from_str_r() {
        let expected = CubeSet {
            red: 1,
            green: 0,
            blue: 0,
        };
        let actual = CubeSet::from_str("1 red").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cube_set_from_str_g() {
        let expected = CubeSet {
            red: 0,
            green: 2,
            blue: 0,
        };
        let actual = CubeSet::from_str("2 green").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cube_set_from_str_b() {
        let expected = CubeSet {
            red: 0,
            green: 0,
            blue: 3,
        };
        let actual = CubeSet::from_str("3 blue").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cube_set_from_str_rg() {
        let expected = CubeSet {
            red: 5,
            green: 4,
            blue: 0,
        };
        let actual = CubeSet::from_str("4 green, 5 red").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cube_set_from_str_gb() {
        let expected = CubeSet {
            red: 0,
            green: 7,
            blue: 6,
        };
        let actual = CubeSet::from_str("6 blue, 7 green").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cube_set_from_str_rb() {
        let expected = CubeSet {
            red: 8,
            green: 0,
            blue: 9,
        };
        let actual = CubeSet::from_str("8 red, 9 blue").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cube_set_from_str_rgb() {
        let expected = CubeSet {
            red: 11,
            green: 12,
            blue: 10,
        };
        let actual = CubeSet::from_str("10 blue, 11 red, 12 green").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cube_set_from_str_err_format() {
        let result = CubeSet::from_str("kaboom");
        assert_eq!(result.unwrap_err(), CubeSetParseErr::NotAmountColorPair);
    }

    #[test]
    fn test_cube_set_from_str_err_amount() {
        let result = CubeSet::from_str("one blue");
        assert_eq!(result.unwrap_err(), CubeSetParseErr::InvalidAmount);
    }

    #[test]
    fn test_cube_set_from_str_err_color() {
        let result = CubeSet::from_str("3 purple");
        assert_eq!(result.unwrap_err(), CubeSetParseErr::UnknownColor);
    }
}
