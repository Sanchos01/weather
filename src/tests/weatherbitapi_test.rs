use crate::temp_call::{forecast::Forecast, temperature::Temperature, weatherbit, GetTemperature};
use mockito::{mock, Matcher, Mock};
use std::env;

pub fn setup(city: String, days: usize) -> Mock {
  env::set_var("WEATHERBIT_APIKEY", "mock_key");
  let matches = vec![
    Matcher::UrlEncoded("city".into(), city.into()),
    Matcher::UrlEncoded("key".into(), "mock_key".into()),
    Matcher::UrlEncoded("days".into(), format!("{}", days)),
  ];
  mock("GET", "/v2.0/forecast/daily")
    .with_header("content-type", "application/json")
    .with_body_from_file("src/tests/fixtures/wb_forecast.json")
    .match_query(Matcher::AllOf(matches))
    .create()
}

#[actix_rt::test]
async fn weatherbit_day_temperature() {
  let mock = setup("Moscow".into(), 3);
  let result = weatherbit::Call::day_temperature("Moscow".into(), 2).await;
  mock.assert();
  let temp = result.unwrap();
  assert_eq!(temp, Temperature(-2.7));
}

#[actix_rt::test]
async fn weatherbit_forecast() {
  let mock = setup("Moscow".into(), 5);
  let result = weatherbit::Call::forecast("Moscow".into()).await;
  mock.assert();
  let forecast = result.unwrap();
  let forecast_match = Forecast::new(0.0, -1.2, -2.7, -4.0, -4.6);
  assert_eq!(forecast, forecast_match);
}
