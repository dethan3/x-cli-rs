use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use xcli_core::{Result, XCliError};

#[derive(Debug, Clone)]
pub struct WebBridgeClient {
    base_url: String,
    http: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStatus {
    pub daemon_running: bool,
    pub extension_connected: bool,
}

#[async_trait]
pub trait BrowserBridge: Send + Sync {
    async fn status(&self) -> Result<BridgeStatus>;
    async fn navigate(&self, url: &str) -> Result<()>;
    async fn eval<T>(&self, javascript: &str) -> Result<T>
    where
        T: DeserializeOwned + Send;
}

impl WebBridgeClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::new(),
        }
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }
}

#[async_trait]
impl BrowserBridge for WebBridgeClient {
    async fn status(&self) -> Result<BridgeStatus> {
        self.http
            .get(self.endpoint("status"))
            .send()
            .await
            .map_err(|_| XCliError::DaemonUnreachable(self.base_url.clone()))?
            .error_for_status()
            .map_err(|_| XCliError::DaemonNotRunning)?
            .json::<BridgeStatus>()
            .await
            .map_err(|err| XCliError::BrowserActionFailed(err.to_string()))
    }

    async fn navigate(&self, url: &str) -> Result<()> {
        let payload = serde_json::json!({ "url": url });
        self.http
            .post(self.endpoint("navigate"))
            .json(&payload)
            .send()
            .await
            .map_err(|_| XCliError::DaemonUnreachable(self.base_url.clone()))?
            .error_for_status()
            .map_err(|err| XCliError::BrowserActionFailed(err.to_string()))?;
        Ok(())
    }

    async fn eval<T>(&self, javascript: &str) -> Result<T>
    where
        T: DeserializeOwned + Send,
    {
        let payload = serde_json::json!({ "javascript": javascript });
        self.http
            .post(self.endpoint("eval"))
            .json(&payload)
            .send()
            .await
            .map_err(|_| XCliError::DaemonUnreachable(self.base_url.clone()))?
            .error_for_status()
            .map_err(|err| XCliError::BrowserActionFailed(err.to_string()))?
            .json::<T>()
            .await
            .map_err(|err| XCliError::BrowserActionFailed(err.to_string()))
    }
}
