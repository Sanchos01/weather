pub mod forecast;
pub mod openweather;
pub mod temperature;
pub mod weatherbit;

use forecast::Forecast;
use futures::{try_join, Future};
use serde::Serialize;
use std::pin::Pin;
use temperature::Temperature;

// Интерфейс для внешних api
pub trait GetTemperature {
  fn day_temperature(
    city: String,
    day: u8,
  ) -> Pin<Box<dyn Future<Output = Result<Temperature, RequestError>>>>;
  fn forecast(city: String) -> Pin<Box<dyn Future<Output = Result<Forecast, RequestError>>>>;
}

pub async fn get_weather(city: String, day: u8) -> Result<Temperature, RequestError> {
  let call1 = openweather::Call::day_temperature(city.clone(), day);
  let call2 = weatherbit::Call::day_temperature(city, day);
  match try_join!(call1, call2) {
    Ok((t1, t2)) => {
      let avg_temp = Temperature::avg(vec![t1, t2]);
      Ok(avg_temp)
    }
    Err(error) => Err(error),
  }
}

pub async fn get_forecast(city: String) -> Result<Forecast, RequestError> {
  let call1 = openweather::Call::forecast(city.clone());
  let call2 = weatherbit::Call::forecast(city);
  match try_join!(call1, call2) {
    Ok((f1, f2)) => {
      let avg_forecast = Forecast::avg(vec![f1, f2]);
      Ok(avg_forecast)
    }
    Err(error) => Err(error),
  }
}

// Кастомная ошибка
#[derive(Serialize)]
pub struct RequestError {
  details: String,
}

impl RequestError {
  pub fn new(details: String) -> Self {
    Self { details }
  }
}

impl std::fmt::Display for RequestError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Request error: {}", self.details)
  }
}

impl std::fmt::Debug for RequestError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Request error {{ file: {}, line: {}, details: {} }}",
      file!(),
      line!(),
      self.details
    )
  }
}

impl std::convert::From<reqwest::Error> for RequestError {
  fn from(error: reqwest::Error) -> Self {
    RequestError {
      details: format!("Request error: {:?}", error),
    }
  }
}

impl std::convert::From<url::ParseError> for RequestError {
  fn from(error: url::ParseError) -> Self {
    RequestError {
      details: format!("Parse url error: {:?}", error),
    }
  }
}
