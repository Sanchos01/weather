use crate::temp_call::{forecast::Forecast, openweather, temperature::Temperature, GetTemperature};
use mockito::{mock, Matcher, Mock};
use std::env;

pub fn setup(city: String) -> Mock {
  env::set_var("OPENWEATHER_APIKEY", "mock_key");
  let matches = vec![
    Matcher::UrlEncoded("q".into(), city.into()),
    Matcher::UrlEncoded("appid".into(), "mock_key".into()),
    Matcher::UrlEncoded("units".into(), "metric".into()),
  ];
  mock("GET", "/data/2.5/forecast")
    .with_header("content-type", "application/json")
    .with_body_from_file("src/tests/fixtures/ow_forecast.json")
    .match_query(Matcher::AllOf(matches))
    .create()
}

#[actix_rt::test]
async fn openweather_day_temperature() {
  let mock = setup("Moscow".into());
  let result = openweather::Call::day_temperature("Moscow".into(), 2).await;
  mock.assert();
  let temp = result.unwrap();
  assert_eq!(temp, Temperature(-1.45));
}

#[actix_rt::test]
async fn openweather_forecast() {
  let mock = setup("Moscow".into());
  let result = openweather::Call::forecast("Moscow".into()).await;
  mock.assert();
  let forecast = result.unwrap();
  let forecast_match = Forecast::new(2.85, 0.43, -1.45, -3.59, -5.39);
  assert_eq!(forecast, forecast_match);
}
