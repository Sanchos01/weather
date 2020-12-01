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
#[derive(Serialize, Debug)]
pub struct RequestError {
  kind: RequestErrorType,
  message: String,
  #[serde(skip_serializing)]
  details: String,
}

#[derive(Debug, Serialize)]
pub enum RequestErrorType {
  Request,
  Decode,
  Client,
}

impl RequestError {
  pub fn new<T: std::fmt::Debug>(kind: RequestErrorType, message: String, data: T) -> Self {
    Self {
      kind,
      message,
      details: format!("{:?}", data),
    }
  }
}

impl std::fmt::Display for RequestError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{} error: {}", self.kind, self.message)
  }
}

impl std::fmt::Display for RequestErrorType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl std::convert::From<reqwest::Error> for RequestError {
  fn from(error: reqwest::Error) -> Self {
    let msg = "Request error to outer api";
    RequestError::new(RequestErrorType::Request, msg.into(), error)
  }
}

impl std::convert::From<url::ParseError> for RequestError {
  fn from(error: url::ParseError) -> Self {
    let msg = "Parse url error";
    RequestError::new(RequestErrorType::Request, msg.into(), error)
  }
}
