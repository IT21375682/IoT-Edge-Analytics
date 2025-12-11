use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;  // For sending data to Django or Celery   

#[derive(Deserialize, Serialize, Debug)]
struct TelemetryData {
    temperature: f64,
    humidity: f64,
}

async fn process_telemetry(data: web::Json<TelemetryData>) -> impl Responder {
    // Process telemetry data here (async work can be done using Tokio)
    println!("Received telemetry data: {:?}", data);

    if data.temperature > 50.0 {
        println!("Alert! High temperature recorded: {}", data.temperature);
    }

    // After processing, send the data to Django or directly to Celery (via API call)
    let client = Client::new();
    let _res = client
        .post("http://localhost:8000/api/telemetry/") 
        .json(&data)
        .send()
        .await
        .unwrap();

    HttpResponse::Ok().json(data)  
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/telemetry", web::post().to(process_telemetry))  //API route
    })
    .bind("127.0.0.1:8080")?  // Bind to a port
    .run()
    .await
}