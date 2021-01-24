#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt;

    #[tokio::test]
    async fn it_works() {
        let protal = Portal::new("https://demo.school.kiwi/api/api.php");
    }
}


use hyper::{body::HttpBody as _, Client, Request, Body};
use hyper_tls::HttpsConnector;
use tokio::io::{self, AsyncWriteExt as _};
use hyper::header::{USER_AGENT, CONTENT_TYPE};
use chrono::prelude::*;


/// Struct used to access the Kamar API.
struct Portal {
    url: String,
    auth_key: String
}

impl Portal {
    pub fn new(url: &str) -> Self {
        Portal {
            url: url.into(),
            auth_key: "vtku".into()
        }
    }
    
    pub fn with_key(url: &str, key: &str) -> Self {
        Portal {
            url: url.into(),
            auth_key: key.into()
        }
    }

    pub async fn get_notices_today(&self) -> Result<String, Box<dyn std::error::Error>> {
        let now = chrono::Utc::now();
        self.get_notices(&now).await?
    }

    pub async fn get_notices(&self, date: &chrono::DateTime<Utc>) -> Result<String, Box<dyn std::error::Error>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let formatted = date.format("dd/MM/yyyy").to_string();

        let request = Request::builder()
            .method("POST")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "KAMAR/ CFNetwork/ Darwin/")
            .uri(self.url)
            .body(Body::from(format!("Command=GetNotices&Key=vtku&Date={}", formatted)))
            .unwrap();

        let mut res = client.request(request).await?;

        let mut data: Vec<u8> = vec!();

        while let Some(chunk) = res.body_mut().data().await {
            let mut b: Vec<u8> = chunk?.as_ref().iter().cloned().collect();
            data.append(&mut b);
        }

        Ok(String::from_utf8(data)?)
    }
}
