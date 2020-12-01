use super::{Forecast, GetTemperature, RequestError, Temperature};
use crate::{HTTP_CLIENT, WEATHERBIT_APIKEY};
use futures::Future;
use log::warn;
use serde::Deserialize;
use std::pin::Pin;

#[cfg(test)]
use mockito;

#[derive(Default)]
pub struct Call();

#[derive(Deserialize)]
struct ForecastResponse {
  data: Vec<DayResponseMain>,
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
        "{}/v2.0/forecast/daily?city={}&key={}&days={}",
        host,
        city,
        *WEATHERBIT_APIKEY,
        day + 1
      );
      let url = reqwest::Url::parse(raw.as_str())?;
      let response = HTTP_CLIENT.get(url).send().await?;
      if response.status() != reqwest::StatusCode::OK {
        return Err(log_response_error(response, "day").await);
      }
      let body = response.json::<ForecastResponse>().await?;
      if let Some(day) = body.data.iter().nth(day as usize) {
        Ok(day.temp)
      } else {
        Err(RequestError {
          details: "weatherbit response error".to_string(),
        })
      }
    };
    Box::pin(func)
  }

  fn forecast(city: String) -> Pin<Box<dyn Future<Output = Result<Forecast, RequestError>>>> {
    let func = async move {
      let host = host();
      let raw = format!(
        "{}/v2.0/forecast/daily?city={}&key={}&days=5",
        host, city, *WEATHERBIT_APIKEY
      );
      let url = reqwest::Url::parse(raw.as_str())?;
      let response = HTTP_CLIENT.get(url).send().await?;
      if response.status() != reqwest::StatusCode::OK {
        return Err(log_response_error(response, "forecast").await);
      }
      let body = response.json::<ForecastResponse>().await?;
      let (temps, count) =
        body
          .data
          .iter()
          .map(|x| x.temp)
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
          details: "not enought data in weatherbit forecast".to_string(),
        })
      }
    };
    Box::pin(func)
  }
}

#[cfg(not(test))]
fn host() -> String {
  "https://api.weatherbit.io".to_string()
}
#[cfg(test)]
fn host() -> String {
  mockito::server_url()
}

async fn log_response_error(response: reqwest::Response, fun_name: &str) -> RequestError {
  warn!(
    "weatherbit {:?} request error {{ status: {:?}, body: {:?} }}",
    fun_name,
    response.status(),
    response.text().await.unwrap()
  );
  RequestError {
    details: "weatherbit api error".to_string(),
  }
}
