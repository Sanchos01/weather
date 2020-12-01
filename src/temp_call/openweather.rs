use super::{Forecast, GetTemperature, RequestError, Temperature};
use crate::{HTTP_CLIENT, OPENWEATHER_APIKEY};
use futures::Future;
use log::warn;
use serde::Deserialize;
use std::pin::Pin;

#[cfg(test)]
use mockito;

#[derive(Default)]
pub struct Call();

#[derive(Deserialize)]
struct DayResponse {
  main: DayResponseMain,
}

#[derive(Deserialize)]
struct ForecastResponse {
  list: Vec<DayResponse>,
}

#[derive(Deserialize)]
struct DayResponseMain {
  temp: Temperature,
}

impl GetTemperature for Call {
  fn day_temperature(
    city: String,
    day: u8,
  ) -> Pin<Box<dyn Future<Output = Result<Temperature, RequestError>>>> {
    let func = async move {
      let host = host();
      let raw = format!(
        "{}/data/2.5/forecast?q={}&appid={}&units=metric",
        host, city, *OPENWEATHER_APIKEY
      );
      let url = reqwest::Url::parse(raw.as_str())?;
      let response = HTTP_CLIENT.get(url).send().await?;
      if response.status() != reqwest::StatusCode::OK {
        return Err(log_response_error(response, "forecast").await);
      }
      let body = response.json::<ForecastResponse>().await?;
      if let Some(temp) = body
        .list
        .iter()
        .step_by(8)
        .map(|x| x.main.temp)
        .nth(day as usize)
      {
        Ok(temp)
      } else {
        Err(RequestError {
          details: "openweather response error".to_string(),
        })
      }
    };
    Box::pin(func)
  }

  fn forecast(city: String) -> Pin<Box<dyn Future<Output = Result<Forecast, RequestError>>>> {
    let func = async move {
      let host = host();
      let raw = format!(
        "{}/data/2.5/forecast?q={}&appid={}&units=metric",
        host, city, *OPENWEATHER_APIKEY
      );
      let url = reqwest::Url::parse(raw.as_str())?;
      let response = HTTP_CLIENT.get(url).send().await?;
      if response.status() != reqwest::StatusCode::OK {
        return Err(log_response_error(response, "forecast").await);
      }
      let body = response.json::<ForecastResponse>().await?;
      let (temps, count) = body
        .list
        .iter()
        .step_by(8)
        .map(|x| x.main.temp)
        .take(5)
        .fold((Vec::new(), 0), |(mut v, count), x| {
          v.push(x);
          (v, count + 1)
        });
      if count == 5 {
        Ok(Forecast::new(
          temps[0].0, temps[1].0, temps[2].0, temps[3].0, temps[4].0,
        ))
      } else {
        Err(RequestError {
          details: "not enought data in openweather forecast".to_string(),
        })
      }
    };
    Box::pin(func)
  }
}

#[cfg(not(test))]
fn host() -> String {
  "https://api.openweathermap.org".to_string()
}
#[cfg(test)]
fn host() -> String {
  mockito::server_url()
}

async fn log_response_error(response: reqwest::Response, fun_name: &str) -> RequestError {
  warn!(
    "openweather {:?} request error {{ status: {:?}, body: {:?} }}",
    fun_name,
    response.status(),
    response.text().await.unwrap()
  );
  RequestError {
    details: "openweather api error".to_string(),
  }
}
