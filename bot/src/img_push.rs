use std::collections::HashMap;

use reqwest;
use serde::Deserialize;

pub struct ImgPush {
    url: String,
}

#[derive(Deserialize)]
struct ImgPushResponse {
    filename: String,
}

impl ImgPush {
    pub fn new(url: String) -> Self {
        ImgPush { url }
    }

    pub async fn upload(&self, url: &str) -> Result<String, String> {
        let body = HashMap::from([("url", url)]);

        let client = reqwest::Client::new();
        let resp = client.post(&self.url).json(&body).send().await.unwrap();

        if let Err(err) = resp.error_for_status_ref() {
            return Err(err.to_string());
        }

        // todo throw error if status != 200
        // if status != 200 {
        //     return Err(format!("Server error when uploading: {t}"));
        // }

        if let Ok(data) = resp.json::<ImgPushResponse>().await {
            let image_url = format!("{}/{}", self.url, data.filename);
            Ok(image_url)
        } else {
            Err("Error uploading image".to_string())
        }
    }
}
