use chrono::{DateTime, Utc};

pub struct Moisture {
    pub time: DateTime<Utc>,
    pub moisture: f64
}
