use std::collections::HashMap;

/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
///
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
///
/// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
///
///     I can be placed before V (5) and X (10) to make 4 and 9.
///     X can be placed before L (50) and C (100) to make 40 and 90.
///     C can be placed before D (500) and M (1000) to make 400 and 900.
///
/// Given a roman numeral, convert it to an integer.
///
///
///
/// Example 1:
///
/// Input: s = "III"
/// Output: 3
/// Explanation: III = 3.

// fn is_greater_or_eq(r1: char, r2: char) -> bool {
//     order_map.get(&r1).unwrap() >= order_map.get(&r2).unwrap()
// }

fn roman_to_dec(roman: &str) -> i32 {
    let map = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut num = 0;

    let mut remaining = roman;
    while remaining != "" {
        if let Some(first_char) = remaining.chars().nth(0) {
            if let Some(second_char) = remaining.chars().nth(1) {
                if map.get(&first_char).unwrap() >= map.get(&second_char).unwrap() {
                    num += map.get(&first_char).unwrap();
                    remaining = &remaining[1..];
                } else {
                    num += map.get(&second_char).unwrap() - map.get(&first_char).unwrap();
                    remaining = &remaining[2..];
                }
            } else {
                num += map.get(&first_char).unwrap();
                remaining = &remaining[1..];
            }
        } else {
            break;
        }
    }

    return num;
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("III", 3)]
    #[test_case("LVIII", 58)]
    #[test_case("MCMXCIV", 1994)]
    fn test(roman: &str, out: i32) {
        assert_eq!(roman_to_dec(roman), out);
    }
}
