use std::time::Instant;

use derive_getters::Getters;

#[derive(Getters)]
pub struct AppContext {
    time: Instant,
}

impl AppContext {
    #[cfg(test)]
    pub fn new_random() -> Self {
        use std::time::Duration;

        let time = Instant::now() + Duration::from_secs(100);
        Self { time }
    }
}
