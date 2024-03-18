use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Euclid, One, Zero};
use super::field_elements::FieldOperation;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct S256Field {
  /// inner value
  num: BigInt, 
  /// finite field
  prime: BigInt 
}

impl FieldOperation for S256Field {

  fn new(num: BigInt, prime: BigInt) -> Self {
    if num >= prime {
      panic!("Num {} not in field range 0 to {}", num, prime)
    }
    S256Field {num,prime}
  }
  
  fn to_string(&self) -> String {
    format!("{:0>64x}", self.num)
  }
  
  fn get_num(&self) -> BigInt {
    self.num.clone()
  }
  fn get_prime(&self) -> BigInt {
    self.prime.clone()
  }
  fn set_num(&mut self, num: BigInt) {
    self.num = num;
  }
  
  fn add(&self, other: &Self) -> Self {
    if self.get_prime() != other.get_prime() {
      panic!("Cannot add two numbers in different Fields")
    }

    let result = Euclid::rem_euclid(&(&self.get_num() + &other.get_num()), &self.get_prime());
    Self::new(result, self.get_prime().clone())
  }
  
  fn sub(&self, other: &Self) -> Self {
    if self.get_prime() != other.get_prime() {
      panic!("Cannot sub two numbers in different Fields")
    };
    let result = Euclid::rem_euclid(&(&self.get_num() - &other.get_num()), &self.get_prime());
    Self::new(result, self.get_prime().clone())
  }
  
  fn mul(&self, other: &Self) -> Self {
    if self.get_prime() != other.get_prime() {
      panic!("Cannot add two numbers in different Fields")
    }
    let result = Euclid::rem_euclid(&(&self.get_num() * &other.get_num()), &self.get_prime());
    Self::new(result, self.get_prime().clone())
  }
  
  fn truediv(&self, other: &Self) -> Self {
    if self.get_prime() != other.get_prime() {
      panic!("Cannot add two numbers in different Fields")
    }
    let other_inverse = other.pow(&self.get_prime() - &BigInt::from(2u128));
    let result = Euclid::rem_euclid(&(&self.get_num() * &other_inverse.get_num()), &self.get_prime());

    Self::new(result, self.get_prime().clone())
  }
  
  fn pow(&self, exp: BigInt) -> Self {
    let n = Euclid::rem_euclid(&exp, &(&self.get_prime() - &BigInt::from(1u128)));

    let result = Self::mod_exp(&self.get_num(), &n, &self.get_prime());

    Self::new(result, self.get_prime().clone())
  }
  
  fn mod_exp(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
    let mut result = BigInt::one();
    let mut base = base.clone();
    let mut exponent = exponent.clone();

    while exponent > BigInt::zero() {
        if exponent.is_odd() {
            result = Euclid::rem_euclid(&(&result * &base), modulus.into());
        }
        base = Euclid::rem_euclid(&(&base * &base), modulus.into());
        exponent >>= 1;
    }
    result
  }

  fn double(&self) -> Self {
    self.add(&self)
  }
  /// Double-and-add algorithm
  fn rmul(&self, coef: S256Field) -> S256Field {
    let mut result: S256Field = Self::new(BigInt::zero(), self.prime.clone());
    let mut base = self.clone();
    let mut c = coef.clone();

    while c.get_num() > BigInt::zero() {
      if c.get_num().is_odd() {
          result = result.add(&base);
      }
      base = base.double();
      c.set_num(c.get_num() >> 1);
    }
    result
  }
}