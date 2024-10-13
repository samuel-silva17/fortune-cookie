use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
struct Fortune {
    message: String,
}

#[derive(Deserialize)]
struct AdviceSlip {
    slip: Advice,
}

#[derive(Deserialize)]
struct Advice {
    id: u32,
    advice: String,
}

#[get("/fortune")]
async fn get_fortune() -> impl Responder {
    // Fetch advice from the external API
    let res = reqwest::get("https://api.adviceslip.com/advice").await;

    match res {
        Ok(response) => {
            if let Ok(json) = response.json::<AdviceSlip>().await {
                let message = json.slip.advice;
                HttpResponse::Ok().json(Fortune { message })
            } else {
                HttpResponse::InternalServerError().body("Failed to parse advice.")
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Failed to fetch advice.")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_fortune))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

