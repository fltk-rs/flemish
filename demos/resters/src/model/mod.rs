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
            url: String::new(),
            responce: String::new(),
            status: String::new(),
        }
    }
}
