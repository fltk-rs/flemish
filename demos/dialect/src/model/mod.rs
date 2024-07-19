use {
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, fs},
};

#[derive(Deserialize)]
struct Lang {
    languages: Vec<HashMap<String, String>>,
}

impl Lang {
    const LINGVA: &'static str = r#"https://lingva.thedaviddelta.com/api/v1/"#;
    fn init() -> Vec<HashMap<String, String>> {
        if let Ok(response) = ureq::get(&format!("{}languages", Self::LINGVA)).call() {
            response.into_json::<Self>().unwrap().languages
        } else {
            Vec::new()
        }
    }
    fn tran(source: String, target: String, query: String) -> String {
        if let Ok(response) =
            ureq::get(&format!("{}/{}/{}/{}", Self::LINGVA, source, target, query)).call()
        {
            response
                .into_json::<serde_json::Value>()
                .unwrap()
                .as_object()
                .unwrap()["translation"]
                .to_string()
        } else {
            String::new()
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Model {
    pub from: i32,
    pub to: i32,
    pub font: i32,
    pub size: i32,
    pub source: String,
    pub target: String,
    pub lang: Vec<HashMap<String, String>>,
}

impl Model {
    pub fn default(file: &str) -> Self {
        let default = Self {
            from: 0,
            to: 0,
            font: 1,
            size: 14,
            source: String::new(),
            target: String::new(),
            lang: Lang::init(),
        };
        if let Ok(value) = fs::read(file) {
            if let Ok(value) = rmp_serde::from_slice(&value) {
                value
            } else {
                default
            }
        } else {
            default
        }
    }
    pub fn click(&self) -> String {
        let from = self.lang[self.from as usize]["code"].clone();
        let to = self.lang[self.to as usize]["code"].clone();
        let source = self.source.clone();
        Lang::tran(from, to, source)
    }
    pub fn save(&mut self, file: &str) {
        fs::write(file, rmp_serde::to_vec(&self).unwrap()).unwrap();
    }
}
