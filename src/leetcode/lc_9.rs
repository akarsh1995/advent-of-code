/// 9. Palindrome Number
/// Given an integer x, return true if x is a palindrome, and false otherwise.

fn palindrome_number(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x % 10 == 0 {
        return false;
    }

    let mut op = x.clone();

    let mut reverse = 0;

    while op != 0 {
        let unit_digit = op % 10;
        reverse = reverse * 10 + unit_digit;
        op /= 10;
    }

    reverse == x
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(121, true)]
    #[test_case(-121, false)]
    #[test_case(10, false)]
    fn test_pal_num(x: i32, res: bool) {
        assert_eq!(palindrome_number(x), res);
    }
}
