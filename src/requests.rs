use super::temp_call;
use actix_web::{web::Query, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use temp_call::RequestError;

#[derive(Deserialize, Debug)]
pub struct QueryDayData {
  city: String,
  #[serde(default)]
  day: u8,
}

#[derive(Deserialize, Debug)]
pub struct QueryForecastData {
  city: String,
}

pub async fn day(query: Query<QueryDayData>, _req: HttpRequest) -> impl Responder {
  if query.day > 5 {
    let error = RequestError::new("day can't be more than 5".into());
    HttpResponse::BadRequest().json(error)
  } else {
    match temp_call::get_weather(query.city.clone(), query.day).await {
      Ok(temp) => HttpResponse::Ok().json(temp),
      Err(error) => HttpResponse::BadRequest().json(error),
    }
  }
}

pub async fn forecast(query: Query<QueryForecastData>) -> impl Responder {
  match temp_call::get_forecast(query.city.clone()).await {
    Ok(forecast) => HttpResponse::Ok().json(forecast),
    Err(error) => HttpResponse::BadRequest().json(error),
  }
}
