#[derive(Clone)]
pub struct Model {
    pub method: u8,
    pub url: String,
    pub responce: String,
    pub status: String,
}

impl Model {
    pub fn default() -> Self {
        Self {
            method: 0,
            url: String::from(r#"https://ipinfo.io/json"#),
            responce: String::new(),
            status: String::new(),
        }
    }
    pub fn click(&self) -> (bool, String) {
        let url = match self.url.starts_with("https://") {
            true => self.url.clone(),
            false => String::from("https://") + &self.url,
        };
        if let Ok(response) = match self.method {
            0 => ureq::get(&url),
            1 => ureq::post(&url),
            _ => unreachable!(),
        }
        .call()
        {
            let body = response.into_string().unwrap();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                (true, serde_json::to_string_pretty(&json).unwrap())
            } else {
                (true, body)
            }
        } else {
            (false, String::from("Error"))
        }
    }
}
