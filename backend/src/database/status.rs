use chrono::{DateTime, Utc};

pub struct Status {
    pub time: DateTime<Utc>,
    pub is_open: bool
}
