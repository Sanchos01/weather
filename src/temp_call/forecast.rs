use super::temperature::Temperature;
use serde::Serialize;
type T = Temperature;

// Структура прогноза погоды на 5 дней
#[derive(Debug, PartialEq, Clone, Copy, Default, Serialize)]
pub struct Forecast(T, T, T, T, T);

impl std::ops::Add for Forecast {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self(
      self.0 + other.0,
      self.1 + other.1,
      self.2 + other.2,
      self.3 + other.3,
      self.4 + other.4,
    )
  }
}

#[test]
fn add_test() {
  let first = Forecast::new(1.0, 1.5, 2.0, 2.5, 3.0);
  let second = Forecast::new(4.0, 4.5, 5.0, 5.5, 6.0);
  let third = Forecast::new(5.0, 6.0, 7.0, 8.0, 9.0);
  assert_eq!(first + second, third);
}

impl Forecast {
  pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64) -> Self {
    Forecast(
      Temperature(a),
      Temperature(b),
      Temperature(c),
      Temperature(d),
      Temperature(e),
    )
  }

  pub fn avg(forecasts: Vec<Self>) -> Self {
    let (sum, count) = forecasts
      .iter()
      .fold((Self::default(), 0), |(s, c), x| (s + *x, c + 1));
    sum.div(count)
  }

  fn div(self, v: u8) -> Self {
    Self(
      self.0.div(v),
      self.1.div(v),
      self.2.div(v),
      self.3.div(v),
      self.4.div(v),
    )
  }
}

#[test]
fn new_test() {
  let (a, b, c, d, e) = (1.0, 2.0, 3.0, 4.0, 5.0);
  assert_eq!(
    Forecast::new(a, b, c, d, e),
    Forecast(
      Temperature(a),
      Temperature(b),
      Temperature(c),
      Temperature(d),
      Temperature(e)
    )
  );
}

#[test]
fn avg_test() {
  let f1 = Forecast::new(1.0, 2.0, 3.0, 4.0, 5.0);
  let f2 = Forecast::new(1.5, 2.0, 2.5, 3.0, 3.5);
  let v = vec![f1, f2];
  let f3 = Forecast::new(1.25, 2.0, 2.75, 3.5, 4.25);
  assert_eq!(Forecast::avg(v), f3);
}
