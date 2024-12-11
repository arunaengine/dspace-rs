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
            headers.insert("X-Api-Key", HeaderValue::from_str(&api_key.key).unwrap());
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
        let hostname = "localhost";
        let default_port = 8282;
        let default_path = "/api/v1/dsp";
        let base_path = format!("http://{}:{}{}", hostname, default_port, default_path);

        Configuration {
            base_path,
            user_agent: Some("okhttp/4.12.0".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: Some(BasicAuth::default()),
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: Some(ApiKey {
                prefix: Some("X-Api-Key".to_string()),
                key: "123456".to_owned(),
            }),
        }
    }
}
