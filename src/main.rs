use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_web::post;
use chrono::Utc;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ClientStatus {
    clientid: String,
    event: String,
    reason: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or(String::from("debug"));
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_level));

    HttpServer::new( ||
                     App::new()
                     .wrap(middleware::Logger::default())
                     .route("/", web::get().to(index))
                     .service(webhook)
                     )
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[post("/webhook")]
async fn webhook(payload: web::Json<ClientStatus>) -> HttpResponse {
    println!("{} {}, {}, {}", Utc::now().to_rfc3339(), payload.clientid, payload.event, payload.reason);
    HttpResponse::Ok().finish()
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
