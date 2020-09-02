use num_rational::Rational64;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub index: u64,
    pub elevation: Rational64,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.elevation, &self.index).cmp(&(other.elevation, &other.index))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_is_respected() {
        let point1 = Point {
            index: 0,
            elevation: Rational64::from_integer(1),
        };
        let point2 = Point {
            index: 2,
            elevation: Rational64::from_integer(1),
        };
        assert!(point1 < point2);
    }

    #[test]
    fn elevation_is_respected() {
        let point1 = Point {
            index: 0,
            elevation: Rational64::from_integer(1),
        };
        let point2 = Point {
            index: 0,
            elevation: Rational64::from_integer(2),
        };
        assert!(point1 < point2);
    }

    #[test]
    fn elevation_takes_precedence() {
        let point1 = Point {
            index: 0,
            elevation: Rational64::from_integer(2),
        };
        let point2 = Point {
            index: 1,
            elevation: Rational64::from_integer(1),
        };
        assert!(point1 > point2);
    }
}
