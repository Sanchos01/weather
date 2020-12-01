use super::{Forecast, GetTemperature, RequestError, RequestErrorType, Temperature};
use crate::{HTTP_CLIENT, OPENWEATHER_APIKEY};
use futures::Future;
use serde::Deserialize;
use std::pin::Pin;

#[cfg(test)]
use mockito;

#[derive(Default)]
pub struct Call();

#[derive(Deserialize, Debug)]
struct DayResponse {
  main: DayResponseMain,
}

#[derive(Deserialize, Debug)]
struct ForecastResponse {
  list: Vec<DayResponse>,
}

#[derive(Deserialize, Debug)]
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
        let msg = "Openweather api error";
        let error = RequestError::new(RequestErrorType::Request, msg.into(), response);
        return Err(error);
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
        let msg = "Openweather response error";
        let error = RequestError::new(RequestErrorType::Decode, msg.into(), body);
        Err(error)
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
        let msg = "Openweather api error";
        let error = RequestError::new(RequestErrorType::Request, msg.to_string(), response);
        return Err(error);
        // return Err(log_response_error(response, "forecast").await);
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
        let msg = "Not enought data in openweather forecast";
        let error = RequestError::new(RequestErrorType::Decode, msg.into(), body);
        Err(error)
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
