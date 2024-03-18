
use super::s256_field::S256Field;

#[derive(Debug, Clone)]
pub struct Signature {
  pub r: S256Field,
  pub s: S256Field
}

impl Signature { 
  pub fn new(r: S256Field, s: S256Field) -> Self {
    Signature { r,  s }
  }

  fn to_string(&self) -> String {
    format!("Signature({:x?},{:x?})",self.r, self.s)
  }
}