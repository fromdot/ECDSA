use num_bigint::BigInt;
use num_traits::{One, Zero, Euclid};
use num_integer::Integer;

pub trait FieldOperation {
  fn get_num(&self) -> BigInt;
  fn get_prime(&self) -> BigInt;
  fn set_num(&mut self, num: BigInt);
  
  fn new(num: BigInt, prime: BigInt) -> Self;
  fn to_string(&self) -> String;
  fn eq(&self, other: &Self) -> bool {
    self.get_num() == other.get_num() && self.get_prime() == other.get_prime()
  }
  fn ne(&self, other: &Self) -> bool {
    !self.eq(other)
  }
  fn add(&self, other: &Self) -> Self;
  fn sub(&self, other: &Self) -> Self;
  fn mul(&self, other: &Self) -> Self;
  fn truediv(&self, other: &Self) -> Self;
  fn pow(&self, exp: BigInt) -> Self;
  fn mod_exp(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt;
  fn double(&self) -> Self;
  fn rmul(&self, coef: Self) -> Self;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FieldElement {
  /// inner value
  num: BigInt, 
  /// finite field
  prime: BigInt 
}

impl FieldOperation for FieldElement {
  fn get_num(&self) -> BigInt {
    self.num.clone()
}
fn get_prime(&self) -> BigInt {
    self.prime.clone()
}
fn set_num(&mut self, num: BigInt) {
  self.num = num;
}

fn new(num:BigInt, prime:BigInt) -> Self {
  if num >= prime {
    panic!("Num {} not in field range 0 to {}", num, prime)
  }
  FieldElement {num,prime}
}

fn to_string(&self) -> String {
  format!("FieldElement_{}({:0>64x})", self.prime, self.num)
}

fn add(&self, other: &Self) -> Self {
  if self.get_prime() != other.get_prime() {
    panic!("Cannot add two numbers in different Fields")
  }

  let result = Euclid::rem_euclid(&(self.get_num() + other.get_num()), &self.get_prime());
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

/// modular exponentiation using the square-and-multiply algorithm:
fn mod_exp(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
  let mut result = BigInt::one();
  let mut base = base.clone();
  let mut exponent = exponent.clone();

  while exponent > BigInt::zero() {
      if exponent.is_odd() {
          result = Euclid::rem_euclid(&(&result * &base), modulus);
      }
      base = Euclid::rem_euclid(&(&base * &base), modulus);
      exponent >>= 1;
  }
  result
}

fn double(&self) -> Self {
  todo!()
}

fn rmul(&self, coef: Self) -> Self {
  todo!()
}

}