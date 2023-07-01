/// 1232. Check If It Is a Straight Line
/// You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.

pub fn is_straight_line(v: Vec<Vec<i32>>) -> bool {
    if v.len() == 2 {
        return true;
    }

    let (x1, y1) = (v[0][0], v[0][1]);
    let (x2, y2) = (v[1][0], v[1][1]);

    if x1 == x2 {
        return v.iter().all(|a| a[0] == x1);
    } else if y1 == y2 {
        return v.iter().all(|a| a[1] == y1);
    }

    let slope: f32 = (y2 - y1) as f32 / (x2 - x1) as f32;
    let constant = y1 as f32 - slope * x1 as f32;

    for elem in v.iter() {
        let x = elem[0] as f32;
        let y = elem[1] as f32;
        if y != slope * x + constant {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn straight_line() {
        let v = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
        ];
        assert_eq!(false, is_straight_line(v));
        let v = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        assert_eq!(true, is_straight_line(v));

        let v = vec![
            vec![0, 2],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
            vec![0, 6],
            vec![0, 7],
        ];
        assert_eq!(true, is_straight_line(v));

        let v = vec![
            vec![0, 2],
            vec![1, 3],
            vec![0, 4],
            vec![0, 5],
            vec![0, 6],
            vec![0, 7],
        ];
        assert_eq!(false, is_straight_line(v));
    }
}
