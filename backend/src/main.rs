use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use rand::seq::SliceRandom;
use serde::Serialize;

#[derive(Serialize)]
struct Fortune {
    message: String,
}

#[get("/fortune")]
async fn get_fortune() -> impl Responder {
    let fortunes = vec![
        "You will have a great day!",
        "Now is the time to try something new.",
        "A pleasant surprise is waiting for you.",
        "Believe in yourself and others will too.",
        "You are on the right path.",
    ];

    let mut rng = rand::thread_rng();
    let message = fortunes.choose(&mut rng).unwrap().to_string();

    HttpResponse::Ok().json(Fortune { message })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .service(get_fortune)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
