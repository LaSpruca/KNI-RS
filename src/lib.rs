extern crate serde;

#[cfg(test)]
mod tests;
/// Structures used to represent the data from the Kamar API
pub mod response;

use hyper::{body::HttpBody as _, Client, Request, Body};
use hyper_tls::HttpsConnector;
use chrono::prelude::*;
use crate::response::{NoticesResults, NoticesError};
use serde_xml_rs::from_reader;


/// Struct used to access the Kamar API.
pub struct Portal {
    url: String,
    auth_key: String
}

impl Portal {
    /// Creates a new `Portal` struct with the given url and the key vtku
    /// # Params
    /// - `url`: The full url of the portal api, i.e. `https://demo.school.kiwi/api/api.php`
    pub fn new(url: &str) -> Self {
        Portal {
            url: url.into(),
            auth_key: "vtku".into()
        }
    }

    /// Creates a new `Portal` struct with the given url and key
    /// # Params
    /// - `url`: The full url of the portal api, i.e. `https://demo.school.kiwi/api/api.php`
    /// - `key`: The authentication key from the API, if the default does not work
    pub fn with_key(url: &str, key: &str) -> Self {
        Portal {
            url: url.into(),
            auth_key: key.into()
        }
    }

    /// Gets the notices for today from the specified portal
    pub async fn get_notices_today(&self) -> Result<NoticesResults, NoticesError> {
        let now = chrono::Utc::now();
        self.get_notices(&now).await
    }

    /// Gets the notices for the specified date from the specified portal
    /// # Params
    /// - `date`: The date that you would like to get the notices for
    pub async fn get_notices(&self, date: &chrono::DateTime<Utc>) -> Result<NoticesResults, NoticesError> {
        // Create a new HTTPS hyper client
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        // Format the date
        let formatted = date.format("%d/%m/%Y").to_string();

        // Create kamar request
        let request = Request::builder()
            .method("POST")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "KAMAR/ CFNetwork/ Darwin/")
            .uri(self.url.clone())
            .body(Body::from(format!("Command=GetNotices&Key={}&Date={}", self.auth_key, formatted)))
            .unwrap();

        // Get the response
        let mut res = client.request(request).await.unwrap();

        // Read the data
        let mut data: Vec<u8> = vec!();

        while let Some(chunk) = res.body_mut().data().await {
            let mut b: Vec<u8> = chunk.unwrap().as_ref().iter().cloned().collect();
            data.append(&mut b);
        }

        // Pass and return the response
        let passed: Result<NoticesResults, serde_xml_rs::Error> = from_reader(data.as_slice());

        return match passed {
            Ok(result) => { Ok(result) },
            Err(_) => {
                if let Ok(result) = from_reader::<&[u8], NoticesError>(data.as_slice()) {
                    return Err(result);
                }
                Ok(from_reader(data.as_slice()).unwrap())
            }
        }
    }
}

/// Turns date string into `DateTime<Utc>`
/// # Params
/// - `date`: The date you want to pass, expected format: %Y-%m-%d
pub fn parse_date(date: &str) -> DateTime<Utc> {
    let adjusted_date = format!("{}T12:00:00", date);
    let naive = NaiveDateTime::parse_from_str(adjusted_date.as_str(), "%Y-%m-%dT%H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();

    date_time.with_timezone(&Utc)
}
