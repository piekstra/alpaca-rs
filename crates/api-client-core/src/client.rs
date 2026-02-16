use reqwest::header::HeaderMap;
use tracing::{debug, warn};

use crate::error::ApiClientError;

/// Generic async REST client with built-in response handling.
pub struct RestClient {
    http: reqwest::Client,
    base_url: String,
}

/// Builder for constructing a `RestClient`.
pub struct RestClientBuilder {
    base_url: String,
    headers: HeaderMap,
    timeout: std::time::Duration,
}

impl RestClientBuilder {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            headers: HeaderMap::new(),
            timeout: std::time::Duration::from_secs(30),
        }
    }

    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn header(mut self, name: &'static str, value: &str) -> Result<Self, ApiClientError> {
        self.headers.insert(
            name,
            value
                .parse()
                .map_err(|e: reqwest::header::InvalidHeaderValue| {
                    ApiClientError::Config(e.to_string())
                })?,
        );
        Ok(self)
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Result<RestClient, ApiClientError> {
        let http = reqwest::Client::builder()
            .default_headers(self.headers)
            .timeout(self.timeout)
            .build()?;
        Ok(RestClient {
            http,
            base_url: self.base_url,
        })
    }
}

impl RestClient {
    pub fn builder(base_url: impl Into<String>) -> RestClientBuilder {
        RestClientBuilder::new(base_url)
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, ApiClientError> {
        let url = self.url(path);
        debug!("GET {url}");
        let resp = self.http.get(&url).send().await?;
        self.handle_response(resp).await
    }

    pub async fn get_with_query<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, ApiClientError> {
        let url = self.url(path);
        debug!("GET {url}");
        let resp = self.http.get(&url).query(query).send().await?;
        self.handle_response(resp).await
    }

    pub async fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, ApiClientError> {
        let url = self.url(path);
        debug!("POST {url}");
        let resp = self.http.post(&url).json(body).send().await?;
        self.handle_response(resp).await
    }

    pub async fn patch<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, ApiClientError> {
        let url = self.url(path);
        debug!("PATCH {url}");
        let resp = self.http.patch(&url).json(body).send().await?;
        self.handle_response(resp).await
    }

    pub async fn delete(&self, path: &str) -> Result<(), ApiClientError> {
        let url = self.url(path);
        debug!("DELETE {url}");
        let resp = self.http.delete(&url).send().await?;
        let status = resp.status();
        if status.as_u16() == 429 {
            return Err(self.extract_rate_limit(&resp));
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api {
                status: status.as_u16(),
                body,
            });
        }
        Ok(())
    }

    pub async fn delete_parsed<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, ApiClientError> {
        let url = self.url(path);
        debug!("DELETE {url}");
        let resp = self.http.delete(&url).send().await?;
        self.handle_response(resp).await
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, ApiClientError> {
        let status = resp.status();

        if status.as_u16() == 429 {
            let retry_after = resp
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(1);
            warn!("Rate limited, retry after {retry_after}s");
            return Err(ApiClientError::RateLimited {
                retry_after_secs: retry_after,
            });
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api {
                status: status.as_u16(),
                body,
            });
        }

        let body = resp.text().await?;
        let parsed = serde_json::from_str(&body)?;
        Ok(parsed)
    }

    fn extract_rate_limit(&self, resp: &reqwest::Response) -> ApiClientError {
        let retry_after = resp
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(1);
        warn!("Rate limited, retry after {retry_after}s");
        ApiClientError::RateLimited {
            retry_after_secs: retry_after,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_client() {
        let client = RestClient::builder("https://example.com")
            .timeout(std::time::Duration::from_secs(10))
            .build();
        assert!(client.is_ok());
    }

    #[test]
    fn builder_with_headers() {
        let client = RestClient::builder("https://example.com")
            .header("X-Custom", "value")
            .unwrap()
            .build();
        assert!(client.is_ok());
    }

    #[test]
    fn url_concatenation() {
        let client = RestClient::builder("https://api.example.com")
            .build()
            .unwrap();
        assert_eq!(client.url("/v2/foo"), "https://api.example.com/v2/foo");
    }
}
