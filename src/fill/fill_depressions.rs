use std::collections::BinaryHeap;
use std::ops::AddAssign;

use num_rational::Rational64;

use crate::fill::local_minimum::local_minimum;
use crate::fill::point::Point;

/// Priority queue based algorithm inspired by
/// "Another Fast and Simple DEM Depression-Filling Algorithm Based on Priority Queue Structure"
/// by LIU Yong-He, ZHANG Wan-Chang, XU Jing-Wen
/// http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.464.4511&rep=rep1&type=pdf
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
                // Next highest point
                let next = binary_heap.peek();
                println!("Current: {:?}\nNext:    {:?}", current, next);
                if let Some(next_peak) = next {
                    let next_index = next_peak.index as usize;
                    // Fill local minimum between 2 elevation points
                    let local_minimum = local_minimum(&water_levels, current_index, next_index);
                    if let Some(index_of_minimum) = local_minimum {
                        println!("Filling local minimum at index {:?}", index_of_minimum);
                        water_levels[index_of_minimum].add_assign(Rational64::from_integer(1));
                    } else {
                        fill_global_minimum(&mut water_levels);
                    }
                } else {
                    fill_global_minimum(&mut water_levels);
                }
            }
        }
    }
    water_levels
}

fn fill_global_minimum(water_levels: &mut Vec<Rational64>) {
    let global_minimum = &water_levels.iter().min().cloned();
    if let Some(value_of_minimum) = global_minimum {
        let count = water_levels
            .iter()
            .filter(|v| **v == *value_of_minimum)
            .count();
        let ratio = Rational64::new(1 as i64, count as i64);
        println!(
            "Global minimum is {:?}",
            *value_of_minimum.numer() as f64 / *value_of_minimum.denom() as f64
        );
        println!(
            "Filling global minimums with {:?}",
            *ratio.numer() as f64 / *ratio.denom() as f64
        );
        for water_level in water_levels.iter_mut() {
            if *water_level == *value_of_minimum {
                water_level.add_assign(ratio);
            }
        }
    }
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
