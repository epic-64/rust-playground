use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use Peano::{Succ, Zero};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Peano {
    Zero,
    Succ(Box<Peano>),
}

#[derive(Debug, PartialEq, Eq)]
enum PeanoError {
    DivisionByZero,
}

impl Peano {
    fn from_int(n: i64) -> Peano {
        if n <= 0 {
            Zero
        } else {
            Succ(Box::new(Peano::from_int(n - 1)))
        }
    }

    fn to_int(&self) -> i64 {
        match self {
            Zero => 0,
            Succ(n) => 1 + n.to_int(),
        }
    }

    fn add(&self, other: &Peano) -> Peano {
        match self {
            Zero => other.clone(),
            Succ(n) => Succ(Box::new(n.add(other))),
        }
    }

    fn sub(&self, other: &Peano) -> Peano {
        match (self, other) {
            (Zero, _) => Zero,
            (Succ(_), Zero) => self.clone(),
            (Succ(n1), Succ(n2)) => n1.sub(n2),
        }
    }

    fn mul(&self, other: &Peano) -> Peano {
        match self {
            Zero => Zero,
            Succ(n) => n.mul(other).add(other),
        }
    }

    fn even(&self) -> bool {
        match self {
            Zero => true,
            Succ(n) => !n.even(),
        }
    }

    fn odd(&self) -> bool {
        match self {
            Zero => false,
            Succ(n) => n.even(),
        }
    }

    fn div(&self, other: &Peano) -> Result<Peano, PeanoError> {
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

    fn compare(&self, other: &Peano) -> Ordering {
        match (self, other) {
            (Zero, Zero) => Equal,
            (Zero, _) => Less,
            (_, Zero) => Greater,
            (Succ(n1), Succ(n2)) => n1.compare(n2), // actually walks backwards until n1 or n2 is Zero
        }
    }
}

fn main() {
    //
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::Greater;
    use super::*;

    #[test]
    fn test_from_int() {
        assert_eq!(Peano::from_int(0), Zero);
        assert_eq!(Peano::from_int(1), Succ(Box::new(Zero)));
        assert_eq!(Peano::from_int(2), Succ(Box::new(Succ(Box::new(Zero)))));
    }

    #[test]
    fn test_to_int() {
        assert_eq!(Zero.to_int(), 0);
        assert_eq!(Succ(Box::new(Zero)).to_int(), 1);
        assert_eq!(Succ(Box::new(Succ(Box::new(Zero)))).to_int(), 2);
    }

    #[test]
    fn test_add() {
        let a = Peano::from_int(2);
        let b = Peano::from_int(3);
        assert_eq!(a.add(&b), Peano::from_int(5));
    }

    #[test]
    fn test_mul() {
        let a = Peano::from_int(2);
        let b = Peano::from_int(3);
        assert_eq!(a.mul(&b), Peano::from_int(6));
    }

    #[test]
    fn test_mul_2() {
        let a = Peano::from_int(0);
        let b = Peano::from_int(3);
        assert_eq!(a.mul(&b), Peano::from_int(0));
    }

    #[test]
    fn test_mul_3() {
        let a = Peano::from_int(5);
        let b = Peano::from_int(7);
        assert_eq!(a.mul(&b), Peano::from_int(35));
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
            TestData { a: Peano::from_int(6), b: Peano::from_int(2), expected: Ok(Peano::from_int(3)) },
            TestData { a: Peano::from_int(6), b: Peano::from_int(0), expected: Err(PeanoError::DivisionByZero) },
            TestData { a: Peano::from_int(17), b: Peano::from_int(4), expected: Ok(Peano::from_int(4)) },
            TestData { a: Peano::from_int(3), b: Peano::from_int(3), expected: Ok(Peano::from_int(1)) },
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

