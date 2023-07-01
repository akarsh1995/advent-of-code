/// 3. Longest Substring Without Repeating Characters
/// Given a string s, find the length of the longest substring without repeating characters.
use std::collections::VecDeque;

fn length_of_longest(x: &str) -> u32 {
    if x.len() == 1 {
        return 1;
    } else if x.len() == 2 {
        return if x.chars().nth(0).unwrap() == x.chars().nth(1).unwrap() {
            1
        } else {
            2
        };
    }

    let mut tracker = VecDeque::new();
    let mut total: u32 = 0;

    for char in x.chars() {
        if tracker.contains(&char) {
            while tracker.pop_front().unwrap() != char {}
        }
        tracker.push_back(char);
        if tracker.len() > total as usize {
            total = tracker.len() as u32;
        }
    }

    return total;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(3, length_of_longest("pwwkew"));
        assert_eq!(1, length_of_longest("bbbbbbbb"));
        assert_eq!(3, length_of_longest("abcabcbb"));
        assert_eq!(1, length_of_longest(" "));
        assert_eq!(2, length_of_longest("au"));
        assert_eq!(3, length_of_longest("dvdf"))
    }
}
