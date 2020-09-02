use num_rational::Rational64;
use std::cmp::{max, min};

/// Finds the index of the local minimum on an interval if it exists
///
/// # Arguments
///
/// * `input` - A slice of rational numbers
/// * `index1` - Left or right index of a closed interval to search within
/// * `index2` - Left or right index of a closed interval to search within
///
/// # Examples
/// 1. There is no local minimum:
/// ```
/// use num_rational::Rational64;
/// use flood::controller::local_minimum::local_minimum;
///
/// let input: Vec<Rational64> = vec![
///     Rational64::from_integer(1),
///     Rational64::from_integer(1),
/// ];
/// let result = local_minimum(&input, 0, 1);
/// assert!(result.is_none());
/// ```
/// 2. There is a local minimum:
/// ```
/// use num_rational::Rational64;
/// use flood::controller::local_minimum::local_minimum;
///
/// let input: Vec<Rational64> = vec![
///     Rational64::from_integer(2),
///     Rational64::from_integer(1),
///     Rational64::from_integer(1),
/// ];
/// let result = local_minimum(&input, 0, 1).unwrap();
/// assert_eq!(result, 1);
/// ```
/// 3. The indexes can be in reverse order:
/// ```
/// use num_rational::Rational64;
/// use flood::controller::local_minimum::local_minimum;
///
/// let input: Vec<Rational64> = vec![
///     Rational64::from_integer(1),
///     Rational64::from_integer(2),
/// ];
/// let result = local_minimum(&input, 1, 0).unwrap();
/// assert_eq!(result, 0);
/// ```
pub fn local_minimum(input: &[Rational64], index1: usize, index2: usize) -> Option<usize> {
    let low = min(index1, index2);
    let high = max(index1, index2);
    let n = input.len();
    let mid = low + (high - low) / 2;

    // Guards
    if mid == low && mid == high {
        return None;
    }
    if mid == 0 || input.get(mid - 1).is_none() || input.get(mid + 1).is_none() {
        let minimum = input
            .iter()
            .enumerate()
            .filter(|v| v.0 >= low && v.0 <= high)
            .map(|v| v.1)
            .min();
        let maximum = input
            .iter()
            .enumerate()
            .filter(|v| v.0 >= low && v.0 <= high)
            .map(|v| v.1)
            .max();
        if minimum == maximum {
            return None;
        }
        return match minimum {
            None => None,
            Some(min) => input.iter().position(|&v| v == *min),
        };
    }

    // Divide and conquer

    // Compare middle element with its neighbours
    if (mid == 0 || input[mid - 1] > input[mid]) && (mid == n - 1 || input[mid + 1] > input[mid]) {
        return Some(mid);
    }
    // If middle element is not minima and its left
    // neighbour is smaller than it, then left half
    // must have a local minima.
    else if mid > 0 && input[mid - 1] < input[mid] {
        return local_minimum(input, low, mid - 1);
    }

    // If middle element is not minima and its right
    // neighbour is smaller than it, then right half
    // must have a local minima.
    local_minimum(input, mid + 1, high)
}
