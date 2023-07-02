fn sort_arrays(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let mut sorted_arr = vec![];
    let mut p1 = 0;
    let mut p2 = 0;

    while p1 < first.len() && p2 < second.len() {
        if first[p1] < second[p2] {
            sorted_arr.push(first[p1]);
            p1 += 1;
        } else if first[p1] > second[p2] {
            sorted_arr.push(second[p2]);
            p2 += 1;
        } else {
            sorted_arr.push(first[p1]);
            sorted_arr.push(second[p2]);
            p1 += 1;
            p2 += 1;
        }
    }

    if p1 == first.len() {
        sorted_arr.extend_from_slice(&second[p2..]);
    }

    if p2 == second.len() {
        sorted_arr.extend_from_slice(&first[p1..]);
    }

    sorted_arr
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(vec![1, 2, 5, 8], vec![3, 5, 6, 9], vec![1, 2, 3, 5, 5, 6, 8, 9])]
    #[test_case(vec![1], vec![3], vec![1, 3])]
    #[test_case(vec![1], vec![0], vec![0, 1])]
    #[test_case(vec![], vec![0], vec![0])]
    #[test_case(vec![], vec![], vec![])]
    fn test(first: Vec<i32>, second: Vec<i32>, result: Vec<i32>) {
        assert_eq!(sort_arrays(first, second), result);
    }
}
