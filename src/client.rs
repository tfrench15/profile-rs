
extern crate reqwest;
extern crate serde_json;

const BASE_URL: &'static str = "https://profiles.segment.com";

pub struct Client {
    pub client: reqwest::Client,
    secret: String,
    space_id: String,
}

impl Client {
    pub fn new(secret: &str, space_id: &str) -> Self {
        Client {
            client: reqwest::Client::new(),
            secret: secret.to_owned(),
            space_id: space_id.to_owned(),
        }
    }

    pub async fn get_traits(&self, key: &str, val: &str) -> reqwest::Result<serde_json::Value> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/traits", BASE_URL, self.space_id, key, val);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?
    }

    pub async fn get_events(&self, key: &str, val: &str) -> reqwest::Result<serde_json::Value> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/events", BASE_URL, self.space_id, key, val);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?
    }

    pub async fn get_metadata(&self, key: &str, val: &str) -> reqwest::Result<serde_json::Value> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/metadata", BASE_URL, self.space_id, key, val);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?
    }

    pub async fn get_external_ids(&self, key: &str, val: &str) -> reqwest::Result<serde_json::Value> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/external_ids", BASE_URL, self.space_id, key, val);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?
    }

    pub async fn get_links(&self, key: &str, val: &str) -> reqwest::Result<serde_json::Value> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/links", BASE_URL, self.space_id, key, val);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?
    }
}

