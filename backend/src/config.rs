use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub webcam_url: String,
    pub refresh_interval_secs: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            webcam_url: env::var("WEBCAM_URL").unwrap_or_else(|_| {
                "https://panodata.panomax.com/cams/879/recent_full.jpg".to_string()
            }),
            refresh_interval_secs: env::var("REFRESH_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300),
        }
    }
}

