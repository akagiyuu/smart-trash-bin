use chrono::{DateTime, Utc};

pub struct TrashLevel {
    pub time: DateTime<Utc>,
    pub level: f64,
}
