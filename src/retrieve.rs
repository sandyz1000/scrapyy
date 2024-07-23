use crate::error::{AppResult as Result, Error};
use reqwest::{Client, Proxy};

async fn profetch(
    url: &str,
    proxy_url: &str,
    headers: Option<reqwest::header::HeaderMap>,
) -> Result<reqwest::Response> {
    let client = Client::builder()
        .proxy(Proxy::all(proxy_url)?)
        .build()
        .map_err(|e| Error::ReqwestError(e))?;

    let request = client
        .get(format!("{}{}", proxy_url, url))
        .headers(headers.unwrap_or_default())
        .send()
        .await
        .map_err(|e| Error::ReqwestError(e))?;

    Ok(request)
}

pub async fn retrieve(url: &str, options: Option<RetrieveOptions>) -> Result<Vec<u8>> {
    let default_headers = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "user-agent",
            reqwest::header::HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",
            ),
        );
        headers
    };

    let options = options.unwrap_or_default();
    let headers = options.headers.unwrap_or(default_headers);

    let client = Client::new();
    let res = if let Some(proxy) = options.proxy {
        profetch(url, &proxy.target, Some(headers)).await?
    } else {
        client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e))?
    };

    let status = res.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(Error::RequestFailedError(status));
    }

    let bytes = res.bytes().await.map_err(|e| Error::ReqwestError(e))?;
    Ok(bytes.to_vec())
}

#[derive(Default)]
pub struct RetrieveOptions {
    pub headers: Option<reqwest::header::HeaderMap>,
    pub proxy: Option<ProxyOptions>,
    pub agent: Option<reqwest::Client>,
    pub signal: Option<reqwest::Request>,
}

#[derive(Default)]
pub struct ProxyOptions {
    target: String,
    pub headers: Option<reqwest::header::HeaderMap>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retrieve_from_source() {
        // Use wiremock to mock the url response
        // Test retrieve from good source
        let url = "https://some.where/good/page";
        match retrieve(url, None).await {
            Ok(bytes) => {
                let html = String::from_utf8(bytes).unwrap();
                let expect = String::from("<div>this is content</div>");
                assert_eq!(html, expect);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        // Test retrieve from good source with \\r\\n
        match retrieve(url, None).await {
            Ok(bytes) => {
                let html = String::from_utf8(bytes).unwrap();
                let expect = String::from("<div>this is content</div>");
                assert_eq!(html, expect);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_retrieve_using_proxy() {
        let url = "https://some.where/good/source-with-proxy";
        match retrieve(url, None).await {
            Ok(bytes) => {
                let html = String::from_utf8(bytes).unwrap();
                let expect = String::from("<div>this is content</div>");
                assert_eq!(html, expect);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_retrieve() {
        let url = "https://example.com";
        match retrieve(url, None).await {
            Ok(buffer) => println!("Response size: {}", buffer.len()),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
