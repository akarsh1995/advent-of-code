/// 35. Search Insert Position
/// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
/// You must write an algorithm with O(log n) runtime complexity.
/// Example 1:
/// Input: nums = [1,3,5,6], target = 5
/// Output: 2
/// Revise
fn v(x: &[i32], l: usize, h: usize, target: i32) -> usize {
    let m = (h + l) / 2;
    if l == m {
        if target == x[m] {
            return m;
        } else {
            return m + 1;
        }
    }
    if target > x[m] {
        return v(x, m, h, target);
    } else if target < x[m] {
        return v(x, l, m, target);
    } else {
        return m;
    }
}

fn ins_pos(x: &[i32], target: i32) -> i32 {
    if x.len() == 1 {
        return {
            if x[0] >= target {
                0
            } else {
                1
            }
        };
    }
    let l = 0;
    let h = x.len() - 1;
    if target > x[h] {
        return x.len() as i32;
    } else if target < x[l] {
        return 0;
    }
    return v(x, l, h, target) as i32;
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(vec![1, 3, 5, 6], 5, 2)]
    #[test_case(vec![1, 3, 5, 6], 2, 1)]
    #[test_case(vec![1, 3, 5, 6], 7, 4)]
    #[test_case(vec![2], 1, 0)]
    #[test_case(vec![1], 2, 1)]
    #[test_case(vec![1, 2], 2, 1)]
    #[test_case(vec![1, 2, 3], 3, 2)]
    #[test_case(vec![1, 3, 5], 4, 2)]
    fn test_pal_num(x: Vec<i32>, target: i32, index: i32) {
        assert_eq!(ins_pos(x.as_slice(), target), index);
    }
}
