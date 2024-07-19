use std::fmt;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct FieldElement
{
    num: i32,
    prime: i32
}

impl FieldElement
{
    pub fn new(num: i32, prime: i32) -> Self {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }

        FieldElement{
            num: num,
            prime: prime
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}
impl Eq for FieldElement {}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        FieldElement::new((self.num + other.num) % self.prime, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let sub = self.num - other.num;
        if sub < 0 {
            return FieldElement::new(self.prime + sub, self.prime);
        }
        FieldElement::new(sub, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        FieldElement::new((self.num * other.num) % self.prime, self.prime)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement {} ({})", self.num, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);

        assert!(a.eq(&a));
        assert!(!a.eq(&b));

        assert!(a == a);
        assert!(a != b);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(11, 19);
        let b = FieldElement::new(9, 19);
        let c = FieldElement::new(2, 19);

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_sub_negative_num() {
        let a = FieldElement::new(6, 19);
        let b = FieldElement::new(13, 19);
        let c = FieldElement::new(12, 19);

        assert_eq!(a - b, c);
    }

    #[test]
    #[should_panic]
    fn test_add_diff_fields() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 19);
        let _c = a + b;
    }

    #[test]
    #[should_panic]
    fn test_sub_diff_fields() {
        let a = FieldElement::new(7, 11);
        let b = FieldElement::new(12, 13);
        let _c = a - b;
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(10, 13);

        assert_eq!(a * b, c);
    }
}