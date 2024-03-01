use std::time::Duration;

use rocket::{tokio::time::sleep, *};

#[get("/")]
fn ping() -> &'static str {
  "hello cargo"
}

#[get("/health")]
fn healthcheck() -> &'static str {
  "health"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
  sleep(Duration::from_secs(seconds)).await;
  format!("Wait for {seconds} seconds")
}

#[launch]
fn rocket() -> Rocket<Build> {
  rocket::build().mount("/", routes![ping, healthcheck, delay])
}

/*
 * TODO: web server <Actix : https://actix.rs/>
 * Require:
 * - Method POST sum 2 number.
 * - Method GET health check.
 */
