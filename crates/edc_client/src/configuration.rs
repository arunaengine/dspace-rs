/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new(
        base_path: String,
        user_agent: Option<String>,
        client: reqwest::Client,
        basic_auth: Option<BasicAuth>,
        oauth_access_token: Option<String>,
        bearer_access_token: Option<String>,
        api_key: Option<ApiKey>,
    ) -> Configuration {
        Configuration {
            base_path,
            user_agent,
            client,
            basic_auth,
            oauth_access_token,
            bearer_access_token,
            api_key,
        }
    }

    pub fn with_headers(mut self) -> Self {
        let mut headers = HeaderMap::new();
        if let Some(api_key) = &self.api_key {
            headers.insert("x-api-key", HeaderValue::from_str(&api_key.key).unwrap());
        }
        self.client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        self
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost:11002/api/management".to_owned(),
            user_agent: Some("okhttp/4.12.0".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: Some(BasicAuth::default()),
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: Some(ApiKey {
                prefix: None,
                key: "ApiKeyDefaultValue".to_owned(),
            }),
        }
    }
}
