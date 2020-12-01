use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Default, Deserialize, Serialize)]
// Структура температуры в конкретный момент
pub struct Temperature(pub f64);

impl std::ops::Add for Temperature {
  type Output = Temperature;

  fn add(self, other: Self) -> Self::Output {
    Self(self.0 + other.0)
  }
}

#[test]
fn add_test() {
  let t1 = Temperature(2.0);
  let t2 = Temperature(3.0);
  let t3 = Temperature(5.0);
  assert_eq!(t1 + t2, t3);
}

impl Temperature {
  pub fn avg(temps: Vec<Self>) -> Self {
    let (sum, count) = temps
      .iter()
      .fold((Self::default(), 0), |(s, c), x| (s + *x, c + 1));
    sum.div(count)
  }

  pub fn div(self, v: u8) -> Self {
    let raw = self.0 / v as f64;
    Self((raw * 100.0).round() / 100.0)
  }
}

#[test]
fn avg_test() {
  let vector = vec![Temperature(3.0), Temperature(5.0)];
  assert_eq!(Temperature::avg(vector), Temperature(4.0));
}

#[test]
fn div_test() {
  let t1 = Temperature(4.0);
  let t2 = Temperature(2.0);
  assert_eq!(Temperature::div(t1, 2), t2);
}
