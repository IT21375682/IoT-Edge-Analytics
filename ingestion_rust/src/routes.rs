use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::models::TelemetryData;


pub async fn process_telemetry(data: web::Json<TelemetryData>) -> impl Responder {
    println!("Received telemetry data: {:?}", data);

    if let Err(e) = push_to_redis(data.telemetry_id) {
        eprintln!("Error pushing to Redis: {}", e);
        return HttpResponse::InternalServerError().body("Error pushing to Redis");
    }

    // Process telemetry data (e.g., send to Celery or Django)
    HttpResponse::Ok().json(data)
}
