use super::{FaviconProvider, Result, FaviconProviderError};
use crate::config::Config;
use axum::async_trait;
use axum::body::Bytes;
use std::sync::Arc;

pub struct FaviconProviderImpl {
    config: Arc<Config>,
    favicon_router_path: &'static str,
}

impl FaviconProviderImpl {
    pub fn new(config: Arc<Config>, favicon_router_path: &'static str) -> Self {
        Self {
            config,
            favicon_router_path,
        }
    }

    fn get_favicon_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.config.data_path).join("favicons")
    }

    fn get_favicon_file_path(&self, feed_id: &str) -> std::path::PathBuf {
        self.get_favicon_dir().join(format!("{}.png", feed_id))
    }

    fn extract_base_url(&self, feed_link: &str) -> String {
        if let Ok(url) = reqwest::Url::parse(feed_link) {
            let host = url.host_str().unwrap_or("");
            let scheme = url.scheme();
            
            let known_feed_prefixes = ["feeds.", "feed.", "rss.", "atom."];
            
            for prefix in known_feed_prefixes.iter() {
                if host.contains(prefix) {
                    let parts: Vec<&str> = host.split('.').collect();
                    if parts.len() > 2 {
                        let tld = parts.last().unwrap_or(&"");
                        let domain = parts.get(parts.len() - 2).unwrap_or(&"");
                        
                        return format!("{}://www.{}.{}/", scheme, domain, tld);
                    }
                }
            }
            
            return format!("{}://{}/", scheme, host);
        }
        feed_link.trim_end_matches('/').to_string()
    }

    fn get_favicon_url(&self, feed_link: &str) -> String {
        format!("{}/favicon.ico", feed_link.trim_end_matches('/'))
    }

    async fn try_download_png_favicon(&self, feed_link: &str) -> Option<Bytes> {
        let url = format!("{}/apple-touch-icon.png", feed_link.trim_end_matches('/'));
        match reqwest::get(&url).await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.bytes().await {
                        Ok(bytes) => {
                            if bytes.len() > 0 {
                                return Some(bytes);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }
        
        let url = format!("{}/favicon.png", feed_link.trim_end_matches('/'));
        match reqwest::get(&url).await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.bytes().await {
                        Ok(bytes) => {
                            if bytes.len() > 0 {
                                return Some(bytes);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }
        
        None
    }
}

#[async_trait]
impl FaviconProvider for FaviconProviderImpl {
    async fn download_favicon(&self, feed_link: &str, feed_id: &str) -> Result<Option<String>> {
        let base_url = self.extract_base_url(feed_link);
        
        let favicon_dir = self.get_favicon_dir();
        tokio::fs::create_dir_all(&favicon_dir)
            .await
            .map_err(|e| FaviconProviderError::IoError(e.to_string()))?;

        let favicon_path = self.get_favicon_file_path(feed_id);

        let ico_url = self.get_favicon_url(&base_url);
        
        match reqwest::get(&ico_url).await {
            Ok(response) => {
                if response.status().is_success() {
                    let bytes = response.bytes().await.map_err(|e| FaviconProviderError::RequestError(e.to_string()))?;
                    if bytes.len() > 0 {
                        tokio::fs::write(&favicon_path, &bytes)
                            .await
                            .map_err(|e| FaviconProviderError::IoError(e.to_string()))?;

                        let relative_path = format!("{}/{}.png", self.favicon_router_path, feed_id);
                        return Ok(Some(relative_path));
                    }
                }
            }
            Err(_) => {}
        }

        if let Some(bytes) = self.try_download_png_favicon(&base_url).await {
            tokio::fs::write(&favicon_path, &bytes)
                .await
                .map_err(|e| FaviconProviderError::IoError(e.to_string()))?;
            
            let relative_path = format!("{}/{}.png", self.favicon_router_path, feed_id);
            return Ok(Some(relative_path));
        }

        Ok(None)
    }
}
