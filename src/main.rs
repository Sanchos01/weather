extern crate log;
#[macro_use]
extern crate lazy_static;

use actix_web::{middleware, web, App, HttpServer};
use std::time::Duration;

pub mod requests;
pub mod temp_call;
#[cfg(test)]
pub mod tests;

lazy_static! {
  pub static ref OPENWEATHER_APIKEY: String =
    std::env::var("OPENWEATHER_APIKEY").expect("OPENWEATHER_APIKEY not present");
  pub static ref WEATHERBIT_APIKEY: String =
    std::env::var("WEATHERBIT_APIKEY").expect("WEATHERBIT_APIKEY not present");
  pub static ref HTTP_CLIENT: reqwest::Client = http_client();
}

fn http_client() -> reqwest::Client {
  reqwest::ClientBuilder::new()
    .pool_max_idle_per_host(20)
    .connect_timeout(Duration::from_secs(5))
    .timeout(Duration::from_secs(5))
    .build()
    .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // init + check_emptiness of env. vars
  if (*OPENWEATHER_APIKEY).chars().count() == 0 {
    panic!("OPENWEATHER_APIKEY is empty")
  }
  if (*WEATHERBIT_APIKEY).chars().count() == 0 {
    panic!("WEATHERBIT_APIKEY is empty")
  }
  std::env::set_var("RUST_LOG", "info");
  env_logger::init();

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .service(web::resource("/day").route(web::get().to(requests::day)))
      .service(web::resource("/forecast").route(web::get().to(requests::forecast)))
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
