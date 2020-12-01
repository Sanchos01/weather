use super::{openweatherapi_test, weatherbitapi_test};
use crate::temp_call::{self, forecast::Forecast, temperature::Temperature};

#[actix_rt::test]
async fn temp_call_get_weather_test() {
  let openweather_mock = openweatherapi_test::setup("Moscow".into());
  let weatherbit_mock = weatherbitapi_test::setup("Moscow".into(), 5);
  let result = temp_call::get_weather("Moscow".into(), 4).await;
  openweather_mock.assert();
  weatherbit_mock.assert();
  let temp = result.unwrap();
  assert_eq!(temp, Temperature(-4.99));
}

#[actix_rt::test]
async fn temp_call_get_forecast() {
  let openweather_mock = openweatherapi_test::setup("Moscow".into());
  let weatherbit_mock = weatherbitapi_test::setup("Moscow".into(), 5);
  let result = temp_call::get_forecast("Moscow".into()).await;
  openweather_mock.assert();
  weatherbit_mock.assert();
  let forecast = result.unwrap();
  let forecast_match = Forecast::new(1.43, -0.39, -2.08, -3.8, -4.99);
  assert_eq!(forecast, forecast_match);
}
