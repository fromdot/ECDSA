use num_bigint::BigInt;
use num_traits::Euclid;

use sha2::Sha256;
use hmac::{Hmac, Mac};
use super::{
  field_elements::FieldOperation, 
  s256_field::S256Field, 
  s256_point::S256Point, 
  signature::Signature
};
use crate::model::constants::{Gs, PRIME, N };

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct PrivateKey {
  secret: BigInt,
  pub point: S256Point
}

impl PrivateKey {
  pub fn new(secret: BigInt) -> Self {
    Self { 
      secret: secret.clone(), 
      point: Gs.rmul(secret)
    }
  }

  fn to_string(&self) -> String {
    format!("{:0>64x}", self.secret)
  }

  /// https://datatracker.ietf.org/doc/html/rfc6979
  fn deterministic_k(&self, mut z: BigInt) -> S256Field {
    let mut k = b"\x00".repeat(32);
    let mut v = b"\x01".repeat(32);
    if z > N.clone() {
      z = (z - N.clone())
    }

    let z_bytes = z.to_bytes_be();
    let secret_bytes = self.secret.to_bytes_be();
    
    let mut mac = HmacSha256::new_from_slice(b"")
      .expect("HMAC can take key of any size");
    mac.update(&k); 
    mac.update(&v);
    mac.update(b"\x00");
    mac.update(&secret_bytes.1);
    mac.update(&z_bytes.1);
    k = mac.finalize().into_bytes().to_vec();

    let mut mac = HmacSha256::new_from_slice(b"")
      .expect("HMAC can take key of any size");
    mac.update(&k); 
    mac.update(&v);
    v = mac.finalize().into_bytes().to_vec();

    let mut mac = HmacSha256::new_from_slice(b"")
      .expect("HMAC can take key of any size");
    mac.update(&k); 
    mac.update(&v);
    mac.update(b"\x01");
    mac.update(&secret_bytes.1); 
    mac.update(&z_bytes.1);
    k = mac.finalize().into_bytes().to_vec();

    let mut mac = HmacSha256::new_from_slice(b"")
      .expect("HMAC can take key of any size");
    mac.update(&k); 
    mac.update(&v);
    v = mac.finalize().into_bytes().to_vec();

    loop {
      let mut mac = HmacSha256::new_from_slice(b"")
        .expect("HMAC can take key of any size");
      mac.update(&k);
      mac.update(&v);
      v = mac.finalize().into_bytes().to_vec();
      let candidate = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v);
      if candidate >= BigInt::from(1u128) && candidate < N.clone() {
          return S256Field::new(candidate, PRIME.clone())
      }

      let mut mac = HmacSha256::new_from_slice(b"")
        .expect("HMAC can take key of any size");
      mac.update(&k); 
      mac.update(&v);
      mac.update(b"\x00");
      k = mac.finalize().into_bytes().to_vec();

      let mut mac = HmacSha256::new_from_slice(b"")
        .expect("HMAC can take key of any size");
      mac.update(&k); 
      mac.update(&v);
      v = mac.finalize().into_bytes().to_vec();
    }
  }

  pub fn sign(&self, z: BigInt) -> Signature {
    let k = self.deterministic_k(z.clone());
    
    let r = match Gs.rmul(k.get_num().clone()).x {
      Some(x_) => x_,
      None => panic!("no r")
    };
        
    let k_inv = S256Field::mod_exp(&k.get_num(), &(N.clone() - &BigInt::from(2u128)), &N);
    let mut s = Euclid::rem_euclid(&((&z + r.get_num() * self.secret.clone() ) * &k_inv), &N.clone());

    if s > (N.clone() / BigInt::from(2u128)) { 
      s = N.clone() - s
    }

    Signature::new(r, S256Field::new(s, PRIME.clone()))
  }
}