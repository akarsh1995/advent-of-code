/// 6. The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
///
// P   A   H   N
// A P L S I I G
// Y   I   R
// And then read line by line: "PAHNAPLSIIGYIR"

pub fn zig_zag(x: &str, n: usize) -> String {
    let mut result = vec![vec![]; n];

    let mut dir = 1;
    let mut i = 0;

    for c in x.chars() {
        result[i].push(c);

        if dir == -1 {
            i -= 1;
        } else {
            i += 1;
        }

        if i == n - 1 {
            dir = -1;
        }

        if i == 0 {
            dir = 1;
        }
    }

    result.iter().flat_map(|r| r.iter()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!("PAHNAPLSIIGYIR".to_string(), zig_zag("PAYPALISHIRING", 3));
        assert_eq!("PINALSIGYAHRPI".to_string(), zig_zag("PAYPALISHIRING", 4));
    }
}
