use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use serde::Serialize;
use log::{info, error};
use flexi_logger::{Logger, FileSpec, Duplicate};
use std::io;
use std::env;

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
    // Read the API URL from the environment variable or use default
    let api_url = env::var("API_URL").unwrap_or_else(|_| "https://api.adviceslip.com/advice".to_string());
    info!("Fetching fortune from: {}", api_url);

    // Fetch advice from the external API
    let res = reqwest::get(&api_url).await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(json) = response.json::<AdviceSlip>().await {
                    let message = json.slip.advice;
                    info!("Successfully fetched a fortune.");
                    HttpResponse::Ok().json(Fortune { message })
                } else {
                    error!("Failed to parse advice JSON.");
                    HttpResponse::InternalServerError().body("Failed to parse advice.")
                }
            } else {
                error!("External API returned an error: {}", response.status());
                HttpResponse::InternalServerError().body("External API returned an error.")
            }
        }
        Err(e) => {
            error!("Error fetching advice: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch advice.")
        }
    }
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize the logger
    Logger::try_with_str("info")
        .map_err(|e| {
            eprintln!("Logger initialization failed: {}", e);
            io::Error::new(io::ErrorKind::Other, e)
        })?
        .log_to_file(FileSpec::default().directory("/var/log/fortune_backend"))
        .duplicate_to_stderr(Duplicate::Info) // Also log to stderr
        .start()
        .map_err(|e| {
            eprintln!("Logger start failed: {}", e);
            io::Error::new(io::ErrorKind::Other, e)
        })?;

    info!("Starting the server...");

    HttpServer::new(|| App::new().service(get_fortune))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

