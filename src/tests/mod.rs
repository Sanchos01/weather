mod openweatherapi_test;
mod temp_call_test;
mod weatherbitapi_test;

use crate::requests;
use actix_web::test::{self, TestRequest};
use actix_web::{body::Body, web, App};
use serde_json::json;

#[actix_rt::test]
async fn test_day_ok() {
  let openweather_mock = openweatherapi_test::setup("Moscow".into());
  let weatherbit_mock = weatherbitapi_test::setup("Moscow".into(), 2);
  let mut app = test::init_service(
    App::new().service(web::resource("/day").route(web::get().to(requests::day))),
  )
  .await;
  let req = TestRequest::with_uri("/day?city=Moscow&day=1")
    .header("content-type", "x-www-urlencoded")
    .to_request();
  let mut resp = test::call_service(&mut app, req).await;
  openweather_mock.assert();
  weatherbit_mock.assert();
  let body = resp.take_body();
  let body = body.as_ref().unwrap();
  assert!(resp.status().is_success());
  assert_eq!(body, &Body::from("-0.39"));
}

#[actix_rt::test]
async fn test_day_error() {
  let mut app = test::init_service(
    App::new().service(web::resource("/day").route(web::get().to(requests::day))),
  )
  .await;
  let req = TestRequest::with_uri("/day?city=Moscow&day=7")
    .header("content-type", "x-www-urlencoded")
    .to_request();
  let mut resp = test::call_service(&mut app, req).await;
  let body = resp.take_body();
  let body = body.as_ref().unwrap();
  assert!(resp.status().is_client_error());
  let json = json!({"details":"day can't be more than 5"});
  assert_eq!(body, &Body::from(json));
}

#[actix_rt::test]
async fn test_forecast_ok() {
  let openweather_mock = openweatherapi_test::setup("Moscow".into());
  let weatherbit_mock = weatherbitapi_test::setup("Moscow".into(), 5);
  let mut app = test::init_service(
    App::new().service(web::resource("/forecast").route(web::get().to(requests::forecast))),
  )
  .await;
  let req = TestRequest::with_uri("/forecast?city=Moscow")
    .header("content-type", "x-www-urlencoded")
    .to_request();
  let mut resp = test::call_service(&mut app, req).await;
  openweather_mock.assert();
  weatherbit_mock.assert();
  let body = resp.take_body();
  let body = body.as_ref().unwrap();
  assert!(resp.status().is_success());
  assert_eq!(body, &Body::from(json!([1.43, -0.39, -2.08, -3.8, -4.99])));
}
