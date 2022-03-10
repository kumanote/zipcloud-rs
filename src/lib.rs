mod error;

use http::{Request, StatusCode, Version};
use hyper::body::Buf;
use hyper_rustls::HttpsConnectorBuilder;
use serde::Deserialize;
use std::io::Read;

pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;

const ENDPOINT: &str = "https://zipcloud.ibsnet.co.jp/api/search";

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub message: Option<String>,
    pub results: Option<Vec<Address>>,
    pub status: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Address {
    pub address1: String,
    pub address2: String,
    pub address3: String,
    pub kana1: String,
    pub kana2: String,
    pub kana3: String,
    pub prefcode: String,
    pub zipcode: String,
}

pub async fn fetch_address(zipcode: &str) -> Result<Option<Address>> {
    let uri = format!("{}?zipcode={}&limit=1", ENDPOINT, zipcode);
    let request = Request::builder()
        .version(Version::HTTP_11)
        .method("GET")
        .uri(uri)
        .body(hyper::Body::empty())
        .unwrap();
    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    let client = hyper::Client::builder().http2_only(false).build(connector);
    let response = client.request(request).await?;
    let response_status = response.status().clone();
    let mut response_body = String::new();
    hyper::body::aggregate(response.into_body())
        .await?
        .reader()
        .read_to_string(&mut response_body)?;
    if response_status == StatusCode::OK {
        let response: Response = serde_json::from_str(&response_body)?;
        if let Some(result) = response.results {
            let address = result.get(0).unwrap();
            Ok(Some(address.clone()))
        } else {
            Ok(None)
        }
    } else {
        Err(Error::Gateway {
            status_code: response_status.as_u16(),
            reason: response_body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ok() {
        let zipcode = "100-0000";
        let address = fetch_address(zipcode).await.unwrap();
        assert!(address.is_some());
        assert_eq!(
            Address {
                address1: "東京都".to_owned(),
                address2: "千代田区".to_owned(),
                address3: "".to_owned(),
                kana1: "ﾄｳｷｮｳﾄ".to_owned(),
                kana2: "ﾁﾖﾀ\u{ff9e}ｸ".to_owned(),
                kana3: "".to_owned(),
                prefcode: "13".to_owned(),
                zipcode: "1000000".to_owned()
            },
            address.unwrap()
        );
    }

    #[tokio::test]
    async fn test_ng() {
        let zipcode = "999-9999";
        let address = fetch_address(zipcode).await.unwrap();
        assert!(address.is_none());
    }
}
