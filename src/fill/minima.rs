use num_rational::Rational64;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn minima(input: &[Rational64], index: usize) -> Vec<usize> {
    let mut left = find_minima(input, index, Direction::Left);
    let mut right = find_minima(input, index, Direction::Right);
    if let Some(left_last) = left.first().cloned() {
        if let Some(right_last) = right.first().cloned() {
            if input[left_last] < input[right_last] {
                right = vec![];
            }
            if input[left_last] > input[right_last] {
                left = vec![];
            }
        }
    }
    let result = [left, right].concat();
    let deduped: HashSet<&usize> = HashSet::from_iter(result.iter());
    let result = Vec::from_iter(deduped.iter().map(|v| **v));
    result
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn find_minima(input: &[Rational64], index: usize, direction: Direction) -> Vec<usize> {
    let mut stack = Vec::with_capacity(input.len());
    let mut i = index as i64;
    let i_min = match direction {
        Direction::Left => 1,
        Direction::Right => 0,
    };
    let i_max = match direction {
        Direction::Left => input.len(),
        Direction::Right => input.len() - 1,
    } as i64;
    stack.push(index);
    while i >= i_min && i < i_max {
        let i_next = match direction {
            Direction::Left => i - 1,
            Direction::Right => i + 1,
        };
        let current = input[i as usize];
        let next = input[i_next as usize];
        if let Some(last) = stack.last().cloned() {
            while !stack.is_empty() && input[last] > next {
                stack.pop();
            }
        }
        if next > current {
            return stack;
        } else {
            stack.push(i_next as usize);
        }
        match direction {
            Direction::Left => i -= 1,
            Direction::Right => i += 1,
        };
    }
    stack
}
