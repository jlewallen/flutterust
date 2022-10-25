use std::{error, fmt, io};

/// A useless Error just for the Demo
#[derive(Clone, Debug)]
pub struct ScrapError {
    message: String,
}

impl fmt::Display for ScrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scrape error: {}", self.message)
    }
}

impl error::Error for ScrapError {}

impl From<reqwest::Error> for ScrapError {
    fn from(e: reqwest::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl From<io::Error> for ScrapError {
    fn from(e: io::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

/// Load a page and return its HTML body as a `String`
pub async fn load_page(url: &str) -> Result<String, ScrapError> {
    Ok(reqwest::get(url).await?.text().await?)
}
