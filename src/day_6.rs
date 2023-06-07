use std::collections::HashSet;

const INPUT: &'static str = include_str!("../data/year_2022__day_6");

fn get_marker(sample_string: &str) -> u32 {
    let zipped = sample_string.chars().zip(
        sample_string.chars().skip(1).zip(
            sample_string
                .chars()
                .skip(2)
                .zip(sample_string.chars().skip(3)),
        ),
    );
    let mut marker = 0;
    for (i, (a, (b, (c, d)))) in zipped.enumerate() {
        if [a, b, c, d].iter().collect::<HashSet<_>>().len() == 4 {
            marker = i + 4;
            break;
        }
    }
    marker as u32
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    #[test]
    fn test_p_1() {
        let sample_string = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(get_marker(sample_string), 11);
        let sample_string = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(get_marker(sample_string), 5);
        dbg!(get_marker(INPUT));
    }

    #[test]
    fn test_p_2() {}
}
