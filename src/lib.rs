
use crate::crypto::field_element::FieldElement;
use crate::crypto::point::Point;

pub mod crypto;

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
    #[should_panic(expected="Cannot add two numbers in different Fields")]
    fn test_add_diff_fields() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 19);
        let _c = a + b;
    }

    #[test]
    #[should_panic(expected="Cannot subtract two numbers in different Fields")]
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

    #[test]
    fn test_pow() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(1, 13);

        assert_eq!(a.pow(3), b);
    }

    #[test]
    fn test_pow_negative_exp() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(8, 13);

        assert_eq!(a.pow(-3), b);
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(7, 19);
        let c = FieldElement::new(3, 19);

        assert_eq!(a/b, c);

        let d = FieldElement::new(7, 19);
        let e = FieldElement::new(5, 19);
        let f = FieldElement::new(9, 19);

        assert_eq!(d/e, f);
    }

    #[test]
    fn test_point_is_on_the_curve() {
        let _point = Point::new(-1, -1, 5, 7);
    }

    #[test]
    #[should_panic(expected="(-1, -2) is not on the curve")]
    fn test_point_is_not_on_the_curve() {
        let _point = Point::new(-1, -2, 5, 7);
    }
}