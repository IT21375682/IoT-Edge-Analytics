use actix_web::{web, App, HttpServer};
use actix_web::web::Json;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use reqwest::Client;
use crate::aggregator::Aggregator;
use crate::gstreamer::start_video_stream;

#[derive(Deserialize, Serialize, Debug)]
struct TelemetryData {
    temperature: f64,
    humidity: f64,
}

async fn process_telemetry(data: web::Json<TelemetryData>, aggregator: web::Data<Arc<Mutex<Aggregator>>>) -> HttpResponse {
    // Process telemetry data with aggregation logic
    let mut aggr = aggregator.lock().unwrap();
    aggr.aggregate(&data);

    // Optionally send data to Django or Celery after aggregation
    if aggr.should_send() {
        let aggregated_data = aggr.get_aggregated();
        let client = Client::new();
        let _res = client
            .post("http://localhost:8000/api/telemetry/")  // Replace with Django API endpoint
            .json(&aggregated_data)
            .send()
            .await
            .unwrap();
    }

    HttpResponse::Ok().json(data)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Start GStreamer video stream in a separate thread (optional)
    std::thread::spawn(|| start_video_stream());

    // Set up the Actix web server with data aggregation
    let aggregator = web::Data::new(Arc::new(Mutex::new(Aggregator::new())));

    HttpServer::new(move || {
        App::new()
            .app_data(aggregator.clone())
            .route("/api/telemetry", web::post().to(process_telemetry))  // Define POST endpoint
    })
    .bind("127.0.0.1:8080")?  // Bind server to localhost:8080
    .run()
    .await
}
