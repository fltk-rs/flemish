use std::fs;

#[derive(Debug)]
pub struct Model {
    pub save: bool,
    pub text: String,
    pub path: String,
}

impl Model {
    pub fn default() -> Self {
        Self {
            save: false,
            text: String::new(),
            path: String::new(),
        }
    }
    pub fn open(&mut self) {
        self.text = fs::read_to_string(&self.path).unwrap();
    }
    pub fn save(&mut self) {
        fs::write(&self.path, self.text.as_bytes()).unwrap();
        self.save = true;
    }
}
