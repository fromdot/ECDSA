use num_bigint::{BigInt, BigUint, ParseBigIntError};
use num_traits::{Num, Pow};
use once_cell::sync::Lazy;

use super::{field_elements::{FieldElement, FieldOperation}, point::Point, s256_field::S256Field, s256_point::S256Point};

/// 유한체의 위수인 소수
pub static PRIME: Lazy<BigInt> = Lazy::new(|| {
  BigInt::from(2u128).pow(BigUint::from(256u128)) - BigInt::from(2u128).pow(32u128) - BigInt::from(977u128)
});
pub static A: Lazy<FieldElement> = 
Lazy::new(|| {
  FieldElement::new(BigInt::from(0u128), PRIME.clone())
});

pub static B: Lazy<FieldElement> = 
Lazy::new(|| {
  FieldElement::new(BigInt::from(7u128), PRIME.clone())
});

pub static As: Lazy<S256Field> = Lazy::new(|| {
  S256Field::new(BigInt::from(0u128), PRIME.clone())
});
pub static Bs: Lazy<S256Field> = Lazy::new(|| {
  S256Field::new(BigInt::from(7u128), PRIME.clone())
});

pub static gx: Lazy<Result<BigInt, ParseBigIntError>> = Lazy::new(|| {
  let gx_str: &str = "79be_667e_f9dc_bbac_55a0_6295_ce87_0b07_029b_fcdb_2dce_28d9_59f2_815b_16f8_1798"; 
  // 0x does not work, but underscore works well
   BigInt::from_str_radix(gx_str, 16) // hex
});

pub static gy: Lazy<Result<BigInt, ParseBigIntError>> = Lazy::new(|| {
  let gy_str: &str = "483a_da77_26a3_c465_5da4_fbfc_0e11_08a8_fd17_b448_a685_5419_9c47_d08f_fb10_d4b8";
  BigInt::from_str_radix(gy_str, 16)
});


static x: Lazy<FieldElement> = Lazy::new(|| {
  match gx.clone() {
    Ok(gx_) => FieldElement::new(gx_, PRIME.clone()),
    _ => panic!("no element")
  }
});

static y: Lazy<FieldElement> = Lazy::new(|| {
  match gy.clone() {
    Ok(gy_) => FieldElement::new(gy_, PRIME.clone()),
    _ => panic!("no element")
  }
});


static xs: Lazy<S256Field> = Lazy::new(|| {
  match gx.clone() {
    Ok(gx_) => S256Field::new(gx_, PRIME.clone()),
    _ => panic!("no element")
  }
});

static ys: Lazy<S256Field> = Lazy::new(|| {
  match gy.clone() {
    Ok(gy_) => S256Field::new(gy_, PRIME.clone()),
    _ => panic!("no element")
  }
});

pub static G: Lazy<Point> = Lazy::new(|| {
  Point::new(Some(x.clone()), Some(y.clone()), A.clone(), B.clone())
});

pub static Gs: Lazy<S256Point> = Lazy::new(|| {
  S256Point::new(Some(xs.clone()), Some(ys.clone()), As.clone(), Bs.clone())
});

pub static N: Lazy<BigInt> = Lazy::new(|| {
  let n_str: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
  let n = BigInt::from_str_radix(n_str, 16);
  // let n = BigUint::from_str_radix(n_str, 16)

  match n.clone() {
    Ok(n_) => n_,
    _ => panic!("no element")
  }

});
