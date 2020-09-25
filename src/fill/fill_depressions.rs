use std::collections::BinaryHeap;
use std::ops::AddAssign;

use num_rational::Rational64;

use crate::fill::minima::minima;
use crate::fill::point::Point;

/// Priority queue based algorithm inspired by
/// "Another Fast and Simple DEM Depression-Filling Algorithm Based on Priority Queue Structure"
/// by LIU Yong-He, ZHANG Wan-Chang, XU Jing-Wen
pub fn fill_depressions(input: &[Point]) -> Vec<Rational64> {
    let mut binary_heap = BinaryHeap::new();
    let mut water_levels = vec![];
    for point in input {
        binary_heap.push(point);
        water_levels.push(point.elevation);
    }
    loop {
        // Traverse from the highest point downwards
        let result = binary_heap.pop();
        match result {
            None => break,
            Some(current) => {
                let current_index = current.index as usize;
                println!("Current index {:?}", current_index);
                let minima = minima(&water_levels, current_index);
                let count = minima.len() as i64;
                for index_of_minimum in minima {
                    let amount = Rational64::new(1 as i64, count);
                    println!(
                        "Filling local minimum at index {:?} {:?}",
                        index_of_minimum, amount
                    );
                    water_levels[index_of_minimum].add_assign(amount);
                }
            }
        }
    }
    water_levels
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_vec(input: Vec<u64>) -> Vec<Point> {
        input
            .iter()
            .enumerate()
            .map(|(index, elevation)| Point {
                index: index as u64,
                elevation: Rational64::from_integer(*elevation as i64),
            })
            .collect()
    }

    #[test]
    fn simple_valley() {
        let results = fill_depressions(&from_vec(vec![3, 0, 3]));
        assert_eq!(
            results,
            vec![
                Rational64::from_integer(3),
                Rational64::from_integer(3),
                Rational64::from_integer(3)
            ]
        );
    }

    #[test]
    fn simple_steps() {
        let results = fill_depressions(&from_vec(vec![3, 2, 1]));
        assert_eq!(
            results,
            vec![
                Rational64::from_integer(3),
                Rational64::from_integer(3),
                Rational64::from_integer(3)
            ]
        );
    }

    #[test]
    fn simple_plain() {
        let results = fill_depressions(&from_vec(vec![1, 1, 1]));
        assert_eq!(
            results,
            vec![
                Rational64::from_integer(2),
                Rational64::from_integer(2),
                Rational64::from_integer(2)
            ]
        );
    }

    #[test]
    fn simple_castle() {
        let results = fill_depressions(&from_vec(vec![1, 0, 1, 0, 1]));
        assert_eq!(
            results,
            vec![
                Rational64::new(8, 5),
                Rational64::new(8, 5),
                Rational64::new(8, 5),
                Rational64::new(8, 5),
                Rational64::new(8, 5),
            ]
        );
    }

    #[test]
    fn example() {
        let results = fill_depressions(&from_vec(vec![3, 1, 6, 4, 8, 9]));
        assert_eq!(
            results,
            vec![
                Rational64::from_integer(4),
                Rational64::from_integer(4),
                Rational64::from_integer(6),
                Rational64::from_integer(6),
                Rational64::from_integer(8),
                Rational64::from_integer(9)
            ]
        );
    }
}
