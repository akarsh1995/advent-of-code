/// 28. Find the Index of the First Occurrence in a String
/// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
///
/// Example 1:
/// Input: haystack = "sadbutsad", needle = "sad"
/// Output: 0
/// Explanation: "sad" occurs at index 0 and 6.
/// The first occurrence is at index 0, so we return 0.

fn first_index(haystack: &str, needle: &str) -> i32 {
    let mut remaining = haystack;
    let mut global_index = 0;

    loop {
        let mut flag = false;
        let mut started_matching_at = None;
        for n in needle.chars() {
            if let Some(first_char) = remaining.chars().nth(0) {
                if n != first_char {
                    if let Some(sm) = started_matching_at {
                        remaining = &haystack[sm..];
                        global_index = sm;
                    }
                    remaining = &remaining[1..];
                    global_index += 1;
                    flag = false;
                    break;
                } else {
                    if started_matching_at.is_none() {
                        started_matching_at = Some(global_index);
                    }
                    remaining = &remaining[1..];
                    global_index += 1;
                    flag = true;
                }
            } else {
                return -1;
            }
        }
        if flag {
            return started_matching_at.unwrap() as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("sadbutsad", "sad", 0)]
    #[test_case("leetcode", "leeto", -1)]
    #[test_case("akarsh", "rs", 3)]
    // issip
    #[test_case("mississippi", "issip", 4)]

    fn test_first_index(haystack: &str, needle: &str, index: i32) {
        assert_eq!(first_index(haystack, needle), index);
    }
}
