use base64::{engine::general_purpose, Engine};

#[derive(Debug)]
pub struct Model {
    pub decode: String,
    pub encode: String,
    pub font: i32,
    pub size: i32,
}

impl Model {
    pub fn default() -> Self {
        Self {
            decode: String::new(),
            encode: String::new(),
            font: 0,
            size: 14,
        }
    }
    pub fn encode(&mut self) {
        self.encode = general_purpose::STANDARD.encode(&self.decode);
    }
    pub fn decode(&mut self) {
        self.decode = match general_purpose::STANDARD.decode(&self.encode) {
            Ok(decode) => String::from_utf8(decode).unwrap(),
            Err(error) => format!("{}", error),
        }
    }
}
