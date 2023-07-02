/// 67. Add Binary: Given two binary strings a and b, return their sum as a binary string.
///
/// Example 1:
///
/// Input: a = "11", b = "1"
/// Output: "100"

fn bin_add(a: &str, b: &str) -> String {
    let mut carry = 0;

    let mut remaining_a = a.chars().rev();
    let mut remaining_b = b.chars().rev();

    let mut f_a;
    let mut f_b;

    let mut a_filled = a.len() > 0;

    let mut b_filled = b.len() > 0;

    let mut s = String::new();

    while a_filled || b_filled {
        if let Some(first_a) = remaining_a.next() {
            f_a = first_a;
        } else {
            f_a = '0';
            a_filled = false;
        }

        if let Some(first_b) = remaining_b.next() {
            f_b = first_b;
        } else {
            b_filled = false;
            f_b = '0';
        }

        let k = match (f_a, f_b) {
            ('1', '1') => {
                if carry == 1 {
                    carry = 1;
                    '1'
                } else {
                    carry = 1;
                    '0'
                }
            }
            ('1', '0') | ('0', '1') => {
                if carry == 1 {
                    carry = 1;
                    '0'
                } else {
                    carry = 0;
                    '1'
                }
            }
            ('0', '0') => {
                if carry == 1 {
                    carry = 0;
                    '1'
                } else {
                    carry = 0;
                    '0'
                }
            }
            (_, _) => 'a',
        };
        s.push(k);
    }
    let mut j = s.chars().rev();
    while let Some(first_char) = j.next() {
        if first_char == '0' {
            continue;
        } else {
            let mut jj = String::from(first_char);
            jj.push_str(j.collect::<String>().as_str());
            return jj;
        }
    }
    "0".to_string()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("11", "1", "100".to_string())]
    #[test_case("1010", "1011", "10101".to_string())]
    #[test_case("0", "0", "0".to_string())]
    #[test_case("0", "1", "1".to_string())]
    fn test(a: &str, b: &str, out: String) {
        assert_eq!(bin_add(a, b), out);
    }
}
