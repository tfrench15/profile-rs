
extern crate reqwest;

use std::error::Error;
use reqwest::Client;

const BASE_URL: &'static str = "https://profiles.segment.com";

pub struct Client {
    pub secret: String,
    pub space_id: String,
    client: Client,
}

impl Client {
    pub fn new(secret: &str, space_id: &str) -> Self {
        Client {
            secret: secret.to_owned(),
            space_id: space_id.to_owned(),
            client: Client::new(),
        }
    }

    pub fn get_traits(&self, id_type: &str, id_value: &str) -> Result<String, Error> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/traits", BASE_URL, self.space_id, id_type, id_value);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()?
            .json()?
    }

    pub fn get_events(&self, id_type: &str, id_value: &str) -> Result<String, Error> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/events", BASE_URL, self.space_id, id_type, id_value);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()?
            .json()?
    }

    pub fn get_metadata(&self, id_type: &str, id_value: &str) -> Result<String, Error> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/metadata", BASE_URL, self.space_id, id_type, id_value);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()?
            .json()?
    }

    pub fn get_external_ids(&self, id_type: &str, id_value: &str) -> Result<String, Error> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/external_ids", BASE_URL, self.space_id, id_type, id_value);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()?
            .json()?
    }

    pub fn get_links(&self, id_type: &str, id_value: &str) -> Result<String, Error> {
        let url = format!("{}/v1/spaces/{}/collections/users/profiles/{}:{}/links", BASE_URL, self.space_id, id_type, id_value);

        self.client.get(url)
            .basic_auth(self.secret, None)
            .send()?
            .json()?
    }
}

