
use crate::crypto::field_element::FieldElement;
use crate::crypto::point::Point;

pub mod crypto;

fn main() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(6, 13);

    print!("{}\n", a);
    print!("{}\n", b);
}
