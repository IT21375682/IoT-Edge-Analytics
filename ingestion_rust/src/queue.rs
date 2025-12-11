use redis::{Commands, Connection};
use serde_json;

pub fn push_to_redis(telemetry_id: u32) -> redis::RedisResult<()> {
    // Open connection to Redis
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    // Push telemetry_id to the Redis list (telemetry_queue)
    con.rpush("telemetry_queue", telemetry_id)
}
