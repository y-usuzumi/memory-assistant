use chrono::NaiveDateTime;
use derive_getters::Getters;

#[derive(Getters)]
pub struct AppContext {
    datetime: NaiveDateTime,
}

impl AppContext {
    #[cfg(test)]
    pub fn new_random() -> Self {
        use chrono::Utc;
        let datetime = Utc::now().naive_utc();
        Self { datetime }
    }
}
