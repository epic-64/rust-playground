use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use Peano::{Succ, Zero};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Peano {
    Zero,
    Succ(Box<Peano>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PeanoError {
    DivisionByZero,
}

impl Peano {
    pub fn from_int(n: i64) -> Peano {
        if n <= 0 {
            Zero
        } else {
            Succ(Box::new(Peano::from_int(n - 1)))
        }
    }

    pub fn to_int(&self) -> i64 {
        match self {
            Zero => 0,
            Succ(n) => 1 + n.to_int(),
        }
    }

    pub fn add(&self, other: &Peano) -> Peano {
        match self {
            Zero => other.clone(),
            Succ(n) => Succ(Box::new(n.add(other))),
        }
    }

    pub fn sub(&self, other: &Peano) -> Peano {
        match (self, other) {
            (Zero, _) => Zero,
            (Succ(_), Zero) => self.clone(),
            (Succ(n1), Succ(n2)) => n1.sub(n2),
        }
    }

    pub fn mul(&self, other: &Peano) -> Peano {
        match self {
            Zero => Zero,
            Succ(n) => n.mul(other).add(other),
        }
    }

    pub fn even(&self) -> bool {
        match self {
            Zero => true,
            Succ(n) => !n.even(),
        }
    }

    pub fn odd(&self) -> bool {
        match self {
            Zero => false,
            Succ(n) => n.even(),
        }
    }

    pub fn div(&self, other: &Peano) -> Result<Peano, PeanoError> {
        match self {
            Zero => Ok(Zero),
            Succ(_n) => {
                if other == &Zero {
                    Err(PeanoError::DivisionByZero)
                } else {
                    let mut count = Zero;
                    let mut remainder = self.clone();

                    while remainder.compare(other) == Greater || remainder.compare(other) == Equal {
                        remainder = remainder.sub(other);
                        count = count.add(&Succ(Box::new(Zero)));
                    }

                    Ok(count)
                }
            }
        }
    }

    pub fn compare(&self, other: &Peano) -> Ordering {
        match (self, other) {
            (Zero, Zero) => Equal,
            (Zero, _) => Less,
            (_, Zero) => Greater,
            (Succ(n1), Succ(n2)) => n1.compare(n2), // actually walks backwards until n1 or n2 is Zero
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_int() {
        assert_eq!(Peano::from_int(0), Zero);
        assert_eq!(Peano::from_int(1), Succ(Box::new(Zero)));
        assert_eq!(Peano::from_int(2), Succ(Box::new(Succ(Box::new(Zero)))));
        assert_eq!(Peano::from_int(3), Succ(Box::new(Succ(Box::new(Succ(Box::new(Zero)))))));
    }

    #[test]
    fn test_to_int() {
        assert_eq!(Zero.to_int(), 0);
        assert_eq!(Succ(Box::new(Zero)).to_int(), 1);
        assert_eq!(Succ(Box::new(Succ(Box::new(Zero)))).to_int(), 2);
        assert_eq!(Succ(Box::new(Succ(Box::new(Succ(Box::new(Zero)))))).to_int(), 3);
    }

    #[test]
    fn test_add() {
        struct TestData { a: Peano, b: Peano, expected: Peano }

        let test_data = vec![
            TestData { a: Peano::from_int(0), b: Peano::from_int(0), expected: Peano::from_int(0) },
            TestData { a: Peano::from_int(1), b: Peano::from_int(0), expected: Peano::from_int(1) },
            TestData { a: Peano::from_int(0), b: Peano::from_int(1), expected: Peano::from_int(1) },
            TestData { a: Peano::from_int(2), b: Peano::from_int(3), expected: Peano::from_int(5) },
            TestData { a: Peano::from_int(3), b: Peano::from_int(2), expected: Peano::from_int(5) },
        ];

        for TestData{ a, b, expected } in test_data {
            let result = a.add(&b);
            assert_eq!(result, expected, "Failed for a: {:?}, b: {:?}", a, b);
        }
    }

    #[test]
    fn test_mul() {
        struct TestData { a: Peano, b: Peano, expected: Peano }

        let test_data = vec![
            TestData { a: Peano::from_int(0), b: Peano::from_int(0), expected: Peano::from_int(0) },
            TestData { a: Peano::from_int(1), b: Peano::from_int(0), expected: Peano::from_int(0) },
            TestData { a: Peano::from_int(0), b: Peano::from_int(1), expected: Peano::from_int(0) },
            TestData { a: Peano::from_int(2), b: Peano::from_int(3), expected: Peano::from_int(6) },
            TestData { a: Peano::from_int(3), b: Peano::from_int(2), expected: Peano::from_int(6) },
        ];

        for TestData{ a, b, expected } in test_data {
            let result = a.mul(&b);
            assert_eq!(result, expected, "Failed for a: {:?}, b: {:?}", a, b);
        }
    }

    #[test]
    fn test_even() {
        assert_eq!(true, Peano::from_int(0).even());
        assert_eq!(false, Peano::from_int(1).even());
        assert_eq!(true, Peano::from_int(2).even());
        assert_eq!(false, Peano::from_int(3).even());
    }

    #[test]
    fn test_odd() {
        assert_eq!(false, Peano::from_int(0).odd());
        assert_eq!(true, Peano::from_int(1).odd());
        assert_eq!(false, Peano::from_int(2).odd());
        assert_eq!(true, Peano::from_int(3).odd());
    }

    #[test]
    fn test_div() {
        struct TestData { a: Peano, b: Peano, expected: Result<Peano, PeanoError> }

        let test_data = vec![
            TestData { a: Peano::from_int(0), b: Peano::from_int(0), expected: Err(PeanoError::DivisionByZero) },
            TestData { a: Peano::from_int(1), b: Peano::from_int(0), expected: Err(PeanoError::DivisionByZero) },
            TestData { a: Peano::from_int(0), b: Peano::from_int(1), expected: Ok(Peano::from_int(0)) },
            TestData { a: Peano::from_int(1), b: Peano::from_int(1), expected: Ok(Peano::from_int(1)) },
            TestData { a: Peano::from_int(2), b: Peano::from_int(1), expected: Ok(Peano::from_int(2)) },
            TestData { a: Peano::from_int(2), b: Peano::from_int(2), expected: Ok(Peano::from_int(1)) },
            TestData { a: Peano::from_int(6), b: Peano::from_int(2), expected: Ok(Peano::from_int(3)) },
            TestData { a: Peano::from_int(9), b: Peano::from_int(2), expected: Ok(Peano::from_int(4)) },
        ];

        for TestData{ a, b, expected } in test_data {
            let result = a.div(&b);
            assert_eq!(result, expected, "Failed for a: {:?}, b: {:?}", a, b);
        }
    }

    #[test]
    fn test_compare() {
        struct TestData { a: Peano, b: Peano, expected: Ordering }

        let test_data = vec![
            TestData { a: Peano::from_int(0), b: Peano::from_int(0), expected: Equal },
            TestData { a: Peano::from_int(1), b: Peano::from_int(0), expected: Greater },
            TestData { a: Peano::from_int(0), b: Peano::from_int(1), expected: Less },
            TestData { a: Peano::from_int(2), b: Peano::from_int(3), expected: Less },
            TestData { a: Peano::from_int(3), b: Peano::from_int(2), expected: Greater },
        ];

        for TestData{ a, b, expected } in test_data {
            let result = a.compare(&b);
            assert_eq!(result, expected, "Failed for a: {:?}, b: {:?}", a, b);
        }
    }
}