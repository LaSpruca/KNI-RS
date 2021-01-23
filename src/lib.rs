#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt;

    #[tokio::test]
    async fn it_works() {

    }
}


use hyper::{body::HttpBody as _, Client, Request, Body};
use hyper_tls::HttpsConnector;
use tokio::io::{self, AsyncWriteExt as _};
use hyper::header::{USER_AGENT, CONTENT_TYPE};
use chrono::prelude::*;

struct Portal {
    url: String,
    auth_key: String
}

impl Protal {
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

    pub async fn send_request(&self, date: &std::time::SystemTime) -> Result<String, Box<dyn std::error::Error>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let request = Request::builder()
            .method("POST")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "KAMAR/ CFNetwork/ Darwin/")
            .uri(self.url)
            .body(Body::from("Command=GetNotices&Key=vtku&Date=11/03/2020"))
            .unwrap();

        let mut res = client.request(request).await?;

        let mut data: Vec<u8> = vec!();

        while let Some(chunk) = res.body_mut().data().await {
            let mut b: Vec<u8> = chunk?.as_ref().iter().cloned().collect();
            data.append(&mut b);
        }

        Ok(Result)
    }
}