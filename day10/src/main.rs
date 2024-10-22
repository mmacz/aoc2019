use std::collections::HashSet;
use std::f64::consts::PI;

mod sanity_inputs;
mod input;

fn asteroid_positions(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        // iterating over lines hence going down the map
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                // iterate over characters in lines hence going right to the map
                // move takes ownership
                .filter_map(move |(x, char)| match char {
                    '#' => Some((x as i32, y as i32)),
                    _ => None,
                })
        })
        .collect()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn vec_diff(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    return (b.0 - a.0, b.1 - a.1);
}

fn direction(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let (dx, dy) = vec_diff(a, b);
    let scale_factor: i32 = gcd(dx, dy);

    (dx / scale_factor, dy / scale_factor)
}

fn count_visible(curr_asteroid: (i32, i32), asteroids: &Vec<(i32, i32)>) -> usize {
    let mut directions: HashSet<(i32, i32)> = HashSet::new();
    for &a in asteroids.iter() {
        if curr_asteroid != a {
            directions.insert(direction(a, curr_asteroid));
        }
    }
    directions.len()
}

fn solution1(input: &str) -> ((i32, i32), usize) {
    let positions: Vec<(i32, i32)> = asteroid_positions(input);
    positions
        .iter()
        .map(|&a| {
            (a, count_visible(a, &positions))
        })
        .max_by(|&(_, cnt1), &(_, cnt2)| cnt1.cmp(&cnt2))
        .unwrap()
}

fn get_angle(pt1: (i32, i32), pt2: (i32, i32)) -> f64 {
    let (dx, dy): (i32, i32) = vec_diff(pt1, pt2);
    let mut angle =  (dy as f64).atan2(dx as f64) + PI / 2.0;
    if angle < 0.0 {
        angle += PI * 2.0;
    }
    angle
}

fn get_distance(pt1: (i32, i32), pt2: (i32, i32)) -> f64 {
    let (dx, dy) = vec_diff(pt1, pt2);
    ((dx * dx) as f64 + (dy * dy) as f64).sqrt()
}

fn solution2(input: &str, best_place: (i32, i32)) -> (i32, i32) {
    let asteroids = asteroid_positions(input);
    (0,0)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_diff_vector() {
        assert_eq!(vec_diff((8, 3), (10, 5)), (2, 2));
    }

    #[test]
    fn test_get_angle_above() {
        let angle = get_angle((8, 3), (8, 1));
        assert!((angle - 0.).abs() < std::f64::EPSILON, "Expected: 0 vs Actual: {}", angle);
    }

    #[test]
    fn test_get_angle_right() {
        let angle = get_angle((8, 3), (10, 3));
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < std::f64::EPSILON, "Expected: ~pi/2 vs Actual: {}", angle);
    }

    #[test]
    fn test_get_angle_down() {
        let angle = get_angle((8, 3), (8, 5));
        assert!((angle - std::f64::consts::PI).abs() < std::f64::EPSILON, "Expected: ~pi vs Actual: {}", angle);
    }

    #[test]
    fn test_get_angle_left() {
        let angle = get_angle((8, 3), (6, 3));
        assert!((angle - 3.0 * std::f64::consts::FRAC_PI_2).abs() < std::f64::EPSILON, "Expected: 3pi/2 vs Actual: {}", angle);
    }
}


fn main() {
    assert_eq!(8, solution1(sanity_inputs::INPUT1).1);
    assert_eq!(33, solution1(sanity_inputs::INPUT2).1);
    assert_eq!(210, solution1(sanity_inputs::INPUT3).1);
    let sanity_part_b = solution2(sanity_inputs::INPUT3, (11, 13));
    assert_eq!(802, sanity_part_b.0 * 100 + sanity_part_b.1);

    let answer1 = solution1(input::INPUT);
    println!("Answer 1. Best place: {:?}, visible asteroids: {}", answer1.0, answer1.1);
    let answer2 = solution2(input::INPUT, answer1.0);
    println!("Answer 2. 200th asteroid: {:?}, answer: {}", answer2, answer2.0 * 100 + answer2.1);
}