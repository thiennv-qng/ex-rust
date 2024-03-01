use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::ops::Add;

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
  a + b
}

#[get("/sum/{a}/{b}")]
async fn sum(path: web::Path<(u32, u32)>) -> Result<String> {
  let (a, b) = path.into_inner();

  let c = add(a, b);
  Ok(format!("{} {} {} {} {} :", a, "+", b, " = ", c))
}

#[post("/health")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn health_check() -> impl Responder {
  HttpResponse::Ok().body("Everything is ok =)))")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(sum)
      .route("/health", web::get().to(health_check))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}
