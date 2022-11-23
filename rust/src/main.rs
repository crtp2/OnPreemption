use actix_web::{get, web, App, HttpServer, Responder};

async fn next(from: i64, sum: i64) -> (i64, i64) {
    (from - 1, from + sum)
}

#[get("/sum/{val}")]
async fn sum_handler(val: web::Path<String>) -> impl Responder {
    let mut from = val.parse::<i64>().unwrap();
    let mut sum = 0_i64;
    while from != 0 {
        tokio::task::yield_now().await;
        (from, sum) = next(from, sum).await
    }
    format!("{sum}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(sum_handler)
    })
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
