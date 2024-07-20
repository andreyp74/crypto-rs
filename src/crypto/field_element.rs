use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::time::Instant;

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

    pub fn pow(self, exponent: i32) -> Self {
        let mut exp = exponent;
        while exp < 0 {
            exp += self.prime - 1; 
        }
        exp = exp % (self.prime - 1);
        let num = i64::pow(self.num as i64, exp.try_into().unwrap()) % self.prime as i64;
        FieldElement::new(num.try_into().unwrap(), self.prime)
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
        //let now = Instant::now();
        let num = (self.num + other.num) % self.prime;
        //let elapsed = now.elapsed();
        //println!("Add elapsed: {:.2?}\n", elapsed);

        FieldElement::new(num, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }

        //let now = Instant::now();
        let mut sub = self.num - other.num;
        if sub < 0 {
            sub = self.prime + sub;
        }
        //let elapsed = now.elapsed();
        //println!("Sub elapsed: {:.2?}\n", elapsed);

        FieldElement::new(sub, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        
        //let now = Instant::now();
        let num = (self.num * other.num) % self.prime;
        //let elapsed = now.elapsed();
        //println!("Mul elapsed: {:.2?}\n", elapsed);

        FieldElement::new(num, self.prime)
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }

        //#[cfg(debug_assertions)]
        //let now = Instant::now();
        
        //TODO optimize this power by module operation?
        let num = (self.num as i64 * i64::pow(other.num as i64, (self.prime - 2).try_into().unwrap())) % self.prime as i64;

        //if cfg!(debug_assertions) {
            //let elapsed = now.elapsed();
            //println!("Div elapsed: {:.2?}\n", elapsed);
        //}

        FieldElement::new(num.try_into().unwrap(), self.prime)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement {} ({})", self.num, self.prime)
    }
}