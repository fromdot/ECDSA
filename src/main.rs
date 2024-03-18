extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
use ecdsa::model::{
  constants::{Gs, N},
  private_key::PrivateKey
};

fn main() {  
  println!("\nHello, Crypto World!\n--------------------");

  let ng = Gs.rmul(N.clone());
  println!("ng: {:?}", ng); // infinity

  let mut rng = rand::thread_rng();
  
  let r1 = rng.gen_biguint(256).to_bigint();
  let r2 = rng.gen_bigint_range(&BigInt::from(0), &N);
  
  let z = match r1.clone() {
    Some(r1_) => r1_,
    _ => panic!("no N")
  };

  let pk = PrivateKey::new(r2);
  let sig = pk.sign(z.clone());
  
  let z_message = "안녕하세요";
  let m_hash = BigInt::from_bytes_be(Sign::Plus, z_message.as_bytes());
  let m_sig = pk.sign(m_hash.clone());

  println!("⛔ verify random integer: {:?}",pk.point.verify(z, sig));
  println!("⛔ verify message: {:?}",pk.point.verify(m_hash, m_sig));
  // assert!(pk.point.verify(z, sig));

  println!("-----------------------------\nit needs time and resource...\n");
}