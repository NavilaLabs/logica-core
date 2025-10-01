use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Item {
    id: u64,
    pid: Uuid,
    title: String,
    description: Option<String>,
    content: Vec<u8>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
