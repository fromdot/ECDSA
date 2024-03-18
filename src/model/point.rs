use num_bigint::BigInt;
use num_traits::Zero;
use num_integer::Integer;
use super::field_elements::FieldElement;
use crate::model::field_elements::FieldOperation;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
  // basic form: y^2 = x^3 + ax + b
  x: Option<FieldElement>, 
  y: Option<FieldElement>,
  a: FieldElement,
  b: FieldElement,
}

impl Point {
  pub fn new(x:Option<FieldElement>, y:Option<FieldElement>, a:FieldElement, b:FieldElement) -> Self 
  {
    if x == None && y == None {
      // 무한 원점, `Point(infinity)`
      return Self { x:None, y:None, a, b };
    }

    let y2 = y.clone().map(|y_| y_.pow(BigInt::from(2u128)));

    let result = match x.clone() {
      Some(x_) => Some(x_.pow(BigInt::from(3u128)).add(&(x_.mul(&a))).add(&b)),
      _ => None,
    };

    if y2 != result {
      panic!("({:?}, {:?}) [{:?} ? {:?}] is not on the curve",x,y, y2, result);
    }
    
    Self {x,y,a,b}
  }

  pub fn to_string(&self) -> String 
  {
    if self.x == None {
      format!("Point(infinity)")
    } else {
      format!("Point({:?},{:?})_{:?}_{:?}", self.x, self.y, self.a, self.b)
    }
  }

  pub fn eq(&self, other: &Self) -> bool 
  {
    self.x == other.x && self.y == other.y && 
    self.a == other.a && self.b == other.b 
  }

  pub fn ne(&self, other: &Self) -> bool 
  {
    !self.eq(other)
  }

  pub fn add(&self, other: &Self) -> Self 
  {
    if self.a != other.a || self.b != other.b {
      panic!("Point ({:?}, {:?}) is not on the same curve", self, other);
    }

    if self.x == None {
      return other.to_owned()
    } 
    
    if other.x == None {
      return self.to_owned()
    }

    
    if self.x == other.x && self.y != other.y // 두 점이 다른 경우 (y축 평행 O, x축 대칭)
       || self == other && self.y == match &self.x { 
          Some(x_) => Some(FieldElement::new(BigInt::from(0u128), x_.get_prime().clone())), 
          _ => None
        }
    {
      return Self::new(None, None, self.a.clone(), self.b.clone())
    }

    if self.x != other.x // 두 점이 다른 경우 (y축 평행 X)
    {
      let (x1, y1) = (self.x.clone(), self.y.clone());
      let (x2, y2) = (other.x.clone(), other.y.clone());
      
      let s = match (&y2, &y1, &x2, &x1) {
        (Some(y2_), Some(y1_), Some(x2_), Some(x1_)) 
        => Some((y2_.sub(&y1_)).truediv(&x2_.sub(&x1_))),
        _ => None
      };
      
      let x3 = match (&s, &x1, &x2) {
        (Some(s_), Some(x1_), Some(x2_)) 
        => Some(s_.pow(BigInt::from(2u128)).sub(&x1_).sub(&x2_)),
        _ => None
      };
      let y3 = match (&s, &x1, &x3, &y1) {
        (Some(s_), Some(x1_), Some(x3_), Some(y1_)) 
        => Some(s_.mul(&x1_.sub(&x3_)).sub(&y1_)),
        _ => None
      };

      return Self::new(x3, y3, self.a.clone(), self.b.clone())
    }

    if self == other // 두 점이 같은 경우 (접하는 경우)
    {      
      let (x1, y1) = (self.x.clone(), self.y.clone());
      let s = match (&x1, &y1) {
        (Some(x1_), Some(y1_)) 
          => Some((x1_.pow(BigInt::from(2u128))
                    .mul(&FieldElement::new(BigInt::from(3u128),x1_.get_prime().clone()))
                    .add(&self.a))
                    .truediv(&y1_.mul(
                      &FieldElement::new(BigInt::from(2u128), y1_.get_prime().clone())
                ))),
        _ => None
      };
      let x3 = match (&s, &x1) {
        (Some(s_val),Some(x1_val)) 
          => Some(s_val.pow(BigInt::from(2u128))
                        .sub(&x1_val.mul(
                          &FieldElement::new(BigInt::from(2u128),x1_val.get_prime().clone()))
                  )),
        _ => None
      };
      let y3 = match (&s, &x1, &x3, &y1) {
        (Some(s_val),Some(x1_val), Some(x3_val), Some(y1_val)) 
        => Some(s_val.mul(&x1_val.sub(&x3_val)).sub(&y1_val)),
        _ => None
      };
      return Self::new(x3, y3, self.a.clone(), self.b.clone())
    }

    unreachable!("Not match")
  }

  fn double(&self) -> Self {
    self.add(&self)
  }
  /// double-and-add algorithm for point multiplication:
  pub fn rmul(&self, coef: BigInt) -> Self {
    let mut result: Point = Self::new(None, None, self.a.clone(), self.b.clone());
    let mut base = self.clone();
    let mut c = coef.clone();

    while c > BigInt::zero() {
      if c.is_odd() {
          result = result.add(&base);
      }
      base = base.double();
      c = c >> 1;
    }
    result
  }
}