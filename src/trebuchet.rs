pub fn get_sum_calibration_values(input: &str) -> i32 {
    input.lines().map(get_calibration_value).sum()
}

fn get_calibration_value(line: &str) -> i32 {
    let mut left_val = -1;
    let mut right_val = -1;

    let mut left_idx = 0;
    let mut right_idx = line.len() - 1;

    while left_idx <= right_idx && (left_val < 0 || right_val < 0) {
        let left_char = line.chars().nth(left_idx).unwrap();
        if left_char.is_ascii_digit() {
            left_val = left_char.to_digit(10).unwrap() as i32;
        } else {
            left_idx += 1;
        }

        let right_char = line.chars().nth(right_idx).unwrap();
        if right_char.is_ascii_digit() {
            right_val = right_char.to_digit(10).unwrap() as i32;
        } else {
            right_idx -= 1;
        }
    }

    10 * left_val + right_val
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_get_calibration_value_1() {
        let actual = get_calibration_value("1abc2");
        assert_eq!(12, actual);
    }

    #[test]
    fn test_get_calibration_value_2() {
        let actual = get_calibration_value("pqr3stu8vwx");
        assert_eq!(38, actual);
    }

    #[test]
    fn test_get_calibration_value_3() {
        let actual = get_calibration_value("a1b2c3d4e5f");
        assert_eq!(15, actual);
    }

    #[test]
    fn test_get_calibration_value_4() {
        let actual = get_calibration_value("treb7uchet");
        assert_eq!(77, actual);
    }

    #[test]
    fn test_get_sum_calibration_values_1() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        let actual = get_sum_calibration_values(input);
        assert_eq!(142, actual);
    }
}
